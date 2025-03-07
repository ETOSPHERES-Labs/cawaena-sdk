use super::error::Result;
use super::wallet_user::WalletUser;
use crate::types::currencies::{CryptoAmount, Currency};
use crate::types::transactions::{GasCostEstimation, WalletTxInfo, WalletTxInfoList};
use crate::wallet::error::WalletError;
use alloy::eips::BlockNumberOrTag;
use alloy::hex::encode;
use alloy::rpc::types::{TransactionInput, TransactionRequest};
use alloy::transports::http::Http;
use alloy::{
    consensus::{SignableTransaction, Signed, TxEip1559, TxEnvelope},
    hex::ToHexExt,
    primitives::Address,
    primitives::U256,
    providers::{Provider, ProviderBuilder},
};
use alloy_consensus::Transaction;
use alloy_primitives::{PrimitiveSignature, TxHash};
use alloy_provider::RootProvider;
use async_trait::async_trait;
use iota_sdk::client::secret::SecretManage;
use iota_sdk::crypto::keys::bip44::Bip44;
use iota_sdk::wallet::account::types::InclusionState;
use iota_sdk::{
    client::{
        api::GetAddressesOptions,
        constants::ETHER_COIN_TYPE,
        secret::{GenerateAddressOptions, SecretManager},
    },
    crypto::keys::bip39::Mnemonic,
    types::block::{address::Hrp, payload::TaggedDataPayload},
    wallet::{
        account::{types::Transaction as IotaWalletTransaction, Account},
        ClientOptions,
    },
};
use log::{error, info};
use reqwest::{Client, Url};
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::fmt::Debug;
use std::ops::{Div, Mul};
use std::path::Path;
use std::str::FromStr;

///The name of the application used as the account name in the wallet
const APP_NAME: &str = "standalone";

const WEI_TO_ETH_DIVISOR: CryptoAmount = unsafe { CryptoAmount::new_unchecked(dec!(1_000_000_000_000_000_000)) }; // SAFETY: the value is non-negative

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct UnifiedTransactionMetadata {
    tag: Option<Vec<u8>>,
    data: Option<Vec<u8>>,
    message: Option<String>,
}

impl UnifiedTransactionMetadata {
    fn from_iota_tag_and_metadata(
        tag: Option<TaggedDataPayload>,
        metadata: Option<String>,
    ) -> UnifiedTransactionMetadata {
        match tag {
            Some(t) => UnifiedTransactionMetadata {
                tag: Some(t.tag().to_vec()),
                data: Some(t.data().to_vec()),
                message: metadata,
            },
            None => UnifiedTransactionMetadata {
                tag: None,
                data: None,
                message: metadata,
            },
        }
    }
}

/// [`WalletUser`] implementation for ETH
#[derive(Debug)]
pub struct WalletImplEth {
    /// State for the account manager to be passed to calling functions
    account_manager: iota_sdk::wallet::Wallet,

    // /// Store a copy of the node urls.
    node_url: String,
    chain_id: u64,

    /// Rpc client
    http_provider: RootProvider<Http<Client>>,
}

impl WalletImplEth {
    #[cfg(test)]
    pub async fn new_with_mocked_provider(
        mnemonic: Mnemonic,
        path: &Path,
        http_provider: RootProvider<Http<Client>>,
        node_url: String,
        chain_id: u64,
    ) -> Result<Self> {
        let node_urls: &[&str] = &[&node_url];

        info!("Used node_urls: {:?}", node_urls);
        info!("Eth eth_node_url: {:?}", node_urls);
        let client_options = ClientOptions::new()
            .with_local_pow(false)
            .with_fallback_to_local_pow(true)
            .with_nodes(node_urls)?;

        // we need to make sure the path exists, or we will get IO errors, but only if we are not on wasm
        #[cfg(not(target_arch = "wasm32"))]
        if let Err(e) = std::fs::create_dir_all(path) {
            error!("Could not create the wallet directory: {e:?}");
        }

        let account_manager = {
            let secret_manager = SecretManager::try_from_mnemonic(mnemonic)?;
            iota_sdk::wallet::Wallet::builder()
                .with_client_options(client_options)
                .with_coin_type(Currency::Eth.coin_type())
                .with_secret_manager(secret_manager)
                .with_storage_path(path) // we still need this since the account stores stuff in jammdb
                .finish()
                .await?
        };

        let account = account_manager.get_account(APP_NAME).await;

        if let Err(account_error) = account {
            info!("{:?}", account_error);
            info!("Creating a new account with alias {APP_NAME}");
            account_manager
                .create_account()
                .with_alias(String::from(APP_NAME))
                .finish()
                .await?;
        }

        info!("Wallet creation successful");

        Ok(WalletImplEth {
            account_manager,
            node_url,
            chain_id,
            http_provider,
        })
    }

    /// Creates a new [`WalletImplEth`] from the specified [`Mnemonic`].
    pub async fn new(mnemonic: Mnemonic, path: &Path, node_url: String, chain_id: u64) -> Result<Self> {
        let node_urls = vec![node_url.as_str()];

        info!("Used node_urls: {:?}", node_urls);
        let client_options = ClientOptions::new()
            .with_local_pow(false)
            .with_fallback_to_local_pow(true)
            .with_nodes(&node_urls)?;

        // we need to make sure the path exists, or we will get IO errors, but only if we are not on wasm
        #[cfg(not(target_arch = "wasm32"))]
        if let Err(e) = std::fs::create_dir_all(path) {
            error!("Could not create the wallet directory: {e:?}");
        }

        let account_manager = {
            let secret_manager = SecretManager::try_from_mnemonic(mnemonic)?;
            iota_sdk::wallet::Wallet::builder()
                .with_client_options(client_options)
                .with_coin_type(Currency::Eth.coin_type())
                .with_secret_manager(secret_manager)
                .with_storage_path(path) // we still need this since the account stores stuff in jammdb
                .finish()
                .await?
        };

        let account = account_manager.get_account(APP_NAME).await;

        if let Err(account_error) = account {
            info!("{:?}", account_error);
            info!("Creating a new account with alias {APP_NAME}");
            account_manager
                .create_account()
                .with_alias(String::from(APP_NAME))
                .finish()
                .await?;
        }

        let url =
            Url::parse(node_urls[0]).map_err(|e| WalletError::Parse(format!("could not parse the url: {e:?}")))?;
        let http_provider = ProviderBuilder::new().on_http(url);

        info!("Wallet creation successful");

        Ok(WalletImplEth {
            account_manager,
            chain_id,
            node_url,
            http_provider,
        })
    }

