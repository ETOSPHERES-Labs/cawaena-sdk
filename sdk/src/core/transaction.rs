use super::Sdk;
use crate::backend::transactions::{
    commit_transaction, create_new_transaction, get_transaction_details, get_transactions_list,
};
use crate::error::Result;
use crate::types::currencies::CryptoAmount;
use crate::types::networks::{Network, NetworkType};
use crate::types::transactions::PurchaseDetails;
use crate::types::{
    newtypes::EncryptionPin,
    transactions::{TxInfo, TxList},
};
use crate::wallet::error::WalletError;
use api_types::api::networks::ApiNetworkType;
use api_types::api::transactions::{ApiApplicationMetadata, ApiTxStatus, PurchaseModel, Reason};
use iota_sdk::types::block::payload::TaggedDataPayload;
use log::{debug, info};

impl Sdk {
    /// Create purchase request
    ///
    /// # Arguments
    ///
    /// * `receiver` - The receiver's username.
    /// * `amount` - The amount of the purchase.
    /// * `product_hash` - The hash of the product.
    /// * `app_data` - The application data.
    /// * `purchase_type` - The type of the purchase.
    ///
    /// # Returns
    ///
    /// The purchase ID. This is an internal index used to reference the transaction in cryptpay
    ///
    /// # Errors
    ///
    /// Returns an error if the user or wallet is not initialized, or if there is an error creating the transaction.
    pub async fn create_purchase_request(
        &self,
        receiver: &str,
        amount: CryptoAmount,
        product_hash: &str,
        app_data: &str,
        purchase_type: &str,
    ) -> Result<String> {
        info!("Creating a new purchase request");
        let Some(active_user) = &self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;
        let sender = &active_user.username;
        let access_token = self
            .access_token
            .as_ref()
            .ok_or(crate::error::Error::MissingAccessToken)?;
        let network = self.network.clone().ok_or(crate::Error::MissingNetwork)?;

        let purchase_model = PurchaseModel::try_from(purchase_type.to_string()).map_err(crate::error::Error::Parse)?;

        let reason = match purchase_model {
            PurchaseModel::CLIK => Reason::LIKE,
            PurchaseModel::CPIC => Reason::PURCHASE,
        };

        let metadata = ApiApplicationMetadata {
            product_hash: product_hash.into(),
            reason: reason.to_string(),
            purchase_model: purchase_model.to_string(),
            app_data: app_data.into(),
        };
        let response =
            create_new_transaction(config, access_token, sender, receiver, network.id, amount, metadata).await?;
        let purchase_id = response.index;
        debug!("Created purchase request with id: {purchase_id}");
        Ok(purchase_id)
    }

    /// Get purchase details
    ///
    /// # Arguments
    ///
    /// * `purchase_id` - The ID of the purchase.
    ///
    /// # Returns
    ///
    /// The purchase details.
    ///
    /// # Errors
    ///
    /// Returns an error if the user or wallet is not initialized, or if there is an error getting the transaction details.
    pub async fn get_purchase_details(&self, purchase_id: &str) -> Result<PurchaseDetails> {
        info!("Getting purchase details with id {purchase_id}");
        let Some(active_user) = &self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        let username = &active_user.username;
        let access_token = self
            .access_token
            .as_ref()
            .ok_or(crate::error::Error::MissingAccessToken)?;

        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;
        let response = get_transaction_details(config, access_token, username, purchase_id).await?;

        let details = PurchaseDetails {
            system_address: response.system_address,
            amount: response.amount,
            status: response.status,
            network: response.network,
        };
        Ok(details)
    }

    /// Confirm purchase request
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN of the user.
    /// * `purchase_id` - The ID of the purchase request.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the purchase request is confirmed successfully.
    ///
    /// # Errors
    ///
    /// Returns an error if the user or wallet is not initialized, if there is an error verifying the PIN,
    /// if there is an error getting the transaction details, or if there is an error committing the transaction.
    pub async fn confirm_purchase_request(&mut self, pin: &EncryptionPin, purchase_id: &str) -> Result<()> {
        info!("Confirming purchase request with id {purchase_id}");
        self.verify_pin(pin).await?;

        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };
        let Some(active_user) = &mut self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        let username = &active_user.username;
        let config = self.config.as_mut().ok_or(crate::Error::MissingConfig)?;
        let access_token = self
            .access_token
            .as_ref()
            .ok_or(crate::error::Error::MissingAccessToken)?;
        let tx_details = get_transaction_details(config, access_token, username, purchase_id).await?;

