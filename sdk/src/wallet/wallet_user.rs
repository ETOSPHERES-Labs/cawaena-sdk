use super::error::{Result, WalletError};
use crate::types::currencies::{CryptoAmount, Currency};
use crate::types::transactions::{GasCostEstimation, WalletTxInfo, WalletTxInfoList};
use alloy_consensus::TxEip1559;
use async_trait::async_trait;
use iota_sdk::client::secret::SecretManager;
use iota_sdk::crypto::keys::bip39::Mnemonic;
use iota_sdk::types::block::payload::transaction::TransactionId;
use iota_sdk::types::block::payload::TaggedDataPayload;
use iota_sdk::wallet::account::types::Transaction;
use iota_sdk::wallet::account::{Account, SyncOptions, TransactionOptions};
use iota_sdk::wallet::ClientOptions;
use log::{error, info};
use rust_decimal_macros::dec;
use std::fmt::Debug;
use std::path::Path;
use std::str::FromStr;

/// The number of addresses to automatically generate when setting up the wallet
const USER_ADDRESS_LIMIT: u32 = 20;

///The name of the application used as the account name in the wallet
const APP_NAME: &str = "standalone";

#[cfg_attr(test, mockall::automock)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
/// Wallet user interface
pub trait WalletUser: Debug {
    /// Gets a new address for the user
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the generated address as a `String` if successful, or an `Error` if it fails.
    ///
    /// # Errors
    ///
    /// This function can return an error if it fails to synchronize the wallet, generate addresses, or encounter any other issues.
    async fn get_address(&self) -> Result<String>;

    /// Gets the balance of a user.
    ///
    /// # Returns
    ///
    /// Returns the available balance of the user as a `f64` if successful, or an `Error` if it fails.
    ///
    /// # Errors
    ///
    /// This function can return an error if it fails to synchronize the wallet or encounters any other issues.
    async fn get_balance(&self) -> Result<CryptoAmount>;

    /// Send amount to receiver
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the receiver.
    /// * `amount` - The amount to send.
    /// * `tag` - The transactions tag. Optional.
    /// * `message` - The transactions message. Optional.
    ///
    ///
    /// Returns a `Result` containing the sent transaction if successful, or an `Error` if it fails.
    ///
    /// # Errors
    ///
    /// This function can return an error if it fails to synchronize the wallet, send the transaction, or encounter any other issues.
    async fn send_amount(
        &self,
        address: &str,
        amount: CryptoAmount,
        tag: Option<TaggedDataPayload>,
        message: Option<String>,
    ) -> Result<Transaction>;

    /// Send eth amount to receiver
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the receiver.
    /// * `amount` - The amount to send (Ether).
    /// * `tag` - The transactions tag. Optional.
    /// * `message` - The transactions message. Optional.
    ///
    ///
    /// Returns a `Result` containing the sent transaction id if successful, or an `Error` if it fails.
    ///
    /// # Errors
    ///
    /// This function can return an error if it fails to synchronize the wallet, send the transaction, or encounter any other issues.
    #[allow(clippy::too_many_arguments)]
    async fn send_amount_eth(
        &self,
        address: &str,
        amount: CryptoAmount,
        tag: Option<TaggedDataPayload>,
        message: Option<String>,
    ) -> Result<String>;

    /// Send a transaction with one output.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the transaction.
    /// * `address` - The address to send the amount.
    /// * `amount` - The amount to send.
    ///
    /// # Returns
    ///
    /// Returns the sent transaction id if successful, or an `Error` if it fails.
    ///
    /// # Errors
    ///
    /// This function can return an error if it fails to synchronize the wallet, if there is insufficient balance, or encounters any other issues.
    async fn send_transaction(&self, index: &str, address: &str, amount: CryptoAmount) -> Result<String>;

    /// Send a transaction with one output.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the transaction.
    /// * `address` - The address to send the amount.
    /// * `amount` - The amount to send (Ether)
    ///
    /// # Returns
    ///
    /// Returns the sent transaction id if successful, or an `Error` if it fails.
    ///
    /// # Errors
    ///
    /// This function can return an error if it fails to synchronize the wallet, if there is insufficient balance, or encounters any other issues.
    #[allow(clippy::too_many_arguments)]
    async fn send_transaction_eth(&self, index: &str, address: &str, amount: CryptoAmount) -> Result<String>;