    async fn sign_transaction(&self, transaction: TxEip1559, sender_addr_raw: String) -> Result<Signed<TxEip1559>> {
        // Prepare transaction body for signing
        let mut buf = vec![];
        transaction.encode_for_signing(&mut buf);
        let encoded_transaction: &[u8] = &buf;

        let bip44_chain = Bip44::new(ETHER_COIN_TYPE)
            .with_account(0)
            .with_change(false as _)
            .with_address_index(0);

        let secret_manager = self.account_manager.get_secret_manager().write().await;

        // Next: sign message with external signer
        let (public_key, recoverable_signature) = secret_manager
            .sign_secp256k1_ecdsa(encoded_transaction, bip44_chain)
            .await?;

        // Validation: recover address and compare it with sender address
        let recovered_sender_addr_raw = recoverable_signature
            .recover_evm_address(encoded_transaction)
            .ok_or(WalletError::SignatureAddressRecoveryError(
                "Could not recover evm address from recoverable signature".to_string(),
            ))?
            .as_ref()
            .encode_hex_with_prefix();

        if recovered_sender_addr_raw != sender_addr_raw {
            let error_msg = format!("{} != {}", recovered_sender_addr_raw, sender_addr_raw);
            return Err(WalletError::RecoveredAddressDoesNotMatchSenderAddress(error_msg));
        }

        // Validation: retrieve address from public key and compare it with generated address
        let public_key_addr_raw = public_key.evm_address().as_ref().encode_hex_with_prefix();
        if public_key_addr_raw != sender_addr_raw {
            let error_msg = format!("{} != {}", public_key_addr_raw, sender_addr_raw);
            return Err(WalletError::PublicKeyAddressDoesNotMatchSenderAddress(error_msg));
        }

        // Next: Read parity (v)
        let recoverable_signature_as_bytes = recoverable_signature.to_bytes();
        let parity = match recoverable_signature_as_bytes[64] {
            0x00 => false,
            0x01 => true,
            _ => return Err(WalletError::InvalidParityByte()),
        };

        // Next: Extract signature
        // ECDSA signature is 512 bits (64 bytes)
        let recoverable_signature_as_bytes = recoverable_signature.as_ref().to_bytes();

        // Next: Create signature with r, s and v
        let signature = PrimitiveSignature::from_bytes_and_parity(&recoverable_signature_as_bytes, parity);

        // Next: Sign transaction
        let signed_transaction = transaction.clone().into_signed(signature);

        // Validation: recover address and compare it with generated address
        let signer_addr = signed_transaction.recover_signer()?;

        if signer_addr.encode_hex_with_prefix() != sender_addr_raw {
            let error_msg = format!("{} != {}", signer_addr.encode_hex_with_prefix(), sender_addr_raw);
            return Err(WalletError::SignerAddressDoesNotMatchSenderAddress(error_msg));
        }

        Ok(signed_transaction)
    }

    #[allow(clippy::too_many_arguments)]
    async fn build_transaction(
        &self,
        from_addr: Address,
        to_addr: Address,
        value: U256,
        gas_limit: u64,
        max_fee_per_gas: u128,
        max_priority_fee_per_gas: u128,
        chain_id: u64,
        tag: Option<TaggedDataPayload>,
        metadata: Option<String>,
    ) -> Result<TxEip1559> {
        let nonce = self.get_next_nonce(from_addr).await?;

        let input: TransactionInput = match tag.clone() {
            Some(_) => {
                let unified_transaction_metadata =
                    UnifiedTransactionMetadata::from_iota_tag_and_metadata(tag, metadata);
                let serialized_unified_transaction_metadata = serde_json::to_string(&unified_transaction_metadata)
                    .map_err(WalletError::UnifiedTransactionMetadataSerializationError)?;
                let encoded_unified_transaction_metadata = encode(serialized_unified_transaction_metadata);
                let bytes = encoded_unified_transaction_metadata.into();
                TransactionInput::new(bytes)
            }
            None => TransactionInput::default(),
        };

        let tx = TxEip1559 {
            chain_id,
            nonce,
            gas_limit,
            max_fee_per_gas,
            max_priority_fee_per_gas,
            to: alloy_primitives::TxKind::Call(to_addr),
            value,
            access_list: Default::default(),
            input: input.into_input().unwrap_or_default(),
        };

        Ok(tx)
    }

    /// Gets nonce (latest transaction count) for Eth transaction
    ///
    /// # Returns
    ///
    /// Returns latest transaction count as a `u64` if successful, or an `Error` if it fails.
    ///
    /// # Errors
    ///
    /// This function can return an error if it fails to synchronize the wallet or encounters any other issues.
    async fn get_next_nonce(&self, addr: Address) -> Result<u64> {
        let nonce = self.http_provider.get_transaction_count(addr).await?;

        Ok(nonce)
    }

    async fn get_wallet_addr(&self) -> Result<Address> {
        let wallet_addr_raw = self.get_wallet_addr_raw().await?;
        let wallet_addr = Address::from_str(&wallet_addr_raw)
            .map_err(|e| WalletError::Parse(format!("could not parse the address: {e:?}")))?;

        Ok(wallet_addr)
    }

    async fn get_wallet_addr_raw(&self) -> Result<String> {
        let wallet_addr_raw = self.get_address().await?;
        Ok(wallet_addr_raw)
    }

    async fn fetch_block_date(&self, block_number: BlockNumberOrTag) -> Result<Option<u64>> {
        let block = self
            .http_provider
            .get_block_by_number(block_number, alloy::network::primitives::BlockTransactionsKind::Full)
            .await?;
        let date = match block {
            Some(b) => Some(b.header.timestamp),
            None => None,
        };

        Ok(date)
    }

    async fn verify_transaction_success(&self, tx_hash: TxHash) -> Result<Option<bool>> {
        let receipt = self.http_provider.get_transaction_receipt(tx_hash).await?;

        let status = match receipt {
            Some(r) => Some(r.inner.is_success()),
            None => None,
        };

        Ok(status)
    }

    fn map_transaction_success_to_inclusion_state(tx_status: Option<bool>) -> InclusionState {
        match tx_status {
            Some(true) => InclusionState::Confirmed,
            Some(false) => InclusionState::Conflicting,
            None => InclusionState::Pending,
        }
    }

    async fn is_transaction_incoming(&self, receiver_addr: Option<Address>) -> Result<bool> {
        let wallet_addr = self.get_wallet_addr().await?;
        Ok(Self::compare_addresses(wallet_addr, receiver_addr))
    }

    fn compare_addresses(address: Address, receiver: Option<Address>) -> bool {
        receiver.is_some_and(|r| address == r)
    }

    fn convert_wei_to_eth(value_wei: CryptoAmount) -> CryptoAmount {
        value_wei.div(WEI_TO_ETH_DIVISOR)
    }

    fn convert_eth_to_wei(value_eth: CryptoAmount) -> CryptoAmount {
        value_eth.mul(WEI_TO_ETH_DIVISOR)
    }

    fn convert_alloy_256_to_crypto_amount(v: alloy_primitives::Uint<256, 4>) -> Result<CryptoAmount> {
        let value_i128 = v.to::<i128>();
        let value_decimal = Decimal::from_i128(value_i128);
        match value_decimal {
            Some(result_decimal) => {
                let crypto_amount = CryptoAmount::try_from(result_decimal);
                match crypto_amount {
                    Ok(result_crypto_amount) => Ok(result_crypto_amount),
                    Err(_) => Err(WalletError::ConversionError(format!(
                        "could not convert decimal to crypto amount: {result_decimal:?}"
                    ))),
                }
            }
            None => Err(WalletError::ConversionError(format!(
                "could not convert alloy 256 to decimal: {v:?}"
            ))),
        }
    }

    fn convert_crypto_amount_to_u256(v: CryptoAmount) -> Result<alloy_primitives::U256> {
        let value_decimal = v.inner();
        let value_i128: i128 = value_decimal.try_into()?;
        let value = U256::from(value_i128);

        Ok(value)
    }
}

#[allow(unused_variables)] // allow unused parameters until everything is implemented
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(test, mockall::automock)]
impl WalletUser for WalletImplEth {
    async fn get_address(&self) -> Result<String> {
        let options = GenerateAddressOptions::default();
        log::debug!(
            "[EVM ADDRESS GENERATION] generating address, internal: {}",
            options.internal
        );
        let account: Account = self.account_manager.get_account(APP_NAME).await?;
        let secret_manager = account.get_secret_manager();
        let secret_manager_lock = secret_manager.read().await;
        let range = 0..1;

        let addresses = secret_manager_lock
            .generate_evm_addresses(GetAddressesOptions {
                coin_type: ETHER_COIN_TYPE,
                account_index: 0,
                range,
                options: Some(options),
                bech32_hrp: Hrp::from_str_unchecked("eth"),
            })
            .await?;

        if addresses.is_empty() {
            return Err(WalletError::EmptyWalletAddress);
        }

        Ok(addresses[0].clone())
    }