        debug!("Tx details: {:?}", tx_details);

        if tx_details.status != ApiTxStatus::Valid {
            return Err(WalletError::InvalidTransaction(format!(
                "Transaction is not valid, current status: {}.",
                tx_details.status
            )))?;
        }

        let current_network = self.network.clone().ok_or(crate::Error::MissingNetwork)?;

        // for now we check that the correct network_id is configured, in the future we might just
        // instantiate the correct wallet instead of throwing an error
        let network: Network = tx_details.network.clone().into();
        if network.id != current_network.id {
            return Err(WalletError::InvalidTransaction(format!(
                "Transaction to commit is in network_id {:?}, but {:?} is the currently active current_network_id.",
                network.id, current_network.id
            )))?;
        }

        let wallet = active_user
            .wallet_manager
            .try_get(config, &self.access_token, repo, network, pin)
            .await?;

        let amount = tx_details.amount.try_into()?;
        let tx_id = match tx_details.network.network_type {
            ApiNetworkType::Evm {
                node_url: _,
                chain_id: _,
            } => {
                let tx_id = wallet
                    .send_transaction_eth(purchase_id, &tx_details.system_address, amount)
                    .await?;

                let newly_created_transaction = wallet.get_wallet_tx(&tx_id).await?;
                let user = repo.get(&active_user.username)?;
                let mut wallet_transactions = user.wallet_transactions;
                wallet_transactions.push(newly_created_transaction);
                let _ = repo.set_wallet_transactions(&active_user.username, wallet_transactions);
                tx_id
            }
            ApiNetworkType::Stardust { node_url: _ } => {
                wallet
                    .send_transaction(purchase_id, &tx_details.system_address, amount)
                    .await?
            }
        };

        debug!("Transaction id on network: {tx_id}");

        commit_transaction(config, access_token, username, purchase_id, &tx_id).await?;

