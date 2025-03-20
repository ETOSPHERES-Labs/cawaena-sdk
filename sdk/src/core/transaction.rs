use super::Sdk;
use crate::backend::transactions::{
    commit_transaction, create_new_transaction, get_transaction_details, get_transactions_list,
};
use crate::error::Result;
use crate::types::currencies::CryptoAmount;
use crate::types::transactions::{GasCostEstimation, PurchaseDetails};
use crate::types::{
    newtypes::EncryptionPin,
    transactions::{TxInfo, TxList},
};
use crate::wallet::error::WalletError;
use crate::wallet_user::TransactionIntent;
use api_types::api::networks::{ApiNetwork, ApiProtocol};
use api_types::api::transactions::{ApiApplicationMetadata, ApiTxStatus, PurchaseModel, Reason};
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
    /// The purchase ID. This is an internal index used to reference the transaction in etopay
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
        let Some(_active_user) = &self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;
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
        let response = create_new_transaction(config, access_token, receiver, network.key, amount, metadata).await?;
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
        let Some(_active_user) = &self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        let access_token = self
            .access_token
            .as_ref()
            .ok_or(crate::error::Error::MissingAccessToken)?;

        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;
        let response = get_transaction_details(config, access_token, purchase_id).await?;

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

        let config = self.config.as_mut().ok_or(crate::Error::MissingConfig)?;
        let access_token = self
            .access_token
            .as_ref()
            .ok_or(crate::error::Error::MissingAccessToken)?;
        let tx_details = get_transaction_details(config, access_token, purchase_id).await?;

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
        let network: ApiNetwork = tx_details.network.clone();
        if network.key != current_network.key {
            return Err(WalletError::InvalidTransaction(format!(
                "Transaction to commit is in network_key {:?}, but {:?} is the currently active current_network_key.",
                network.key, current_network.key
            )))?;
        }

        let wallet = active_user
            .wallet_manager
            .try_get(config, &self.access_token, repo, network, pin)
            .await?;

        let amount = tx_details.amount.try_into()?;

        let intent = TransactionIntent {
            address_to: tx_details.system_address.clone(),
            amount,
            data: Some(purchase_id.to_string().into_bytes()),
        };

        let tx_id = wallet.send_amount(&intent).await?;

        // Store tx details for all networks other than Stardust (which stores transactions internally)
        // TODO: rework this to always store the transactions
        if let ApiProtocol::Evm { .. } = tx_details.network.protocol {
            let tx_id = wallet.send_amount(&intent).await?;

            let newly_created_transaction = wallet.get_wallet_tx(&tx_id).await?;
            let mut user = repo.get(&active_user.username)?;
            user.wallet_transactions.push(newly_created_transaction);
            let _ = repo.set_wallet_transactions(&active_user.username, user.wallet_transactions);
        }

        if let ApiProtocol::EvmERC20 { .. } = tx_details.network.protocol {
            let tx_id = wallet.send_amount(&intent).await?;

            let newly_created_transaction = wallet.get_wallet_tx(&tx_id).await?;
            let mut user = repo.get(&active_user.username)?;
            user.wallet_transactions.push(newly_created_transaction);
            let _ = repo.set_wallet_transactions(&active_user.username, user.wallet_transactions);
        }

        debug!("Transaction id on network: {tx_id}");

        commit_transaction(config, access_token, purchase_id, &tx_id).await?;

        Ok(())
    }

    /// Send amount to receiver address
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN of the user.
    /// * `address` - The receiver's address.
    /// * `amount` - The amount to send.
    /// * `data` - The associated data with the tag. Optional.
    ///
    /// # Returns
    ///
    /// Returns `Ok(String)` containing the transaction hash if the amount is sent successfully.
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
        data: Option<Vec<u8>>,
    ) -> Result<String> {
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
        let intent = TransactionIntent {
            address_to: address.to_string(),
            amount,
            data,
        };

        let tx_id = match network.protocol {
            ApiProtocol::EvmERC20 {
                chain_id: _,
                contract_address: _,
            } => wallet.send_amount(&intent).await?,
            ApiProtocol::Evm { chain_id: _ } => {
                let tx_id = wallet.send_amount(&intent).await?;

                // store the created transaction in the repo
                let newly_created_transaction = wallet.get_wallet_tx(&tx_id).await?;
                let user = repo.get(&active_user.username)?;
                let mut wallet_transactions = user.wallet_transactions;
                wallet_transactions.push(newly_created_transaction);
                let _ = repo.set_wallet_transactions(&active_user.username, wallet_transactions);
                tx_id
            }
            ApiProtocol::Stardust {} => wallet.send_amount(&intent).await?,
        };

        Ok(tx_id)
    }

    /// Estimate gas for sending amount to receiver
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN of the user.
    /// * `address` - The receiver's address.
    /// * `amount` - The amount to send.
    /// * `data` - The associated data with the tag. Optional.
    ///
    /// # Returns
    ///
    /// Returns the gas estimation.
    ///
    /// # Errors
    ///
    /// Returns an error if the user or wallet is not initialized, if there is an error verifying the PIN,
    /// or if there is an error estimating the gas.
    pub async fn estimate_gas(
        &mut self,
        pin: &EncryptionPin,
        address: &str,
        amount: CryptoAmount,
        data: Option<Vec<u8>>,
    ) -> Result<GasCostEstimation> {
        info!("Estimating gas for sending amount {amount:?} to receiver {address}");
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
        let intent = TransactionIntent {
            address_to: address.to_string(),
            amount,
            data,
        };

        let estimate = wallet.estimate_gas_cost(&intent).await?;
        info!("Estimate: {estimate:?}");

        Ok(estimate)
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
        let txs_list = get_transactions_list(config, access_token, start, limit).await?;
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
                        currency: val.incoming.network.display_symbol,
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
        example_api_network, example_api_networks, example_get_user, example_tx_details, example_tx_metadata,
        example_wallet_borrow, set_config, AUTH_PROVIDER, HEADER_X_APP_NAME, PURCHASE_ID, TOKEN, TX_INDEX, USERNAME,
    };
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
    use mockito::Matcher;
    use rstest::rstest;
    use rust_decimal_macros::dec;

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
                    network: example_api_network(String::from("IOTA")),
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
                    network: example_api_network(String::from("IOTA")),
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
        sdk.set_networks(example_api_networks());
        sdk.set_network(String::from("IOTA")).await.unwrap();
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
        sdk.set_networks(example_api_networks());
        sdk.set_network(String::from("IOTA")).await.unwrap();
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
                        .expect_send_amount()
                        .once()
                        .returning(|_| Ok("tx_id".to_string()));

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
                    network: example_api_network(String::from("IOTA")),
                };
                let body = serde_json::to_string(&mock_tx_response).unwrap();

                mock_server_details = Some(
                    srv.mock("GET", "/api/transactions/details?index=123")
                        .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
                        .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
                        .with_status(200)
                        .with_body(&body)
                        .with_header("content-type", "application/json")
                        .create(),
                );

                mock_server_commit = Some(
                    srv.mock("POST", "/api/transactions/commit")
                        .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
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
                    network: example_api_network(String::from("IOTA")),
                };
                let body = serde_json::to_string(&mock_tx_response).unwrap();

                mock_server_details = Some(
                    srv.mock("GET", "/api/transactions/details?index=123")
                        .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
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
                        network: example_api_network(String::from("IOTA")),
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
        sdk.set_networks(example_api_networks());
        sdk.set_network(String::from("IOTA")).await.unwrap();

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
                        .returning(move |_| Ok(String::from("transaction id")));
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
                Some(String::from("test message").into_bytes()),
            )
            .await;

        // Assert
        match expected {
            Ok(_) => {
                response.unwrap();
            }
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
        sdk.set_networks(example_api_networks());
        sdk.set_network(String::from("ETH")).await.unwrap();

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
                .expect_send_amount()
                .times(1)
                .returning(move |_| Ok(String::from("tx_id")));

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
                Some(String::from("test message").into_bytes()),
            )
            .await;

        // Assert
        response.unwrap();
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