    /// Synchronizes the wallet with the network.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the synchronization is successful, or an `Error` if it fails.
    ///
    /// # Errors
    ///
    /// This function can return an error if it fails to synchronize the wallet or encounters any other issues.
    async fn sync_wallet(&self) -> Result<()>;

    /// Synchronizes given wallet transactions with the network.
    /// If the transaction information cannot be retrieved from the network, the input transaction will remain unchanged.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())`.
    async fn sync_transactions(&self, transactions: &mut [WalletTxInfo], start: usize, limit: usize) -> Result<()>;

    /// Gets the list of transactions
    ///
    /// # Arguments
    ///
    /// * `start` - The index of the first wallet transaction to return
    /// * `limit` - The number of following wallet transactions to return
    ///
    /// # Returns
    ///
    /// The list of wallet transactions.
    ///
    /// # Errors
    ///
    /// This function can return an error if it cannot retrieve the list of wallet transactions.
    async fn get_wallet_tx_list(&self, start: usize, limit: usize) -> Result<WalletTxInfoList>;

    /// Get detailed report of a particular transaction in the history
    ///
    /// # Arguments
    ///
    /// * `tx_id` - The id of the wallet transaction to return the details for.
    ///
    /// # Returns
    ///
    /// The wallet transaction details.
    ///
    /// # Errors
    ///
    /// This function can return an error if it cannot retrieve the wallet transaction.
    async fn get_wallet_tx(&self, tx_id: &str) -> Result<WalletTxInfo>;

    /// Estimate gas cost for eip 1559 transaction
    ///
    /// # Arguments
    ///
    /// * `transaction` - A transaction with a priority fee ([EIP-1559](https://eips.ethereum.org/EIPS/eip-1559))
    ///
    /// # Returns the estimated gas cost for the underlying transaction to be executed (gas limit, max fee per gas and max priority fee per gas)
    ///
    /// This function can return an error if it cannot parse input transaction or retrieve information from the node.
    async fn estimate_gas_cost_eip1559(&self, transaction: TxEip1559) -> Result<GasCostEstimation>;
}

/// [`WalletUser`] implementation for IOTA and SMR using the stardust protocol
#[derive(Debug)]
pub struct WalletImplStardust {
    /// State for the account manager to be passed to calling functions
    account_manager: iota_sdk::wallet::Wallet,
}