        Ok(())
    }

    /// Send amount to receiver address
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN of the user.
    /// * `address` - The receiver's address.
    /// * `amount` - The amount to send.
    /// * `tag` - The transactions tag. Optional.
    /// * `data` - The associated data with the tag. Optional.
    /// * `message` - The transactions message. Optional.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the amount is sent successfully.
    ///
    /// # Errors
    ///
    /// Returns an error if the user or wallet is not initialized, if there is an error verifying the PIN,
    /// or if there is an error sending the amount.
    pub async fn send_amount(
        &mut self,
        pin: &EncryptionPin,
        address: &str,
        amount: CryptoAmount,
        tag: Option<Vec<u8>>,
        data: Option<Vec<u8>>,
        message: Option<String>,
    ) -> Result<()> {
        info!("Sending amount {amount:?} to receiver {address}");
        self.verify_pin(pin).await?;

        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };
        let Some(active_user) = &mut self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        let config = self.config.as_mut().ok_or(crate::Error::MissingConfig)?;
        let network = self.network.clone().ok_or(crate::Error::MissingNetwork)?;

        let wallet = active_user
            .wallet_manager
            .try_get(config, &self.access_token, repo, network.clone(), pin)
            .await?;

        // create the transaction payload which holds a tag and associated data
        let tag: Box<[u8]> = tag.unwrap_or_default().into_boxed_slice();
        let data: Box<[u8]> = data.unwrap_or_default().into_boxed_slice();
        let tagged_data_payload = Some(TaggedDataPayload::new(tag, data).map_err(WalletError::Block)?);

        match network.network_type {
            NetworkType::Evm {
                node_url: _,
                chain_id: _,
            } => {
                let tx_id = wallet
                    .send_amount_eth(address, amount, tagged_data_payload, message)
                    .await?;

                let newly_created_transaction = wallet.get_wallet_tx(&tx_id).await?;
                let user = repo.get(&active_user.username)?;
                let mut wallet_transactions = user.wallet_transactions;
                wallet_transactions.push(newly_created_transaction);
                let _ = repo.set_wallet_transactions(&active_user.username, wallet_transactions);
            }
            NetworkType::Stardust { node_url: _ } => {
                wallet
                    .send_amount(address, amount, tagged_data_payload, message)
                    .await?;
            }
        }

        Ok(())
    }

    /// Get transaction list
    ///
    /// # Arguments
    ///
    /// * `start` - The starting page number.
    /// * `limit` - The maximum number of transactions per page.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `TxList` if successful.
    ///
    /// # Errors
    ///
    /// Returns an error if there is a problem getting the list of transactions.
    pub async fn get_tx_list(&self, start: u32, limit: u32) -> Result<TxList> {
        info!("Getting list of transactions");
        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;

        let user = self.get_user().await?;

        let access_token = self
            .access_token
            .as_ref()
            .ok_or(crate::error::Error::MissingAccessToken)?;
        let txs_list = get_transactions_list(config, access_token, &user.username, start, limit).await?;
        log::debug!("Txs list for user {}: {:?}", user.username, txs_list);

        Ok(TxList {
            txs: txs_list
                .txs
                .into_iter()
                .map(|val| {
                    Ok(TxInfo {
                        date: Some(val.created_at),
                        sender: val.incoming.username,
                        receiver: val.outgoing.username,
                        reference_id: val.index,
                        amount: val.incoming.amount.try_into()?,
                        currency: val.incoming.network.currency, // ?????????????????????
                        application_metadata: val.application_metadata,
                        status: val.status,
                        transaction_hash: val.incoming.transaction_id,
                        course: val.incoming.exchange_rate.try_into()?,
                    })
                })
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::core_testing_utils::handle_error_test_cases;
    use crate::testing_utils::{
        example_api_network, example_get_user, example_network, example_tx_details, example_tx_metadata,
        example_wallet_borrow, set_config, AUTH_PROVIDER, HEADER_X_APP_NAME, HEADER_X_APP_USERNAME, PURCHASE_ID, TOKEN,
        TX_INDEX, USERNAME,
    };
    use crate::types::currencies::Currency;
    use crate::types::transactions::WalletTxInfo;
    use crate::types::users::KycType;
    use crate::{
        core::Sdk,
        user::MockUserRepo,
        wallet_manager::{MockWalletManager, WalletBorrow},
        wallet_user::MockWalletUser,
    };
    use api_types::api::transactions::GetTxsDetailsResponse;
    use api_types::api::transactions::{
        ApiTransaction, ApiTransferDetails, CreateTransactionResponse, GetTransactionDetailsResponse,
    };
    use api_types::api::viviswap::detail::SwapPaymentDetailKey;
    use iota_sdk::wallet::account::types::InclusionState;
    use iota_sdk::{
        types::{
            block::{
                address::{
                    dto::{AddressDto, Ed25519AddressDto},
                    Ed25519Address,
                },
                input::{
                    dto::{InputDto, UtxoInputDto},
                    UtxoInput,
                },
                output::{
                    dto::{BasicOutputDto, OutputDto},
                    unlock_condition::{
                        dto::{AddressUnlockConditionDto, UnlockConditionDto},
                        AddressUnlockCondition,
                    },
                    BasicOutput,
                },
                payload::{
                    dto::{PayloadDto, TaggedDataPayloadDto, TransactionPayloadDto},
                    transaction::{
                        dto::{RegularTransactionEssenceDto, TransactionEssenceDto},
                        RegularTransactionEssence, TransactionId,
                    },
                    TransactionPayload,
                },
                signature::{
                    dto::{Ed25519SignatureDto, SignatureDto},
                    Ed25519Signature,
                },
                unlock::{
                    dto::{SignatureUnlockDto, UnlockDto},
                    SignatureUnlock,
                },
            },
            TryFromDto,
        },
        wallet::account::types::Transaction,
    };
    use mockito::Matcher;
    use rstest::rstest;
    use rust_decimal_macros::dec;

    fn example_wallet_transaction() -> Transaction {
        /* transaction payload for reference
        Transaction { payload: TransactionPayload { essence: Regular(RegularTransactionEssence { network_id: dec!(7784367046153662236), inputs: [UtxoInput(0x36989ba6c4e23fe4c62837551bd1b6e5548ddd0e33c6ae84cf6bc669fbd98ae90000)], inputs_commitment: InputsCommitment(0xb2bb637c62f9a8a46483fcaa532c0a4b63de82d379a10892fdfa82c0dceae5d0), outputs: [BasicOutput { amount: dec!(1000000), native_tokens: NativeTokens([]), unlock_conditions: UnlockConditions([AddressUnlockCondition(Ed25519Address(0x64549c8ac945b5c0acde3e29b13793b17e1303d081de7afb12169f5ee06c456e))]), features: Features([]) }], payload: OptionalPayload(None) }), unlocks: Unlocks([SignatureUnlock(Ed25519Signature { public_key: 0x128a7167d30c6928aedef86ec19c0793cabfeec51c1fedb3d852924922775c43, signature: 0x34f6335cc3d892c0bef52531c75a92c041cc8c8aa44665ba46be17968090ded0c27332f2ff7097fd0b3851c55ae441e7c7be791012301151775fa520346a3804 })]) }, block_id: Some(BlockId(0x95e78bc63f2f1707e761e37533facb939902f6bde49f02e57d3f74347bd83e4d)), inclusion_state: Pending, timestamp: dec!(1705321862422), transaction_id: TransactionId(0x5e30005197a27ac0829b2ea9183b9a98d1f1e386fcc1774050f94ff42604de99), network_id: dec!(7784367046153662236), incoming: false, note: None, inputs: [OutputWithMetadataResponse { metadata: OutputMetadata { block_id: BlockId(0x9e76d69b26748650b54dccbf4ae2b88cea16b3d48dfea0393931be8ed7c5f678), output_id: OutputId(0x36989ba6c4e23fe4c62837551bd1b6e5548ddd0e33c6ae84cf6bc669fbd98ae90000), is_spent: false, milestone_index_spent: None, milestone_timestamp_spent: None, transaction_id_spent: None, milestone_index_booked: dec!(1497442), milestone_timestamp_booked: dec!(1705321430), ledger_index: 1497527 }, output: Basic(BasicOutputDto { kind: dec!(3), amount: "1000000", native_tokens: [], unlock_conditions: [Address(AddressUnlockConditionDto { kind: dec!(0), address: Ed25519(Ed25519AddressDto { kind: dec!(0), pub_key_hash: "0x64549c8ac945b5c0acde3e29b13793b17e1303d081de7afb12169f5ee06c456e" }) })], features: [] }) }] }

         */
        let network_id_u64 = 7784367046153662236u64;
        let network_id = network_id_u64.to_string();
        let pub_key_hash = String::from("0x64549c8ac945b5c0acde3e29b13793b17e1303d081de7afb12169f5ee06c456e");
        let pub_key = String::from("0x128a7167d30c6928aedef86ec19c0793cabfeec51c1fedb3d852924922775c43");
        let signature = String::from("0x34f6335cc3d892c0bef52531c75a92c041cc8c8aa44665ba46be17968090ded0c27332f2ff7097fd0b3851c55ae441e7c7be791012301151775fa520346a3804");
        let transaction_id_input = "0x36989ba6c4e23fe4c62837551bd1b6e5548ddd0e33c6ae84cf6bc669fbd98ae9".to_string();
        let utxo_input_dto = UtxoInputDto {
            kind: UtxoInput::KIND,
            transaction_id: transaction_id_input,
            transaction_output_index: 0u16,
        };
        let input = InputDto::Utxo(utxo_input_dto);
        let inputs = vec![input];
        let inputs_commitment = String::from("0xb2bb637c62f9a8a46483fcaa532c0a4b63de82d379a10892fdfa82c0dceae5d0");

        let ed25519_address_dto = Ed25519AddressDto {
            kind: Ed25519Address::KIND,
            pub_key_hash,
        };

        let address_dto = AddressDto::Ed25519(ed25519_address_dto);

        let address_unlock_conditions_dto = AddressUnlockConditionDto {
            kind: AddressUnlockCondition::KIND,
            address: address_dto,
        };

        let unlock_conditions_dto = UnlockConditionDto::Address(address_unlock_conditions_dto);

        let basic_output_dto = BasicOutputDto {
            kind: BasicOutput::KIND,
            amount: "25000000".to_string(),
            native_tokens: vec![],
            unlock_conditions: vec![unlock_conditions_dto],
            features: vec![],
        };

        let output = OutputDto::Basic(basic_output_dto);

        let ed25519_signature_dto = Ed25519SignatureDto {
            kind: Ed25519Signature::KIND,
            public_key: pub_key,
            signature,
        };

        let signature_dto = SignatureDto::Ed25519(Box::new(ed25519_signature_dto));

        let signature_unlock_dto = SignatureUnlockDto {
            kind: SignatureUnlock::KIND,
            signature: signature_dto,
        };

        let unlock = UnlockDto::Signature(signature_unlock_dto);

        let essence = TransactionEssenceDto::Regular(RegularTransactionEssenceDto {
            kind: RegularTransactionEssence::KIND,
            network_id: network_id.clone(),
            inputs,
            inputs_commitment,
            outputs: vec![output],
            payload: Some(PayloadDto::TaggedData(Box::new(TaggedDataPayloadDto {
                kind: TransactionPayload::KIND,
                tag: "test tag".to_string().into_bytes().into_boxed_slice(),
                data: 968547501u64.to_be_bytes().into(),
            }))),
        });

        let transaction_payload_dto = TransactionPayloadDto {
            kind: TransactionPayload::KIND,
            essence,
            unlocks: vec![unlock],
        };

        let transaction_id: [u8; 32] = [0; 32];

        let transaction_id = TransactionId::new(transaction_id);

        let payload = TransactionPayload::try_from_dto(transaction_payload_dto).unwrap();

        Transaction {
            payload,
            block_id: None,
            inclusion_state: iota_sdk::wallet::account::types::InclusionState::Pending,
            timestamp: 0u128,
            transaction_id,
            network_id: network_id_u64,
            incoming: false,
            note: Some(String::from("test message")),
            inputs: vec![],
        }
    }

    fn examples_wallet_tx_list() -> GetTxsDetailsResponse {
        let main_address = "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r".to_string();
        let aux_address = "atoi1qpnrumvaex24dy0duulp4q07lpa00w20ze6jfd0xly422kdcjxzakzsz5kf".to_string();

        GetTxsDetailsResponse {
            txs: vec![ApiTransaction {
                index: "1127f4ba-a0b8-4ecc-a928-bbebc401ac1a".to_string(),
                status: ApiTxStatus::Completed,
                created_at: "2022-12-09T09:30:33.52Z".to_string(),
                updated_at: "2022-12-09T09:30:33.52Z".to_string(),
                fee_rate: dec!(0.2),
                incoming: ApiTransferDetails {
                    transaction_id: Some(
                        "0x215322f8afdba4e22463a9d8a2e25d96ab0cb9ae6d56ee5ab13065068dae46c0".to_string(),
                    ),
                    block_id: Some("0x215322f8afdba4e22463a9d8a2e25d96ab0cb9ae6d56ee5ab13065068dae46c0".to_string()),
                    username: "satoshi".into(),
                    address: main_address.clone(),
                    amount: dec!(920.89),
                    exchange_rate: dec!(0.06015),
                    network: example_api_network(Currency::Iota),
                },
                outgoing: ApiTransferDetails {
                    transaction_id: Some(
                        "0x215322f8afdba4e22463a9d8a2e25d96ab0cb9ae6d56ee5ab13065068dae46c0".to_string(),
                    ),
                    block_id: Some("0x215322f8afdba4e22463a9d8a2e25d96ab0cb9ae6d56ee5ab13065068dae46c0".to_string()),
                    username: "hulk".into(),
                    address: aux_address.clone(),
                    amount: dec!(920.89),
                    exchange_rate: dec!(0.06015),
                    network: example_api_network(Currency::Iota),
                },
                application_metadata: Some(example_tx_metadata()),
            }],
        }
    }

    #[rstest]
    #[case::success(Ok(CreateTransactionResponse { index: TX_INDEX.into() }))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[case::unauthorized(Err(crate::Error::MissingAccessToken))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[tokio::test]
    async fn test_create_purchase_request(#[case] expected: Result<CreateTransactionResponse>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();
        sdk.set_network(example_network(Currency::Iota));
        let mut mock_server = None;

        match &expected {
            Ok(_) => {
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(MockWalletManager::new()),
                });
                sdk.access_token = Some(TOKEN.clone());

                let mock_response = CreateTransactionResponse { index: TX_INDEX.into() };
                let body = serde_json::to_string(&mock_response).unwrap();

                mock_server = Some(
                    srv.mock("POST", "/api/transactions/create")
                        .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
                        .match_header(HEADER_X_APP_USERNAME, USERNAME)
                        .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
                        .with_status(201)
                        .with_header("content-type", "application/json")
                        .with_body(body)
                        .expect(1)
                        .create(),
                );
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 0, 0).await;
            }
        }

        // Act
        let amount = CryptoAmount::try_from(dec!(10.0)).unwrap();
        let response = sdk
            .create_purchase_request("receiver", amount, "hash", "app_data", "CLIK")
            .await;

        // Assert
        match expected {
            Ok(resp) => {
                assert_eq!(response.unwrap(), resp.index);
            }
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
        if let Some(m) = mock_server {
            m.assert();
        }
    }

    #[rstest]
    #[case::success(Ok(()))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[case::unauthorized(Err(crate::Error::MissingAccessToken))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[case::invalid_tx(Err(crate::Error::Wallet(WalletError::InvalidTransaction(format!(
        "Transaction is not valid, current status: {}.",
        ApiTxStatus::Invalid(vec!["ReceiverNotVerified".to_string()])
    )))))]
    #[tokio::test]
    async fn test_commit_transaction(#[case] expected: Result<()>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();
        sdk.set_network(example_network(Currency::Iota));
        let mut mock_server_details = None;
        let mut mock_server_commit = None;

        match &expected {
            Ok(_) => {
                let mock_user_repo = example_get_user(SwapPaymentDetailKey::Iota, false, 1, KycType::Undefined);
                sdk.repo = Some(Box::new(mock_user_repo));

                let mut mock_wallet_manager = MockWalletManager::new();
                mock_wallet_manager.expect_try_get().returning(move |_, _, _, _, _| {
                    let mut mock_wallet_user = MockWalletUser::new();
                    mock_wallet_user
                        .expect_send_transaction()
                        .once()
                        .returning(|_, _, _| Ok("tx_id".to_string()));

                    Ok(WalletBorrow::from(mock_wallet_user))
                });
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(mock_wallet_manager),
                });

                sdk.access_token = Some(TOKEN.clone());

                let mock_tx_response = GetTransactionDetailsResponse {
                    system_address: "".to_string(),
                    amount: dec!(5.0),
                    status: ApiTxStatus::Valid,
                    network: example_api_network(Currency::Iota),
                };
                let body = serde_json::to_string(&mock_tx_response).unwrap();

                mock_server_details = Some(
                    srv.mock("GET", "/api/transactions/details?index=123")
                        .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
                        .match_header(HEADER_X_APP_USERNAME, USERNAME)
                        .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
                        .with_status(200)
                        .with_body(&body)
                        .with_header("content-type", "application/json")
                        .create(),
                );

                mock_server_commit = Some(
                    srv.mock("POST", "/api/transactions/commit")
                        .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
                        .match_header(HEADER_X_APP_USERNAME, USERNAME)
                        .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
                        .with_status(202)
                        .expect(1)
                        .with_header("content-type", "application/json")
                        .create(),
                );
            }
            Err(crate::Error::Wallet(WalletError::InvalidTransaction(_))) => {
                let mock_user_repo = example_get_user(SwapPaymentDetailKey::Iota, false, 1, KycType::Undefined);
                sdk.repo = Some(Box::new(mock_user_repo));

                let mock_wallet_manager = example_wallet_borrow();
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(mock_wallet_manager),
                });

                sdk.access_token = Some(TOKEN.clone());

                let mock_tx_response = GetTransactionDetailsResponse {
                    system_address: "".to_string(),
                    amount: dec!(5.0),
                    status: ApiTxStatus::Invalid(vec!["ReceiverNotVerified".to_string()]),
                    network: example_api_network(Currency::Iota),
                };
                let body = serde_json::to_string(&mock_tx_response).unwrap();

                mock_server_details = Some(
                    srv.mock("GET", "/api/transactions/details?index=123")
                        .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
                        .match_header(HEADER_X_APP_USERNAME, USERNAME)
                        .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
                        .with_status(200)
                        .with_body(&body)
                        .with_header("content-type", "application/json")
                        .create(),
                );
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 1, 1).await;
            }
        }

        // Act
        let pin = EncryptionPin::try_from_string("1234").unwrap();
        let response = sdk.confirm_purchase_request(&pin, PURCHASE_ID).await;

        // Assert
        match expected {
            Ok(_) => response.unwrap(),
            Err(ref err) => {
                assert_eq!(response.unwrap_err().to_string(), err.to_string());
            }
        }
        if mock_server_details.is_some() & mock_server_commit.is_some() {
            mock_server_details.unwrap().assert();
            mock_server_commit.unwrap().assert();
        }
    }

    #[rstest]
    #[case::success(Ok(example_tx_details()))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[case::unauthorized(Err(crate::Error::MissingAccessToken))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[tokio::test]
    async fn test_get_purchase_details(#[case] expected: Result<GetTransactionDetailsResponse>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();
        let mut mock_server = None;

        match &expected {
            Ok(_) => {
                sdk.repo = Some(Box::new(MockUserRepo::new()));
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(MockWalletManager::new()),
                });
                sdk.access_token = Some(TOKEN.clone());

                let mock_response = example_tx_details();
                let body = serde_json::to_string(&mock_response).unwrap();

                mock_server = Some(
                    srv.mock("GET", "/api/transactions/details?index=123")
                        .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
                        .match_header(HEADER_X_APP_USERNAME, USERNAME)
                        .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
                        .with_status(200)
                        .with_body(&body)
                        .with_header("content-type", "application/json")
                        .with_body(&body)
                        .create(),
                );
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 0, 0).await;
            }
        }

        // Act
        let response = sdk.get_purchase_details(PURCHASE_ID).await;

        // Assert
        match expected {
            Ok(resp) => {
                assert_eq!(
                    GetTransactionDetailsResponse {
                        system_address: response.as_ref().unwrap().system_address.clone(),
                        amount: response.as_ref().unwrap().amount,
                        status: response.unwrap().status,
                        network: example_api_network(Currency::Iota),
                    },
                    resp
                );
            }
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
        if let Some(m) = mock_server {
            m.assert();
        }
    }

    #[rstest]
    #[case::success(Ok(()))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[tokio::test]
    async fn test_send_amount(#[case] expected: Result<()>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();
        sdk.set_network(example_network(Currency::Iota));

        match &expected {
            Ok(_) => {
                let mock_user_repo = example_get_user(SwapPaymentDetailKey::Iota, false, 1, KycType::Undefined);
                sdk.repo = Some(Box::new(mock_user_repo));

                let mut mock_wallet_manager = MockWalletManager::new();
                mock_wallet_manager.expect_try_get().returning(move |_, _, _, _, _| {
                    let mut mock_wallet = MockWalletUser::new();
                    mock_wallet
                        .expect_send_amount()
                        .times(1)
                        .returning(move |_, _, _, _| Ok(example_wallet_transaction()));
                    Ok(WalletBorrow::from(mock_wallet))
                });

                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(mock_wallet_manager),
                });
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 1, 0).await;
            }
        }

        // Act
        let amount = CryptoAmount::try_from(dec!(25.0)).unwrap();
        let response = sdk
            .send_amount(
                &EncryptionPin::try_from_string("1234").unwrap(),
                "smrq1...",
                amount,
                Some(Vec::from([8, 16])),
                Some(Vec::from([8, 16])),
                Some(String::from("test message")),
            )
            .await;

        // Assert
        match expected {
            Ok(_) => response.unwrap(),
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    #[tokio::test]
    async fn test_send_amount_with_eth_should_trigger_a_call_to_set_wallet_transaction() {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();
        sdk.set_network(example_network(Currency::Iota));

        let wallet_transaction = WalletTxInfo {
            date: String::new(),
            block_id: Some(String::new()),
            transaction_id: String::from("tx_id"),
            incoming: false,
            amount: 5.0,
            network: String::from("ETH"),
            status: format!("{:?}", InclusionState::Pending),
            explorer_url: Some(String::new()),
        };

        let wallet_transactions = vec![wallet_transaction.clone()].to_owned();

        let mut mock_user_repo = example_get_user(SwapPaymentDetailKey::Eth, false, 2, KycType::Undefined);
        mock_user_repo
            .expect_set_wallet_transactions()
            .times(1)
            .returning(move |_, expected_wallet_transactions| {
                assert_eq!(wallet_transactions, expected_wallet_transactions);
                Ok(())
            });
        sdk.repo = Some(Box::new(mock_user_repo));

        let mut mock_wallet_manager = MockWalletManager::new();
        mock_wallet_manager.expect_try_get().returning(move |_, _, _, _, _| {
            let mut mock_wallet = MockWalletUser::new();
            mock_wallet
                .expect_send_amount_eth()
                .times(1)
                .returning(move |_, _, _, _| Ok(String::from("tx_id")));

            let value = wallet_transaction.clone();
            mock_wallet
                .expect_get_wallet_tx()
                .times(1)
                .returning(move |_| Ok(value.clone()));

            Ok(WalletBorrow::from(mock_wallet))
        });

        sdk.active_user = Some(crate::types::users::ActiveUser {
            username: USERNAME.into(),
            wallet_manager: Box::new(mock_wallet_manager),
        });

        // Act
        let amount = CryptoAmount::try_from(dec!(5.0)).unwrap();
        let response = sdk
            .send_amount(
                &EncryptionPin::try_from_string("1234").unwrap(),
                "0xb0b...",
                amount,
                Some(Vec::from([8, 16])),
                Some(Vec::from([8, 16])),
                Some(String::from("test message")),
            )
            .await;

        // Assert
        response.unwrap()
    }

    #[rstest]
    #[case::success(Ok(examples_wallet_tx_list()))]
    #[case::unauthorized(Err(crate::Error::MissingAccessToken))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[tokio::test]
    async fn test_get_tx_list(#[case] expected: Result<GetTxsDetailsResponse>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        let start = 1u32;
        let limit = 5u32;

        let mut mock_server = None;
        match &expected {
            Ok(_) => {
                let mock_user_repo = example_get_user(SwapPaymentDetailKey::Iota, false, 1, KycType::Undefined);
                sdk.repo = Some(Box::new(mock_user_repo));
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(MockWalletManager::new()),
                });
                sdk.access_token = Some(TOKEN.clone());

                let txs_details_mock_response = examples_wallet_tx_list();
                let body = serde_json::to_string(&txs_details_mock_response).unwrap();

                mock_server = Some(
                    srv.mock("GET", "/api/transactions/txs-details")
                        .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
                        .match_header(HEADER_X_APP_USERNAME, USERNAME)
                        .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
                        .match_query(Matcher::Exact(format!("is_sender=false&start={start}&limit={limit}")))
                        .with_status(200)
                        .with_body(&body)
                        .expect(1)
                        .with_header("content-type", "application/json")
                        .with_body(&body)
                        .create(),
                );
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 0, 1).await;
            }
        }

        // Act
        let response = sdk.get_tx_list(start, limit).await;

        // Assert
        match expected {
            Ok(_) => assert!(response.is_ok()),
            Err(ref err) => {
                assert_eq!(response.unwrap_err().to_string(), err.to_string());
            }
        }
        if let Some(m) = mock_server {
            m.assert();
        }
    }
}