    async fn get_balance(&self) -> Result<CryptoAmount> {
        let wallet_addr = self.get_wallet_addr().await?;
        let balance: alloy_primitives::Uint<256, 4> = self
            .http_provider
            .get_balance(wallet_addr)
            .await
            .map_err(|e| WalletError::Rpc(format!("{e:?}")))?;

        let balance_wei_crypto_amount = Self::convert_alloy_256_to_crypto_amount(balance)?;
        let balance_eth_crypto_amount = Self::convert_wei_to_eth(balance_wei_crypto_amount);
        Ok(balance_eth_crypto_amount)
    }

    async fn send_amount(
        &self,
        address: &str,
        amount: CryptoAmount,
        tag: Option<TaggedDataPayload>,
        message: Option<String>,
    ) -> Result<IotaWalletTransaction> {
        Err(WalletError::WalletFeatureNotImplemented)
    }

    async fn send_amount_eth(
        &self,
        address: &str,
        amount: CryptoAmount,
        tag: Option<TaggedDataPayload>,
        message: Option<String>,
    ) -> Result<String> {
        let addr_from = self.get_wallet_addr().await?;
        let addr_to = Address::from_str(address)?;

        let wallet_addr_raw = self.get_wallet_addr_raw().await?;
        let amount_wei = Self::convert_eth_to_wei(amount);
        let amount_wei_u256 = Self::convert_crypto_amount_to_u256(amount_wei)?;

        let mut transaction = self
            .build_transaction(
                addr_from,
                addr_to,
                amount_wei_u256,
                0,
                0,
                0,
                self.chain_id,
                tag,
                message,
            )
            .await?;

        // Estimate gas cost
        let gas_cost = self.estimate_gas_cost_eip1559(transaction.clone()).await?;
        transaction.gas_limit = gas_cost.gas_limit;
        transaction.max_fee_per_gas = gas_cost.max_fee_per_gas;
        transaction.max_priority_fee_per_gas = gas_cost.max_priority_fee_per_gas;

        // Sign transaction
        let signed_transaction = self.sign_transaction(transaction, wallet_addr_raw).await?;
        let transaction_envelope = TxEnvelope::from(signed_transaction);

        // Send transaction
        let pending_tx = self.http_provider.send_tx_envelope(transaction_envelope).await?;
        Ok(pending_tx.tx_hash().to_string())
    }

    async fn send_transaction(&self, index: &str, address: &str, amount: CryptoAmount) -> Result<String> {
        unimplemented!("use send_transaction_eth");
    }

    // todo: unlock method for other protocols by passing interface instead of hardcoded list of fields (eip 1559 at the moment)
    async fn send_transaction_eth(&self, index: &str, address: &str, amount: CryptoAmount) -> Result<String> {
        let addr_from = self.get_wallet_addr().await?;
        let addr_to = Address::from_str(address)?;

        let wallet_addr_raw = self.get_wallet_addr_raw().await?;
        let amount_wei = Self::convert_eth_to_wei(amount);
        let amount_wei_u256 = Self::convert_crypto_amount_to_u256(amount_wei)?;

        let mut transaction = self
            .build_transaction(addr_from, addr_to, amount_wei_u256, 0, 0, 0, self.chain_id, None, None)
            .await?;

        // Estimate gas cost
        let gas_cost = self.estimate_gas_cost_eip1559(transaction.clone()).await?;
        transaction.gas_limit = gas_cost.gas_limit;
        transaction.max_fee_per_gas = gas_cost.max_fee_per_gas;
        transaction.max_priority_fee_per_gas = gas_cost.max_priority_fee_per_gas;

        // Sign transaction
        let signed_transaction = self.sign_transaction(transaction, wallet_addr_raw).await?;
        let transaction_envelope = TxEnvelope::from(signed_transaction);

        // Send transaction
        let pending_tx = self.http_provider.send_tx_envelope(transaction_envelope).await?;
        Ok(pending_tx.tx_hash().to_string())
    }

    async fn sync_wallet(&self) -> Result<()> {
        unimplemented!("dead interface");
    }

    async fn sync_transactions(&self, transactions: &mut [WalletTxInfo], start: usize, limit: usize) -> Result<()> {
        for i in start..start + limit {
            if let Some(transaction) = transactions.get_mut(i) {
                let synchronized_transaction = self.get_wallet_tx(&transaction.transaction_id).await;
                match synchronized_transaction {
                    Ok(stx) => *transaction = stx,
                    Err(e) => {
                        // On error, return historical (cached) transaction data
                        log::debug!(
                        "[sync_transactions] could not retrieve data about transaction from the network, transaction: {:?}, error: {:?}",
                        transaction.clone(), e
                        );
                    }
                }
            } else {
                break;
            }
        }

        Ok(())
    }

    // The network does not provide information about historical transactions
    // (they can be retrieved manually, but this is a time-consuming process),
    // so the handling of this method is implemented at the SDK level.
    async fn get_wallet_tx_list(&self, start: usize, limit: usize) -> Result<WalletTxInfoList> {
        Err(WalletError::WalletFeatureNotImplemented)
    }

    async fn get_wallet_tx(&self, transaction_id: &str) -> Result<WalletTxInfo> {
        let account = self.account_manager.get_account(APP_NAME).await?;
        let wallet_addr = self.get_wallet_addr().await?;
        let transaction_hash = TxHash::from_str(transaction_id)?;
        let transaction = self.http_provider.get_transaction_by_hash(transaction_hash).await?;

        match transaction {
            Some(tx) => {
                let block_number = tx.block_number;
                let is_transaction_incoming = self.is_transaction_incoming(tx.to()).await?;
                let value = tx.value();

                let date = match block_number {
                    Some(b) => self.fetch_block_date(b.into()).await?,
                    None => None,
                };

                let is_transaction_successful = self.verify_transaction_success(transaction_hash).await?;

                let status = self::WalletImplEth::map_transaction_success_to_inclusion_state(is_transaction_successful);

                let balance_wei_crypto_amount = Self::convert_alloy_256_to_crypto_amount(tx.value())?;
                let value_eth_crypto_amount = Self::convert_wei_to_eth(balance_wei_crypto_amount);

                let value_eth_f64: f64 = value_eth_crypto_amount.inner().try_into()?; // TODO: WalletTxInfo f64 -> Decimal ? maybe

                Ok(WalletTxInfo {
                    date: date.map(|n| n.to_string()).unwrap_or_else(String::new),
                    block_id: block_number.map(|n| n.to_string()),
                    transaction_id: transaction_id.to_string(),
                    incoming: is_transaction_incoming,
                    amount: value_eth_f64,
                    network: "ETH".to_string(),
                    status: format!("{:?}", status),
                    explorer_url: Some(self.node_url.clone()),
                })
            }
            None => Err(WalletError::TransactionNotFound),
        }
    }