impl WalletImplStardust {
    /// Creates a new [`WalletImpl`] from the specified [`Config`] and [`Mnemonic`].
    pub async fn new(mnemonic: Mnemonic, path: &Path, currency: Currency, node_url: String) -> Result<Self> {
        // we now have the mnemonic and can initialize a wallet
        let node_urls = vec![node_url.as_str()];

        info!("Used node_urls: {:?}", node_urls);
        let client_options = ClientOptions::new()
            .with_local_pow(false)
            .with_fallback_to_local_pow(true)
            .with_nodes(&node_urls)?;

        let coin_type = currency.coin_type();

        // we need to make sure the path exists, or we will get IO errors, but only if we are not on wasm
        #[cfg(not(target_arch = "wasm32"))]
        if let Err(e) = std::fs::create_dir_all(path) {
            error!("Could not create the wallet directory: {e:?}");
        }

        let account_manager = {
            let secret_manager = SecretManager::try_from_mnemonic(mnemonic)?;
            iota_sdk::wallet::Wallet::builder()
                .with_client_options(client_options)
                .with_coin_type(coin_type)
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

        Ok(WalletImplStardust { account_manager })
    }
}

/// The minimum amount of IOTA to consider an output as non-dust output. Everything below this is dust.
const MIN_DUST_OUTPUT: CryptoAmount = unsafe { CryptoAmount::new_unchecked(dec!(0.1)) }; // SAFETY: the value is non-negative

/// The value to divide an amount in GLOW with to get IOTA
const GLOW_TO_IOTA_DIVISOR: CryptoAmount = unsafe { CryptoAmount::new_unchecked(dec!(1_000_000)) }; // SAFETY: the value is non-negative

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl WalletUser for WalletImplStardust {
    async fn get_address(&self) -> Result<String> {
        self.sync_wallet().await?;
        let account: Account = self.account_manager.get_account(APP_NAME).await?;
        let account_addresses = account.generate_ed25519_addresses(USER_ADDRESS_LIMIT, None).await?;
        let account_address = account_addresses.first();
        match account_address {
            Some(address) => {
                let bech32_address = address.clone().into_bech32().to_string();
                Ok(bech32_address)
            }
            None => Err(WalletError::EmptyWalletAddress),
        }
    }

    async fn get_balance(&self) -> Result<CryptoAmount> {
        self.sync_wallet().await?;
        let account = self.account_manager.get_account(APP_NAME).await?;
        let balance = account.balance().await?;
        let available_balance_glow = balance.base_coin().available();
        let available_balance_iota = CryptoAmount::from(available_balance_glow) / GLOW_TO_IOTA_DIVISOR;
        Ok(available_balance_iota)
    }

    async fn send_amount(
        &self,
        address: &str,
        amount: CryptoAmount,
        tag: Option<TaggedDataPayload>,
        message: Option<String>,
    ) -> Result<Transaction> {
        self.sync_wallet().await?;

        let account = self.account_manager.get_account(APP_NAME).await?;

        let options = TransactionOptions {
            allow_micro_amount: true,
            tagged_data_payload: tag,
            note: message,
            ..Default::default()
        };

        let amount_glow: u64 = (amount.inner() * dec!(1_000_000)).round().try_into()?;
        let transaction = account.send(amount_glow, address, options).await?;
        Ok(transaction)
    }

    async fn send_amount_eth(
        &self,
        _address: &str,
        _amount: CryptoAmount,
        _tag: Option<TaggedDataPayload>,
        _message: Option<String>,
    ) -> Result<String> {
        Err(WalletError::WalletFeatureNotImplemented)
    }

    async fn estimate_gas_cost_eip1559(&self, _transaction: TxEip1559) -> Result<GasCostEstimation> {
        Err(WalletError::WalletFeatureNotImplemented)
    }

    async fn sync_transactions(&self, _transactions: &mut [WalletTxInfo], _start: usize, _limit: usize) -> Result<()> {
        Err(WalletError::WalletFeatureNotImplemented)
    }

    async fn send_transaction(&self, index: &str, address: &str, amount: CryptoAmount) -> Result<String> {
        let transaction = prepare_and_send_transaction(self, index, address, amount).await?;
        Ok(transaction.transaction_id.to_string())
    }

    #[allow(clippy::too_many_arguments)]
    async fn send_transaction_eth(&self, _index: &str, _address: &str, _amount: CryptoAmount) -> Result<String> {
        Err(WalletError::WalletFeatureNotImplemented)
    }

    async fn sync_wallet(&self) -> Result<()> {
        let account = self.account_manager.get_account(APP_NAME).await?;

        let options = SyncOptions {
            force_syncing: true,
            ..Default::default()
        };
        account.sync(Some(options)).await?;

        Ok(())
    }

    // Gets the list of transactions
    async fn get_wallet_tx_list(&self, start: usize, limit: usize) -> Result<WalletTxInfoList> {
        let account = self.account_manager.get_account(APP_NAME).await?;

        // Calculate the start index for the current page
        let start_index = start * limit;

        // Fetch all iota transactions from the account
        // notice those are iota Transactions
        let all_transactions = account.transactions().await;

        // Extract transactions for the current page based on the start index and page size
        // and convert to the internal wallet Transaction
        let transactions_for_page = all_transactions
            .into_iter()
            .skip(start_index)
            .take(limit)
            .map(Into::into)
            .collect::<Vec<WalletTxInfo>>();

        Ok(WalletTxInfoList {
            transactions: transactions_for_page,
        })
    }

    /// Get detailed report of a particular transaction in the history
    async fn get_wallet_tx(&self, tx_id: &str) -> Result<WalletTxInfo> {
        let account = self.account_manager.get_account(APP_NAME).await?;
        // Parse the transaction ID string into the appropriate type, assuming TransactionId is a specific type
        let transaction_id: TransactionId = tx_id
            .parse()
            .map_err(|e: <TransactionId as FromStr>::Err| WalletError::InvalidTransaction(e.to_string()))?;
        // Fetch detailed transaction by its id using your wallet SDK's methods
        if let Some(transaction) = account.get_transaction(&transaction_id).await {
            Ok(transaction.into())
        } else {
            Err(WalletError::MissingAccessToken)
        }
    }
}

/// Prepare and send transaction
async fn prepare_and_send_transaction(
    user: &WalletImplStardust,
    index: &str,
    address: &str,
    amount: CryptoAmount,
) -> Result<Transaction> {
    // Check if we have enough balance, otherwise return with err
    let balance = user.get_balance().await?;

    if balance < (amount + MIN_DUST_OUTPUT) {
        return Err(WalletError::InsufficientBalance(String::from("Not enough balance")));
    }

    let account = user.account_manager.get_account(APP_NAME).await?;

    // convert to glow
    let amount_glow: u64 = (amount.inner() * dec!(1_000_000)).round().try_into()?;

    let tag = String::from(APP_NAME).into_bytes();
    let data = String::from(index).into_bytes();
    let tagged_data_payload = TaggedDataPayload::new(tag, data)?;

    let options = TransactionOptions {
        allow_micro_amount: true,
        tagged_data_payload: Some(tagged_data_payload),
        ..Default::default()
    };

    let transaction = account.send(amount_glow, address, options).await?;
    let block_id = account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    info!("Transaction successfully included in block: {block_id}!");

    Ok(transaction)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Config;
    use crate::testing_utils::MNEMONIC;
    use crate::types::{self, currencies::Currency};
    use iota_sdk::{
        crypto::keys::bip39::Mnemonic,
        types::block::payload::{transaction::TransactionEssence, Payload},
    };
    use rstest::rstest;
    use rust_decimal::prelude::ToPrimitive;
    use rust_decimal_macros::dec;
    use testing::CleanUp;

    // General Note:
    // - Check the corresponding wallet address on the explorer: https://explorer.shimmer.network/testnet
    // - Obtain testnet tokens for the wallet (if needed): https://faucet.testnet.shimmer.network/

    /// helper function to get a [`WalletUser`] instance.
    async fn get_wallet_user(mnemonic: impl Into<Mnemonic>, currency: Currency) -> (WalletImplStardust, CleanUp) {
        let (_, cleanup) = Config::new_test_with_cleanup();
        let node_url = String::from("https://api.testnet.iotaledger.net");
        let wallet = WalletImplStardust::new(mnemonic.into(), Path::new(&cleanup.path_prefix), currency, node_url)
            .await
            .expect("should initialize wallet");
        (wallet, cleanup)
    }

    #[cfg_attr(coverage, ignore = "Takes too long under code-coverage")]
    #[tokio::test]
    async fn test_serial_send_tx() {
        // Arrange
        let mnemonic = "predict wrist plug desert mobile crowd build leg swap impose breeze loyal surge brand hair bronze melody scale hello cereal car item slow bring";

        let (wallet_user, _cleanup) = get_wallet_user(mnemonic, Currency::Iota).await;

        let address = wallet_user.get_address().await.unwrap(); // tst1qzlzd9q4aygcnuteefaekd2kvatpt3xp673sspr2hcw27zuewxpmugyavz9
        let index = String::from("TestMessageFromStandalone");
        let balance = wallet_user.get_balance().await.unwrap();
        println!("Address: {address}, balance: {balance:?}");
        let amount = CryptoAmount::try_from(dec!(10.0)).unwrap();
        // Act
        let transaction = prepare_and_send_transaction(&wallet_user, &index, &address, amount).await;

        // Assert
        let ok_transaction = transaction.unwrap();

        // get output
        let essence = ok_transaction.payload.essence();
        let TransactionEssence::Regular(regular_essence) = essence;

        let outputs = regular_essence.outputs();
        for output in outputs {
            println!("{output:#?}");
        }

        println!("{ok_transaction:#?}");
    }

    #[rstest]
    #[case(Currency::Iota, "tst")]
    #[tokio::test]
    async fn test_get_address(#[case] currency: Currency, #[case] expected_prefix: &str) {
        // Arrange
        let (wallet_user, _cleanup) = get_wallet_user(MNEMONIC, currency).await;

        // Act
        let address = wallet_user.get_address().await;

        // Check the address prefix
        let address = address.unwrap(); // Unwrap the address since we already checked it's Ok
        assert!(
            address.starts_with(expected_prefix),
            "Address did not start with the expected prefix"
        );
    }

    #[rstest]
    #[case(Currency::Iota)]
    #[tokio::test]
    async fn test_get_balance(#[case] currency: Currency) {
        // Arrange
        let (wallet_user, _cleanup) = get_wallet_user(MNEMONIC, currency).await;

        // Act
        let balance = wallet_user.get_balance().await;

        // Assert
        balance.unwrap();
    }

    #[tokio::test]
    async fn test_send_tx_more_than_balance() {
        // Arrange
        let mnemonic = "champion legend palm hollow describe timber coast lonely future holiday head torch race orange ranch gun broccoli average margin glue age awake nurse erase";
        let (wallet_user, _cleanup) = get_wallet_user(mnemonic, Currency::Iota).await;
        let address = wallet_user.get_address().await.unwrap();
        let index = String::from("TestMessageFromStandalone");
        let balance = wallet_user.get_balance().await.unwrap();
        println!("Address: {address}, balance: {balance:?}");
        let amount = balance + CryptoAmount::try_from(dec!(10.0)).unwrap();

        // Act
        let transaction = wallet_user.send_transaction(&index, &address, amount).await;

        // Assert
        assert!(transaction.is_err(), "Wallet user has insufficient balance");
    }

    #[cfg_attr(coverage, ignore = "Takes too long under code-coverage")]
    #[tokio::test]
    async fn test_send_tx_less_than_balance() {
        // Arrange
        let mnemonic = "arch sentence dwarf label unlock grace left orient practice oval sport rubber airport carbon moral can scan clinic false hen fancy repeat hip green";
        let (wallet_user, _cleanup) = get_wallet_user(mnemonic, Currency::Iota).await;

        let address = wallet_user.get_address().await.unwrap(); // st1qz46hgjrtjkgxvmqn6q42l832w3mjerdjxfv756hc4fnrlfeh9g07ge7m2w
        let index = String::from("TestMessageFromStandalone");
        let balance = wallet_user.get_balance().await.unwrap();
        println!("Address: {address}, balance: {balance:?}");
        let amount = CryptoAmount::try_from(balance.inner() - dec!(1.0)).unwrap();

        // Act
        let transaction = wallet_user.send_transaction(&index, &address, amount).await;

        // Assert
        transaction.unwrap();
    }

    #[cfg_attr(coverage, ignore = "Takes too long under code-coverage")]
    #[tokio::test]
    async fn test_serial_get_wallet_tx_list() {
        // Arrange
        let mnemonic = "west neutral cannon wreck notice disorder message three phrase accident office flavor merit kiss claim finish finger forum mesh mouse torch cradle inside glue";
        let (wallet_user, _cleanup) = get_wallet_user(mnemonic, Currency::Iota).await;

        let address = wallet_user.get_address().await.unwrap(); // tst1qpaha27ytq8ay3ahqlfl6rn5xndnwxwahunf20h59cadpt9q2gx0crqekxk

        let balance = wallet_user.get_balance().await.unwrap();
        println!("Address: {address}, balance: {balance:?}");
        let amount = CryptoAmount::try_from(balance.inner() - dec!(1.0)).unwrap();

        let index = String::from("TestMessageFromStandalone");
        let transaction = prepare_and_send_transaction(&wallet_user, &index, &address, amount)
            .await
            .unwrap();

        let start = 0;
        let limit = 10;

        // Act
        let result = wallet_user.get_wallet_tx_list(start, limit).await;

        // Assert
        let wallet_tx_info_list = result.unwrap();
        let transactions: Vec<types::transactions::WalletTxInfo> = wallet_tx_info_list.transactions.to_vec();

        assert_eq!(transactions.len(), 1);
        assert_eq!(transactions[0], transaction.into());
    }

    #[cfg_attr(coverage, ignore = "Takes too long under code-coverage")]
    #[tokio::test]
    async fn test_serial_get_wallet_tx_success() {
        // Arrange
        let mnemonic = "century jazz giant zebra pledge head school supreme aim certain moment mechanic curtain chronic duck addict despair pistol romance risk impulse upgrade rubber grid";
        let (wallet_user, _cleanup) = get_wallet_user(mnemonic, Currency::Iota).await;

        let address = wallet_user.get_address().await.unwrap(); //tst1qq68c239vacsq3gnksqss5glh9e3w0wc9mra9d6cs39e8hs3xrmgjscdpac
        let balance = wallet_user.get_balance().await.unwrap();
        println!("Address: {address}, balance: {balance:?}");
        let amount = CryptoAmount::try_from(balance.inner() - dec!(1.0)).unwrap();

        let index = String::from("TestMessageFromStandalone");
        let transaction = prepare_and_send_transaction(&wallet_user, &index, &address, amount)
            .await
            .unwrap();

        let transaction_str = transaction.transaction_id.to_string();

        // Act
        let result = wallet_user.get_wallet_tx(&transaction_str).await;

        // Assert
        assert_eq!(result.unwrap(), transaction.into());
    }

    #[tokio::test]
    async fn test_get_wallet_tx_error_nonexistent_transaction() {
        // Arrange
        let (wallet_user, _cleanup) = get_wallet_user(MNEMONIC, Currency::Iota).await;

        let transaction_id = "nonexistent_transaction_id";

        // Act
        let result = wallet_user.get_wallet_tx(transaction_id).await;

        // Assert
        assert!(result.is_err(), "Error occurred while fetching wallet transaction");
    }

    #[cfg_attr(coverage, ignore = "Takes too long under code-coverage")]
    #[tokio::test]
    async fn it_should_send_amount_with_tag_and_message() {
        // Arrange
        let (wallet_user, _cleanup) = get_wallet_user(MNEMONIC, Currency::Iota).await;

        let address = wallet_user.get_address().await.unwrap();
        let balance = wallet_user.get_balance().await.unwrap();
        println!("Address: {address}, balance: {balance:?}");

        let amount = CryptoAmount::try_from(balance.inner() - dec!(1.0)).unwrap();
        let tag: Box<[u8]> = "test tag".to_string().into_bytes().into_boxed_slice();
        let data: Box<[u8]> = (amount.inner() * dec!(1_000_000))
            .round()
            .to_u64()
            .unwrap()
            .to_be_bytes()
            .into();
        let tagged_data_payload = Some(TaggedDataPayload::new(tag.clone(), data.clone()).unwrap());
        let message = Some(String::from("test message"));

        // Act
        let result = wallet_user
            .send_amount(&address, amount, tagged_data_payload.clone(), message.clone())
            .await;

        // Assert
        let transaction = result.unwrap();

        assert_eq!(
            transaction.payload.essence().as_regular().payload(),
            Some(Payload::TaggedData(Box::new(
                TaggedDataPayload::new(tag, data).unwrap()
            )))
            .as_ref()
        );
        assert_eq!(transaction.note, message);
    }

    #[cfg_attr(coverage, ignore = "Takes too long under code-coverage")]
    #[tokio::test]
    async fn it_should_send_amount_without_tag_and_message() {
        // Arrange
        let (wallet_user, _cleanup) = get_wallet_user(MNEMONIC, Currency::Iota).await;

        let address = wallet_user.get_address().await.unwrap();
        let balance = wallet_user.get_balance().await.unwrap();
        println!("Address: {address}, balance: {balance:?}");
        let amount = CryptoAmount::try_from(balance.inner() - dec!(1.0)).unwrap();

        // Act
        let result = wallet_user.send_amount(&address, amount, None, None).await;

        // Assert
        let transaction = result.unwrap();

        assert_eq!(transaction.payload.essence().as_regular().payload(), None);
        assert_eq!(transaction.note, None);
    }

    #[tokio::test]
    async fn it_should_not_send_amount_more_then_balance() {
        // Arrange
        let (wallet_user, _cleanup) = get_wallet_user(MNEMONIC, Currency::Iota).await;

        let address = wallet_user.get_address().await.unwrap();
        let balance = wallet_user.get_balance().await.unwrap();
        println!("Address: {address}, balance: {balance:?}");
        let amount = CryptoAmount::try_from(dec!(96854.0)).unwrap();

        // Act
        let transaction = wallet_user.send_amount(&address, amount, None, None).await;

        // Assert
        assert!(
            matches!(
                transaction,
                Err(WalletError::IotaWallet(
                    iota_sdk::wallet::Error::InsufficientFunds { .. }
                ))
            ),
            "Got: {transaction:?}"
        );
    }
}