    async fn estimate_gas_cost_eip1559(&self, transaction: TxEip1559) -> Result<GasCostEstimation> {
        let from = self.get_address().await?;

        let to = transaction
            .to
            .to()
            .ok_or(WalletError::InvalidTransaction(String::from("receiver is empty")))?;

        let tx = TransactionRequest::default()
            .from(Address::from_str(&from)?)
            .to(*to)
            .input(TransactionInput::new(transaction.input))
            .access_list(transaction.access_list)
            .value(transaction.value);

        // Returns the estimated gas cost for the underlying transaction to be executed
        let gas_limit = self.http_provider.estimate_gas(&tx).await?;

        // Estimates the EIP1559 `maxFeePerGas` and `maxPriorityFeePerGas` fields in wei.
        let eip1559_estimation = self.http_provider.estimate_eip1559_fees(None).await?;

        let max_priority_fee_per_gas = eip1559_estimation.max_priority_fee_per_gas;
        let max_fee_per_gas = eip1559_estimation.max_fee_per_gas;

        Ok(GasCostEstimation {
            max_fee_per_gas,
            max_priority_fee_per_gas,
            gas_limit,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::core::Config;

    use super::*;
    use alloy::{hex::decode, primitives::Address};
    use iota_sdk::crypto::keys::bip39::Mnemonic;
    use rust_decimal::prelude::FromPrimitive;
    use serde_json::json;
    use testing::CleanUp;

    // Dummy address
    pub const RECEIVER_ADDR_RAW: &str = "0xb0b0000000000000000000000000000000000000";

    // Account #0: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 (10000 ETH)
    // Private Key: 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
    pub const HARDHAT_MNEMONIC: &str = "test test test test test test test test test test test junk";

    /// helper function to get a [`WalletUser`] instance.
    async fn get_wallet_user(mnemonic: impl Into<Mnemonic>) -> (WalletImplEth, CleanUp) {
        let (_, cleanup) = Config::new_test_with_cleanup();
        let node_url = String::from("https://sepolia.mode.network");
        let chain_id = 31337;

        let wallet = WalletImplEth::new(mnemonic.into(), Path::new(&cleanup.path_prefix), node_url, chain_id)
            .await
            .expect("should initialize wallet");
        (wallet, cleanup)
    }

    /// helper function to get a [`WalletUser`] instance.
    async fn get_wallet_user_with_mocked_provider(
        mnemonic: impl Into<Mnemonic>,
        http_provider: RootProvider<Http<Client>>,
        node_url: String,
        chain_id: u64,
    ) -> (WalletImplEth, CleanUp) {
        let (_, cleanup) = Config::new_test_with_cleanup();
        let wallet = WalletImplEth::new_with_mocked_provider(
            mnemonic.into(),
            Path::new(&cleanup.path_prefix),
            http_provider,
            node_url,
            chain_id,
        )
        .await
        .expect("should initialize wallet");

        (wallet, cleanup)
    }

    #[tokio::test]
    async fn test_get_address() {
        //Arrange
        let (wallet_user, _cleanup) = get_wallet_user(HARDHAT_MNEMONIC).await;

        // Act
        let addr_raw = wallet_user.get_address().await.unwrap();

        // Assert
        assert!(addr_raw.starts_with("0x"));
    }

    #[tokio::test]
    async fn test_convert_wei_to_decimal() {
        //Arrange
        let value_wei: i128 = 10000000000000000000000; // 22 zeros
        let value_wei_decimal = Decimal::from_i128(value_wei).unwrap();
        // SAFETY: the value is non-negative
        let value_wei_crypto_amount: CryptoAmount = unsafe { CryptoAmount::new_unchecked(value_wei_decimal) };

        // Act
        let value_eth_crypto_amount = WalletImplEth::convert_wei_to_eth(value_wei_crypto_amount);

        // Assert

        /***
         * 1 10^22
         * 10 10^21
         * ...
         * 10000 10^18
         */
        assert_eq!(value_eth_crypto_amount, CryptoAmount::from(10000))
    }

    #[tokio::test]
    async fn test_convert_alloy_256_to_decimal() {
        //Arrange
        let expected_value_decimal: i128 = 10000000000000000000000;
        let expected_value_decimal = Decimal::from_i128(expected_value_decimal).unwrap();
        // SAFETY: the value is non-negative
        let expected_value_crypto_amount: CryptoAmount = unsafe { CryptoAmount::new_unchecked(expected_value_decimal) };

        let value_alloy_256: alloy_primitives::Uint<256, 4> =
            alloy_primitives::Uint::from_str("10000000000000000000000").unwrap();

        // Act
        let value_crypto_amount = WalletImplEth::convert_alloy_256_to_crypto_amount(value_alloy_256).unwrap();

        // Assert
        assert_eq!(value_crypto_amount, expected_value_crypto_amount)
    }

    #[tokio::test]
    async fn test_convert_crypto_amount_to_alloy_256() {
        //Arrange
        let expected_value_u256 = U256::from(1);
        let value_crypto_amount = CryptoAmount::from(1);

        // Act
        let value_u256 = WalletImplEth::convert_crypto_amount_to_u256(value_crypto_amount).unwrap();

        // Assert
        assert_eq!(value_u256, expected_value_u256)
    }

    #[tokio::test]
    async fn test_get_balance() {
        //Arrange
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let node_url = Url::parse(&url).unwrap();
        let mockito_http_provider = ProviderBuilder::new().on_http(node_url.clone());
        let chain_id = 31337;

        let (wallet_user, _cleanup) = get_wallet_user_with_mocked_provider(
            HARDHAT_MNEMONIC,
            mockito_http_provider,
            node_url.to_string(),
            chain_id,
        )
        .await;

        let wallet_addr = wallet_user.get_wallet_addr().await.unwrap();

        let mocked_balance: i128 = 10_000_000_000_000_000_000_000; // 10^22
        let mocked_balance_in_hex = format!("0x{:X}", mocked_balance);

        let mocked_rpc_get_balance = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "method": "eth_getBalance",
                "params": [
                    wallet_addr,
                    "latest"
                ],
            })))
            .with_status(200)
            .with_body(format!(
                r#"{{
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": "{}"
                }}"#,
                mocked_balance_in_hex
            ))
            .create();

        // Act
        let balance = wallet_user.get_balance().await.unwrap();

        mocked_rpc_get_balance.assert();

        // Assert
        assert_eq!(balance, CryptoAmount::from(10000))
    }

    #[tokio::test]
    async fn test_get_next_nonce() {
        //Arrange
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let node_url = Url::parse(&url).unwrap();
        let mockito_http_provider = ProviderBuilder::new().on_http(node_url.clone());
        let chain_id = 31337;

        let (wallet_user, _cleanup) = get_wallet_user_with_mocked_provider(
            HARDHAT_MNEMONIC,
            mockito_http_provider,
            node_url.to_string(),
            chain_id,
        )
        .await;

        let wallet_addr = wallet_user.get_wallet_addr().await.unwrap();
        let mocked_transaction_count = 5;

        let mocked_rpc_get_transaction_count = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "method": "eth_getTransactionCount",
                "params": [
                    wallet_addr,
                    "latest"
                ],
            })))
            .with_status(200)
            .with_body(format!(
                r#"{{
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": "{}"
                }}"#,
                mocked_transaction_count
            ))
            .create();

        // Act
        let nonce = wallet_user.get_next_nonce(wallet_addr).await.unwrap();
        mocked_rpc_get_transaction_count.assert();

        // Assert
        assert_eq!(nonce, mocked_transaction_count)
    }

    #[tokio::test]
    async fn test_build_transaction() {
        //Arrange
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let node_url = Url::parse(&url).unwrap();
        let mockito_http_provider = ProviderBuilder::new().on_http(node_url.clone());
        let chain_id = 31337;

        let (wallet_user, _cleanup) = get_wallet_user_with_mocked_provider(
            HARDHAT_MNEMONIC,
            mockito_http_provider,
            node_url.to_string(),
            chain_id,
        )
        .await;

        let wallet_addr = wallet_user.get_wallet_addr().await.unwrap();
        let mocked_transaction_count = 5;

        let mocked_rpc_get_transaction_count = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "method": "eth_getTransactionCount",
                "params": [
                    wallet_addr,
                    "latest"
                ],
            })))
            .with_status(200)
            .with_body(format!(
                r#"{{
                        "jsonrpc": "2.0",
                        "id": 1,
                        "result": "{}"
                    }}"#,
                mocked_transaction_count
            ))
            .create();

        let addr_from = wallet_addr;
        let addr_to = Address::from_str(RECEIVER_ADDR_RAW).unwrap();
        let amount_to_send = U256::from(1);
        let gas_limit = 21_000;
        let max_fee_per_gas = 20_000_000_000;
        let max_priority_fee_per_gas = 1_000_000_000;

        // Act
        let transaction = wallet_user
            .build_transaction(
                addr_from,
                addr_to,
                amount_to_send,
                gas_limit,
                max_fee_per_gas,
                max_priority_fee_per_gas,
                wallet_user.chain_id,
                None,
                None,
            )
            .await;

        // Assert
        mocked_rpc_get_transaction_count.assert();
        transaction.unwrap();
    }

    #[tokio::test]
    async fn test_build_transaction_with_tag_and_metadata() {
        //Arrange
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let node_url = Url::parse(&url).unwrap();
        let mockito_http_provider = ProviderBuilder::new().on_http(node_url.clone());
        let chain_id = 31337;

        let (wallet_user, _cleanup) = get_wallet_user_with_mocked_provider(
            HARDHAT_MNEMONIC,
            mockito_http_provider,
            node_url.to_string(),
            chain_id,
        )
        .await;

        let wallet_addr = wallet_user.get_wallet_addr().await.unwrap();
        let mocked_transaction_count = 5;

        let mocked_rpc_get_transaction_count = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "method": "eth_getTransactionCount",
                "params": [
                    wallet_addr,
                    "latest"
                ],
            })))
            .with_status(200)
            .with_body(format!(
                r#"{{
                        "jsonrpc": "2.0",
                        "id": 1,
                        "result": "{}"
                    }}"#,
                mocked_transaction_count
            ))
            .create();

        let addr_from = wallet_addr;
        let addr_to = Address::from_str(RECEIVER_ADDR_RAW).unwrap();
        let amount_to_send = U256::from(1);
        let gas_limit = 21_000;
        let max_fee_per_gas = 20_000_000_000;
        let max_priority_fee_per_gas = 1_000_000_000;
        let tag = TaggedDataPayload::new(Vec::from([8, 16]), Vec::from([8, 16])).unwrap();
        let metadata = String::from("test message");

        // Act
        let transaction = wallet_user
            .build_transaction(
                addr_from,
                addr_to,
                amount_to_send,
                gas_limit,
                max_fee_per_gas,
                max_priority_fee_per_gas,
                wallet_user.chain_id,
                Some(tag),
                Some(metadata),
            )
            .await;

        // Assert
        mocked_rpc_get_transaction_count.assert();
        transaction.unwrap();
    }

    #[tokio::test]
    async fn test_sign_transaction() {
        //Arrange
        let (wallet_user, _cleanup) = get_wallet_user(HARDHAT_MNEMONIC).await;

        let transaction_to_sign = TxEip1559 {
            chain_id: 31337,
            nonce: 0,
            gas_limit: 21_000,
            max_fee_per_gas: 20_000,
            max_priority_fee_per_gas: 1_000,
            to: alloy_primitives::TxKind::Call(Address::from_str(RECEIVER_ADDR_RAW).unwrap()),
            value: U256::from(100),
            access_list: Default::default(),
            input: Default::default(),
        };

        let wallet_addr_raw = wallet_user.get_wallet_addr_raw().await.unwrap();

        // Act
        let signed_transaction = wallet_user.sign_transaction(transaction_to_sign, wallet_addr_raw).await;

        // Assert
        signed_transaction.unwrap();
    }

    #[tokio::test]
    async fn test_send_transaction_eth() {
        //Arrange
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let node_url = Url::parse(&url).unwrap();
        let mockito_http_provider = ProviderBuilder::new().on_http(node_url.clone());
        let chain_id = 31337;

        let (wallet_user, _cleanup) = get_wallet_user_with_mocked_provider(
            HARDHAT_MNEMONIC,
            mockito_http_provider,
            node_url.to_string(),
            chain_id,
        )
        .await;

        let mocked_transaction_count = 5;
        let index = "1";
        let amount_to_send = CryptoAmount::from(100);
        let from = wallet_user.get_address().await.unwrap();
        let to = String::from("0xb0b0000000000000000000000000000000000000");

        let mocked_rpc_get_transaction_count = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "method": "eth_getTransactionCount",
                "params": [
                    from,
                    "latest"
                ],
            })))
            .with_status(200)
            .with_body(format!(
                r#"{{
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": "{}"
                }}"#,
                mocked_transaction_count
            ))
            .create();

        let mocked_rpc_estimate_gas = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "eth_estimateGas",
                "params": [{"from":format!("{}", from),"to":format!("{}", to),"value":format!("{}", "0x56bc75e2d63100000"),"input":"0x","accessList":[]},"pending"],
            })))
            .with_status(200)
            .with_body(
                r#"{
                    "jsonrpc": "2.0",
                    "result": 24009
                }"#,
            )
            .create();

        let mocked_rpc_eth_fee_history = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "method": "eth_feeHistory",
            })))
            .with_status(200)
            .with_body(
                r#"{
                "jsonrpc": "2.0",
                "result": {
                    "baseFeePerGas": [1000000000, 875000000],
                    "gasUsedRatio": [0.0],
                    "oldestBlock": 0,
                    "reward": [[0]]
                }
            }
            "#,
            )
            .create();

        let mocked_transaction_hash = "0x969dc1d6a97464e62fb1dab451b03d24111c278bf6f4d2e2b3910205a8682ed2";
        let mocked_rpc_send_raw_transaction = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "method": "eth_sendRawTransaction",
                "id": 3,
                "params": [
                    "0x02f871827a6905018477359401825dc994b0b000000000000000000000000000000000000089056bc75e2d6310000080c001a0b97a8495837eb1ade4b01d5e1789e66927aacaaa2e62ba3d8ab844d575187573a07cca5b3851847758c1bbf70d7682ce2ab82e53f9b1b56202c57f71c50971e5ee"
                ],
            })))
            .with_status(200)
            .with_body(format!(
                r#"{{
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": "{}"
                }}"#,
                mocked_transaction_hash
            ))
            .create();

        // Act
        let transaction_id = wallet_user.send_transaction_eth(index, &to, amount_to_send).await;

        // Assert
        mocked_rpc_get_transaction_count.assert();
        mocked_rpc_estimate_gas.assert();
        mocked_rpc_eth_fee_history.assert();
        mocked_rpc_send_raw_transaction.assert();
        transaction_id.unwrap();
    }

    #[tokio::test]
    async fn test_send_amount_eth() {
        //Arrange
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let node_url = Url::parse(&url).unwrap();
        let mockito_http_provider = ProviderBuilder::new().on_http(node_url.clone());
        let chain_id = 31337;

        let (wallet_user, _cleanup) = get_wallet_user_with_mocked_provider(
            HARDHAT_MNEMONIC,
            mockito_http_provider,
            node_url.to_string(),
            chain_id,
        )
        .await;

        let mocked_transaction_count = 5;
        let amount_to_send = CryptoAmount::from(100);
        let from = wallet_user.get_address().await.unwrap();
        let to = String::from("0xb0b0000000000000000000000000000000000000");

        let mocked_rpc_get_transaction_count = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "method": "eth_getTransactionCount",
                "params": [
                    from,
                    "latest"
                ],
            })))
            .with_status(200)
            .with_body(format!(
                r#"{{
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": "{}"
                }}"#,
                mocked_transaction_count
            ))
            .create();

        let mocked_rpc_estimate_gas = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "eth_estimateGas",
                "params": [{"from":format!("{}", from),"to":format!("{}", to),"value":format!("{}", "0x56bc75e2d63100000"),"input":"0x37623232373436313637323233613562333832633331333635643263323236343631373436313232336135623338326333313336356432633232366436353733373336313637363532323361323237343635373337343230366436353733373336313637363532323764","accessList":[]},"pending"],
            })))
            .with_status(200)
            .with_body(
                r#"{
                    "jsonrpc": "2.0",
                    "result": 24009
                }"#,
            )
            .create();

        let mocked_rpc_eth_fee_history = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "method": "eth_feeHistory",
            })))
            .with_status(200)
            .with_body(
                r#"{
                "jsonrpc": "2.0",
                "result": {
                    "baseFeePerGas": [1000000000, 875000000],
                    "gasUsedRatio": [0.0],
                    "oldestBlock": 0,
                    "reward": [[0]]
                }
            }
            "#,
            )
            .create();

        let mocked_transaction_hash = "0x969dc1d6a97464e62fb1dab451b03d24111c278bf6f4d2e2b3910205a8682ed2";
        let mocked_rpc_send_raw_transaction = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "method": "eth_sendRawTransaction",
                "id": 3,
                "params": [
                    "0x02f8dc827a6905018477359401825dc994b0b000000000000000000000000000000000000089056bc75e2d63100000b86a37623232373436313637323233613562333832633331333635643263323236343631373436313232336135623338326333313336356432633232366436353733373336313637363532323361323237343635373337343230366436353733373336313637363532323764c080a014837e78ab7f4b04f358ecca39638934afd707e85f038d3d1333e556936215aea04b8bb1b31609fd14e56ea73df3fcdd695bd6122f8c60ab8412b2418613ee92db"
                ],
            })))
            .with_status(200)
            .with_body(format!(
                r#"{{
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": "{}"
                }}"#,
                mocked_transaction_hash
            ))
            .create();

        let tagged_data_payload = TaggedDataPayload::new(Vec::from([8, 16]), Vec::from([8, 16])).unwrap();
        let metadata = String::from("test message");

        // Act
        let transaction_id = wallet_user
            .send_amount_eth(&to, amount_to_send, Some(tagged_data_payload), Some(metadata))
            .await;

        // Assert
        mocked_rpc_get_transaction_count.assert();
        mocked_rpc_estimate_gas.assert();
        mocked_rpc_eth_fee_history.assert();
        mocked_rpc_send_raw_transaction.assert();
        transaction_id.unwrap();
    }

    #[tokio::test]
    async fn test_get_wallet_tx_returns_error_when_transaction_cannot_be_found() {
        //Arrange
        let non_existent_transaction_hash = "0x00000000000000000000000000000000000000000000000000000000000e4404";

        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let node_url = Url::parse(&url).unwrap();
        let mockito_http_provider = ProviderBuilder::new().on_http(node_url.clone());
        let chain_id = 31337;

        let (wallet_user, _cleanup) = get_wallet_user_with_mocked_provider(
            HARDHAT_MNEMONIC,
            mockito_http_provider,
            node_url.to_string(),
            chain_id,
        )
        .await;

        let mocked_rpc_get_transaction_by_hash = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "method": "eth_getTransactionByHash",
                "params": [
                    non_existent_transaction_hash,
                ],
            })))
            .with_status(200)
            .with_body(
                r#"{
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": null
                }"#,
            )
            .create();

        // Act
        let transaction = wallet_user.get_wallet_tx(non_existent_transaction_hash).await;

        // Assert
        mocked_rpc_get_transaction_by_hash.assert();
        let error = transaction.unwrap_err();
        assert!(matches!(error, WalletError::TransactionNotFound))
    }

    #[tokio::test]
    async fn test_get_wallet_tx() {
        //Arrange
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let node_url = Url::parse(&url).unwrap();
        let mockito_http_provider = ProviderBuilder::new().on_http(node_url.clone());
        let chain_id = 31337;

        let (wallet_user, _cleanup) = get_wallet_user_with_mocked_provider(
            HARDHAT_MNEMONIC,
            mockito_http_provider,
            node_url.to_string(),
            chain_id,
        )
        .await;

        let dummy_transaction_hash = "0xcd718a69d478340dc28fdf6bf8056374a52dc95841b44083163ced8dfe29310c";
        let dummy_block_number = "0x107d7b0";

        let mocked_rpc_get_transaction_by_hash_response_json = json!({
            "id": "1",
            "jsonrpc": "2.0",
            "result": {
                "accessList": [],
                "blockHash": "0xe6262c1924326d12b88aaa35a95a0c7cdd11f2d20ebae84618484120bd037c34",
                "blockNumber": "0x107d7b0",
                "chainId": "0x1",
                "from": "0x901c7c311d39e0b26257219765e71e8db3107a81",
                "gas": "0x31d74",
                "gasPrice": "0xb9029a7ea",
                "hash": "0xcd718a69d478340dc28fdf6bf8056374a52dc95841b44083163ced8dfe29310c",
                "input": "0x3593564c000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000646701fb000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000a968163f0a57b400000000000000000000000000000000000000000000000000000000000010326d79400000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000002bb7135877cd5d40aa3b086ac6f21c51bbafbbb41f002710dac17f958d2ee523a2206206994597c13d831ec7000000000000000000000000000000000000000000",
                "maxFeePerGas": "0xf22a22912",
                "maxPriorityFeePerGas": "0x5f5e100",
                "nonce": "0x4",
                "r": "0xef566fc229bb0a10eee5f99c9cabe47f0f20ebaa6d16e4f7b90ee144086b21e9",
                "s": "0x109de5d9baca8daeee1ce1b7d1a304e223d07b1420b37704e675ccffd364a4dc",
                "to": "0xef1c6e67703c7bd7107eed8303fbe6ec2554bf6b",
                "transactionIndex": "0xfc",
                "type": "0x2",
                "v": "0x0",
                "value": "0x0"
            }
        });

        let mocked_rpc_get_transaction_by_hash = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "method": "eth_getTransactionByHash",
                "params": [
                    dummy_transaction_hash,
                ],
            })))
            .with_status(200)
            .with_body(serde_json::to_vec(&mocked_rpc_get_transaction_by_hash_response_json).unwrap())
            .create();

        let mocked_rpc_get_block_by_number_response_json = json!({
            "id": "1",
            "jsonrpc": "2.0",
            "result": {
                "difficulty": "0x1046bb7e3f8",
                "extraData": "0x476574682f76312e302e302d30636463373634372f6c696e75782f676f312e34",
                "gasLimit": "0x1388",
                "gasUsed": "0x0",
                "hash": "0xf7756d836b6716aaeffc2139c032752ba5acf02fe94acb65743f0d177554b2e2",
                "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
                "miner": "0x33bc13fdf135073277971b4d9f4f72082e907996",
                "mixHash": "0x8c2dc0f970fa3aa6beb64c9f06a202a4314acfa4effaa4c75fd5bc9f9c77a519",
                "nonce": "0x28df43dd283aab1d",
                "number": "0x107d7b0",
                "parentHash": "0xbc33aa8829350cc2e3ba7cf64d4beb2f1b554d570efc8bccb7b05ef50d76a47a",
                "receiptsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
                "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
                "size": "0x223",
                "stateRoot": "0x8af5429b649f9fc633ce3c95219026fd08a249867e28c7eab22994eaa6125bb9",
                "timestamp": "0x55bf47e3",
                "totalDifficulty": "0x3f3cfd84833af0",
                "transactions": [],
                "transactionsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
                "uncles": []
            }
        });

        let mocked_rpc_get_block_by_number = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "method": "eth_getBlockByNumber",
                "params": [
                    dummy_block_number,
                    true
                ],
            })))
            .with_status(200)
            .with_body(serde_json::to_vec(&mocked_rpc_get_block_by_number_response_json).unwrap())
            .create();

        let mocked_rpc_get_transaction_receipt_response_json = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "blockHash": "0xe6262c1924326d12b88aaa35a95a0c7cdd11f2d20ebae84618484120bd037c34",
                "blockNumber": "0x107d7b0",
                "contractAddress": null,
                "cumulativeGasUsed": "0x19aac9a",
                "effectiveGasPrice": "0xb9029a7ea",
                "from": "0x901c7c311d39e0b26257219765e71e8db3107a81",
                "gasUsed": "0x27fb4",
                "logs": [],
                "logsBloom": "0x00000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000010000000000800020000000000000200000000040000000000800104008000000000000000000000800000000001200000000000000000000000000000000000000000000000020000000020010000800000000000000000000000000000000000000000000000000000000000000120000020800000000000000100084000000000000000000000000000000000000000000000002000000000000001000000000400000000000000000000000000000000010000000000000000000000000000000000000000000000000000090000000",
                "status": "0x1",
                "to": "0xef1c6e67703c7bd7107eed8303fbe6ec2554bf6b",
                "transactionHash": "0xcd718a69d478340dc28fdf6bf8056374a52dc95841b44083163ced8dfe29310c",
                "transactionIndex": "0xfc",
                "type": "0x2"
            }
        });

        let mocked_rpc_get_transaction_receipt = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "method": "eth_getTransactionReceipt",
                "params": [
                    dummy_transaction_hash
                ],
            })))
            .with_status(200)
            .with_body(serde_json::to_vec(&mocked_rpc_get_transaction_receipt_response_json).unwrap())
            .create();

        // Act
        let transaction = wallet_user.get_wallet_tx(dummy_transaction_hash).await;

        // Assert
        mocked_rpc_get_transaction_by_hash.assert();
        mocked_rpc_get_transaction_receipt.assert();
        mocked_rpc_get_block_by_number.assert();
        transaction.unwrap();
    }

    #[tokio::test]
    async fn test_verify_transaction_success() {
        //Arrange
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let node_url = Url::parse(&url).unwrap();
        let mockito_http_provider = ProviderBuilder::new().on_http(node_url.clone());
        let chain_id = 31337;

        let (wallet_user, _cleanup) = get_wallet_user_with_mocked_provider(
            HARDHAT_MNEMONIC,
            mockito_http_provider,
            node_url.to_string(),
            chain_id,
        )
        .await;

        let dummy_transaction_id = "0xcd718a69d478340dc28fdf6bf8056374a52dc95841b44083163ced8dfe29310c";

        let mocked_rpc_get_transaction_receipt_response_json = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "blockHash": "0xe6262c1924326d12b88aaa35a95a0c7cdd11f2d20ebae84618484120bd037c34",
                "blockNumber": "0x107d7b0",
                "contractAddress": null,
                "cumulativeGasUsed": "0x19aac9a",
                "effectiveGasPrice": "0xb9029a7ea",
                "from": "0x901c7c311d39e0b26257219765e71e8db3107a81",
                "gasUsed": "0x27fb4",
                "logs": [
                    {
                        "address": "0xdac17f958d2ee523a2206206994597c13d831ec7",
                        "blockHash": "0xe6262c1924326d12b88aaa35a95a0c7cdd11f2d20ebae84618484120bd037c34",
                        "blockNumber": "0x107d7b0",
                        "data": "0x0000000000000000000000000000000000000000000000000000000103f1bfef",
                        "logIndex": "0x24e",
                        "removed": false,
                        "topics": [
                            "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
                            "0x000000000000000000000000a82f91562e1cef9dec93a4ad328d01ea7827910a",
                            "0x000000000000000000000000901c7c311d39e0b26257219765e71e8db3107a81"
                        ],
                        "transactionHash": "0xcd718a69d478340dc28fdf6bf8056374a52dc95841b44083163ced8dfe29310c",
                        "transactionIndex": "0xfc"
                    },
                    {
                        "address": "0xb7135877cd5d40aa3b086ac6f21c51bbafbbb41f",
                        "blockHash": "0xe6262c1924326d12b88aaa35a95a0c7cdd11f2d20ebae84618484120bd037c34",
                        "blockNumber": "0x107d7b0",
                        "data": "0x00000000000000000000000000000000000000000003f6526b99745385c00000",
                        "logIndex": "0x24f",
                        "removed": false,
                        "topics": [
                            "0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925",
                            "0x000000000000000000000000901c7c311d39e0b26257219765e71e8db3107a81",
                            "0x000000000000000000000000000000000022d473030f116ddee9f6b43ac78ba3"
                        ],
                        "transactionHash": "0xcd718a69d478340dc28fdf6bf8056374a52dc95841b44083163ced8dfe29310c",
                        "transactionIndex": "0xfc"
                    },
                    {
                        "address": "0xb7135877cd5d40aa3b086ac6f21c51bbafbbb41f",
                        "blockHash": "0xe6262c1924326d12b88aaa35a95a0c7cdd11f2d20ebae84618484120bd037c34",
                        "blockNumber": "0x107d7b0",
                        "data": "0x000000000000000000000000000000000000000000000a968163f0a57b400000",
                        "logIndex": "0x250",
                        "removed": false,
                        "topics": [
                            "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
                            "0x000000000000000000000000901c7c311d39e0b26257219765e71e8db3107a81",
                            "0x000000000000000000000000a82f91562e1cef9dec93a4ad328d01ea7827910a"
                        ],
                        "transactionHash": "0xcd718a69d478340dc28fdf6bf8056374a52dc95841b44083163ced8dfe29310c",
                        "transactionIndex": "0xfc"
                    },
                    {
                        "address": "0xa82f91562e1cef9dec93a4ad328d01ea7827910a",
                        "blockHash": "0xe6262c1924326d12b88aaa35a95a0c7cdd11f2d20ebae84618484120bd037c34",
                        "blockNumber": "0x107d7b0",
                        "data": "0x000000000000000000000000000000000000000000000a968163f0a57b400000fffffffffffffffffffffffffffffffffffffffffffffffffffffffefc0e40110000000000000000000000000000000000000000000004f77993b72687d0b8d40000000000000000000000000000000000000000000000002672ab0fa51842cafffffffffffffffffffffffffffffffffffffffffffffffffffffffffffb6981",
                        "logIndex": "0x251",
                        "removed": false,
                        "topics": [
                            "0xc42079f94a6350d7e6235f29174924f928cc2ac818eb64fed8004e115fbcca67",
                            "0x000000000000000000000000ef1c6e67703c7bd7107eed8303fbe6ec2554bf6b",
                            "0x000000000000000000000000901c7c311d39e0b26257219765e71e8db3107a81"
                        ],
                        "transactionHash": "0xcd718a69d478340dc28fdf6bf8056374a52dc95841b44083163ced8dfe29310c",
                        "transactionIndex": "0xfc"
                    }
                ],
                "logsBloom": "0x00000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000010000000000800020000000000000200000000040000000000800104008000000000000000000000800000000001200000000000000000000000000000000000000000000000020000000020010000800000000000000000000000000000000000000000000000000000000000000120000020800000000000000100084000000000000000000000000000000000000000000000002000000000000001000000000400000000000000000000000000000000010000000000000000000000000000000000000000000000000000090000000",
                "status": "0x1",
                "to": "0xef1c6e67703c7bd7107eed8303fbe6ec2554bf6b",
                "transactionHash": "0xcd718a69d478340dc28fdf6bf8056374a52dc95841b44083163ced8dfe29310c",
                "transactionIndex": "0xfc",
                "type": "0x2"
            }
        });

        let mocked_rpc_get_transaction_receipt = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "method": "eth_getTransactionReceipt",
                "params": [
                    dummy_transaction_id
                ],
            })))
            .with_status(200)
            .with_body(serde_json::to_vec(&mocked_rpc_get_transaction_receipt_response_json).unwrap())
            .create();

        let transaction_hash = TxHash::from_str(dummy_transaction_id).unwrap();

        // Act
        let is_transaction_successful = wallet_user.verify_transaction_success(transaction_hash).await;

        // Assert
        mocked_rpc_get_transaction_receipt.assert();
        is_transaction_successful.unwrap();
    }

    #[tokio::test]
    async fn test_is_transaction_incoming_returns_true_if_the_transaction_receiver_is_our_wallet_address() {
        //Arrange
        let (wallet_user, _cleanup) = get_wallet_user(HARDHAT_MNEMONIC).await;

        let wallet_addr_raw = wallet_user.get_wallet_addr_raw().await.unwrap();
        let receiver_addr = Address::from_str(&wallet_addr_raw).unwrap();

        // Act
        let is_transaction_incoming = wallet_user.is_transaction_incoming(Some(receiver_addr)).await.unwrap();

        // Assert
        assert!(is_transaction_incoming)
    }

    #[tokio::test]
    async fn test_is_transaction_incoming_returns_false_if_the_transaction_receiver_is_not_our_wallet_address() {
        //Arrange
        let (wallet_user, _cleanup) = get_wallet_user(HARDHAT_MNEMONIC).await;

        let receiver_addr = Address::from_str(RECEIVER_ADDR_RAW).unwrap();

        // Act
        let is_transaction_incoming = wallet_user.is_transaction_incoming(Some(receiver_addr)).await.unwrap();

        // Assert
        assert!(!is_transaction_incoming)
    }

    #[tokio::test]
    async fn test_compare_addresses_returns_true_when_addresses_are_the_same() {
        //Arrange
        let (wallet_user, _cleanup) = get_wallet_user(HARDHAT_MNEMONIC).await;

        let wallet_addr_raw = wallet_user.get_wallet_addr_raw().await.unwrap();
        let wallet_addr = Address::from_str(&wallet_addr_raw).unwrap();
        let receiver_addr = Address::from_str(&wallet_addr_raw).unwrap();

        // Act
        let result = WalletImplEth::compare_addresses(wallet_addr, Some(receiver_addr));

        // Assert
        assert!(result)
    }

    #[tokio::test]
    async fn test_compare_addresses_returns_false_when_addresses_are_different() {
        //Arrange
        let (wallet_user, _cleanup) = get_wallet_user(HARDHAT_MNEMONIC).await;

        let wallet_addr = wallet_user.get_wallet_addr().await.unwrap();
        let receiver_addr = Address::from_str(RECEIVER_ADDR_RAW).unwrap();

        // Act
        let result = WalletImplEth::compare_addresses(wallet_addr, Some(receiver_addr));

        // Assert
        assert!(!result)
    }

    #[tokio::test]
    async fn test_compare_addresses_returns_false_when_receiver_addr_is_none() {
        //Arrange
        let (wallet_user, _cleanup) = get_wallet_user(HARDHAT_MNEMONIC).await;

        let wallet_addr_raw = wallet_user.get_wallet_addr_raw().await.unwrap();
        let wallet_addr = Address::from_str(&wallet_addr_raw).unwrap();
        let receiver_addr = None;

        // Act
        let result = WalletImplEth::compare_addresses(wallet_addr, receiver_addr);

        // Assert
        assert!(!result)
    }

    #[tokio::test]
    async fn test_serialize_and_deserialize_unified_transaction_metadata() {
        // Arrange
        let readable_tag = "temperature";
        let readable_data = "20";
        let readable_metadata = "still too warm".to_string();

        let unified_transaction_metadata = UnifiedTransactionMetadata {
            tag: Some(readable_tag.as_bytes().to_vec()),
            data: Some(readable_data.as_bytes().to_vec()),
            message: Some(readable_metadata),
        };

        // Act
        let serialized_unified_transaction_metadata = serde_json::to_string(&unified_transaction_metadata).unwrap();
        let encoded_unified_transaction_metadata = encode(serialized_unified_transaction_metadata);
        let decoded_unified_transaction_metadata = decode(encoded_unified_transaction_metadata).unwrap();

        let decoded_unified_transaction_metadata = String::from_utf8(decoded_unified_transaction_metadata).unwrap();
        let deserialized_unified_transaction_metadata: UnifiedTransactionMetadata =
            serde_json::from_str(&decoded_unified_transaction_metadata).unwrap();

        // Assert
        assert_eq!(unified_transaction_metadata, deserialized_unified_transaction_metadata)
    }

    #[tokio::test]
    async fn should_estimate_gas_cost_eip1559() {
        // Arrange
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let node_url = Url::parse(&url).unwrap();
        let mockito_http_provider = ProviderBuilder::new().on_http(node_url.clone());
        let chain_id = 31337;

        let (wallet_user, _cleanup) = get_wallet_user_with_mocked_provider(
            HARDHAT_MNEMONIC,
            mockito_http_provider,
            node_url.to_string(),
            chain_id,
        )
        .await;

        let to = String::from("0xb0b0000000000000000000000000000000000000");
        let value = U256::from(1);
        let chain_id = 31337;

        let readable_tag = "temperature";
        let readable_data = "20";
        let readable_metadata = "still too warm".to_string();

        let unified_transaction_metadata = UnifiedTransactionMetadata {
            tag: Some(readable_tag.as_bytes().to_vec()),
            data: Some(readable_data.as_bytes().to_vec()),
            message: Some(readable_metadata),
        };

        let serialized_unified_transaction_metadata = serde_json::to_string(&unified_transaction_metadata).unwrap();
        let encoded_unified_transaction_metadata = encode(serialized_unified_transaction_metadata);

        let tx = TxEip1559 {
            chain_id,
            nonce: 0,
            gas_limit: 0,
            max_fee_per_gas: 0,
            max_priority_fee_per_gas: 0,
            to: alloy_primitives::TxKind::Call(Address::from_str(&to).unwrap()),
            value,
            access_list: Default::default(),
            input: encoded_unified_transaction_metadata.into(),
        };

        let expected_estimation = GasCostEstimation {
            gas_limit: 24009,
            max_fee_per_gas: 2000000001,
            max_priority_fee_per_gas: 1,
        };

        let from = wallet_user.get_address().await.unwrap();
        let mocked_rpc_estimate_gas = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "id": 0,
                "method": "eth_estimateGas",
                "params": [{"from":format!("{}", from),"to":format!("{}", to),"value":format!("{}", "0x1")},"pending"],
            })))
            .with_status(200)
            .with_body(
                r#"{
                    "jsonrpc": "2.0",
                    "result": 24009
                }"#,
            )
            .create();

        let mocked_rpc_eth_fee_history = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "method": "eth_feeHistory",
            })))
            .with_status(200)
            .with_body(
                r#"{
                "jsonrpc": "2.0",
                "result": {
                    "baseFeePerGas": [1000000000, 875000000],
                    "gasUsedRatio": [0.0],
                    "oldestBlock": 0,
                    "reward": [[0]]
                }
            }
            "#,
            )
            .create();

        // Act
        let result = wallet_user.estimate_gas_cost_eip1559(tx).await;

        // Assert
        mocked_rpc_estimate_gas.assert();
        mocked_rpc_eth_fee_history.assert();
        let response = result.unwrap();

        assert_eq!(expected_estimation, response)
    }
}
