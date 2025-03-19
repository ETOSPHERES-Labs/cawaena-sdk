//! This module provides methods for initializing, verifying, deleting, and creating wallets, as well as
//! migrating wallets from mnemonic or backup, creating backups, and verifying PINs.
//!
//! It also includes various helper functions and imports required for the wallet functionality.

use super::Sdk;
use crate::{
    backend::dlt::put_user_address,
    error::Result,
    types::{
        currencies::CryptoAmount,
        newtypes::{EncryptionPin, EncryptionSalt, PlainPassword},
        transactions::{WalletTxInfo, WalletTxInfoList},
    },
    wallet::error::{ErrorKind, WalletError},
};
use log::{debug, info, warn};

impl Sdk {
    /// Create and store a wallet from a new random mnemonic
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN for the wallet.
    ///
    /// # Returns
    ///
    /// The new random mnemonic.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::MissingConfig`] - If the sdk config is missing.
    /// * [`crate::Error::UserRepoNotInitialized`] - If there is an error initializing the repository.
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    pub async fn create_wallet_from_new_mnemonic(&mut self, pin: &EncryptionPin) -> Result<String> {
        info!("Creating a new wallet from random mnemonic");

        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };

        let Some(active_user) = &mut self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;

        let mnemonic = active_user
            .wallet_manager
            .create_wallet_from_new_mnemonic(config, &self.access_token, repo, pin)
            .await?;
        Ok(mnemonic)
    }

    /// Create and store a wallet from an existing mnemonic
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN for the wallet.
    /// * `mnemonic` - The mnemonic to use for the wallet.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::MissingConfig`] - If the sdk config is missing.
    /// * [`crate::Error::UserRepoNotInitialized`] - If there is an error initializing the repository.
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    pub async fn create_wallet_from_existing_mnemonic(&mut self, pin: &EncryptionPin, mnemonic: &str) -> Result<()> {
        info!("Creating a new wallet from existing mnemonic");

        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };

        let Some(active_user) = &mut self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;

        active_user
            .wallet_manager
            .create_wallet_from_existing_mnemonic(config, &self.access_token, repo, pin, mnemonic)
            .await?;
        Ok(())
    }

    /// Create and store a wallet from an existing kdbx backup file
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN for the wallet.
    /// * `backup` - The bytes representing the backup file.
    /// * `backup_password` - The password used when creating the backup file.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::MissingConfig`] - If the sdk config is missing.
    /// * [`crate::Error::UserRepoNotInitialized`] - If there is an error initializing the repository.
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    pub async fn create_wallet_from_backup(
        &mut self,
        pin: &EncryptionPin,
        backup: &[u8],
        backup_password: &PlainPassword,
    ) -> Result<()> {
        info!("Creating a new wallet from backup");

        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };

        let Some(active_user) = &mut self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;

        active_user
            .wallet_manager
            .create_wallet_from_backup(config, &self.access_token, repo, pin, backup, backup_password)
            .await?;
        Ok(())
    }

    /// Create a kdbx wallet backup from an existing wallet.
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN for the wallet.
    /// * `backup_password` - The password to use when creating the backup file.
    ///
    /// # Returns
    ///
    /// The bytes of the kdbx backup file.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::MissingConfig`] - If the sdk config is missing.
    /// * [`crate::Error::UserRepoNotInitialized`] - If there is an error initializing the repository.
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    pub async fn create_wallet_backup(
        &mut self,
        pin: &EncryptionPin,
        backup_password: &PlainPassword,
    ) -> Result<Vec<u8>> {
        info!("Creating wallet backup");

        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };

        let Some(active_user) = &mut self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;

        let backup = active_user
            .wallet_manager
            .create_wallet_backup(config, &self.access_token, repo, pin, backup_password)
            .await?;
        Ok(backup)
    }

    /// Verify the mnemonic by checking if the mnemonic is the same as the one in the shares
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN for the wallet.
    /// * `mnemonic` - The mnemonic to verify.
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if the mnemonic is successfully verified, otherwise returns `Ok(false)`,
    /// or an `Error`.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::MissingConfig`] - If the sdk config is missing.
    /// * [`crate::Error::UserRepoNotInitialized`] - If there is an error initializing the repository.
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    pub async fn verify_mnemonic(&mut self, pin: &EncryptionPin, mnemonic: &str) -> Result<bool> {
        info!("Verifying mnemonic");

        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };

        let Some(active_user) = &mut self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;

        let is_verified = active_user
            .wallet_manager
            .check_mnemonic(config, &self.access_token, repo, pin, mnemonic)
            .await?;
        Ok(is_verified)
    }

    /// Delete the currently active wallet
    ///
    /// Deletes the currently active wallet, potentially resulting in loss of funds if the mnemonic or wallet is not backed up.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the wallet is successfully deleted, otherwise returns an `Error`.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::MissingConfig`] - If the sdk config is missing.
    /// * [`crate::Error::UserRepoNotInitialized`] - If there is an error initializing the repository.
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    pub async fn delete_wallet(&mut self, pin: &EncryptionPin) -> Result<()> {
        warn!("Deleting wallet for user. Potential loss of funds if mnemonic/wallet is not backed up!");

        self.verify_pin(pin).await?;

        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };
        let Some(active_user) = &mut self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;

        active_user
            .wallet_manager
            .delete_wallet(config, &self.access_token, repo)
            .await?;

        Ok(())
    }

    /// Verify pin
    ///
    /// Verifies the pin for the wallet.
    ///
    /// # Arguments
    ///
    /// * `pin` - The pin to verify.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the pin is verified successfully, otherwise returns an `Error`.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::UserRepoNotInitialized`] - If there is an error initializing the repository.
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    /// * [`WalletError::WalletNotInitialized`] - If there is an error initializing the wallet.
    /// * [`WalletError::WrongPinOrPassword`] - If the pin or password is incorrect.
    pub async fn verify_pin(&self, pin: &EncryptionPin) -> Result<()> {
        info!("Verifying wallet pin");
        let Some(repo) = &self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };
        let Some(active_user) = &self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        let username = &active_user.username;
        let user = repo.get(username)?;

        // Ensure encrypted password exists in user
        let Some(encrypted_password) = user.encrypted_password else {
            return Err(WalletError::WalletNotInitialized(ErrorKind::MissingPassword))?;
        };

        // Decrypt the password using the provided PIN
        if encrypted_password.decrypt(pin, &user.salt).is_err() {
            return Err(WalletError::WrongPinOrPassword)?;
        }
        Ok(())
    }

    /// Reset pin
    ///
    /// Resets the pin for the wallet using the provided password and new pin.
    ///
    /// # Arguments
    ///
    /// * `old_pin` - The old wallet pin.
    /// * `new_pin` - The new pin to set for the wallet.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the pin is changed successfully, otherwise returns an `Error`.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::UserRepoNotInitialized`] - If there is an error initializing the repository.
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    /// * [`WalletError::WalletNotInitialized`] - If there is an error initializing the wallet.
    /// * [`WalletError::WrongPinOrPassword`] - If the pin or password is incorrect.
    pub async fn change_pin(&mut self, old_pin: &EncryptionPin, new_pin: &EncryptionPin) -> Result<()> {
        info!("Resetting pin with password");
        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };
        let Some(active_user) = &mut self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        let username = &active_user.username;
        let mut user = repo.get(username)?;

        let Some(encrypted_password) = user.encrypted_password else {
            return Err(WalletError::WalletNotInitialized(ErrorKind::MissingPassword))?;
        };

        // decrypt the password
        let password = encrypted_password.decrypt(old_pin, &user.salt)?;

        // Set new pin and encrypted password
        let salt = EncryptionSalt::generate();
        let encrypted_password = password.encrypt(new_pin, &salt)?;

        // Update user
        user.salt = salt;
        user.encrypted_password = Some(encrypted_password);
        repo.update(&user)?;

        Ok(())
    }

    /// Set the password to use for wallet operations. If the password was already set, this changes it.
    ///
    /// # Arguments
    ///
    /// * `pin` - The pin to encrypt the password with.
    /// * `new_password` - The new password to set for the wallet.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the password is set successfully, otherwise returns an `Error`.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::UserRepoNotInitialized`] - If there is an error initializing the repository.
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    pub async fn set_wallet_password(&mut self, pin: &EncryptionPin, new_password: &PlainPassword) -> Result<()> {
        info!("Setting password");

        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };
        let Some(active_user) = &mut self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        let mut user = repo.get(&active_user.username)?;

        // if password already exists, return an error!
        if let Some(encrypted_password) = user.encrypted_password {
            info!("Password exists, changing password");

            // verify that the pin is correct by decrypting the password using the provided PIN
            if encrypted_password.decrypt(pin, &user.salt).is_err() {
                return Err(WalletError::WrongPinOrPassword)?;
            }

            let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;

            active_user
                .wallet_manager
                .change_wallet_password(config, &self.access_token, repo, pin, new_password)
                .await?;
        } else {
            // Set new pin and encrypted password
            let salt = EncryptionSalt::generate();
            let encrypted_password = new_password.encrypt(pin, &salt)?;

            // Update user
            user.salt = salt;
            user.encrypted_password = Some(encrypted_password);
            repo.update(&user)?;
        }

        Ok(())
    }

    /// Check if the password to use for wallet operations is set. If this returns `false`,
    /// the password should be set with [`set_wallet_password`], otherwise you need to use
    /// [`change_password`] to change it.
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if the password is set successfully, otherwise returns `Ok(false)`.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::UserRepoNotInitialized`] - If there is an error initializing the repository.
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    pub async fn is_wallet_password_set(&self) -> Result<bool> {
        info!("Checking if password is set");

        let Some(repo) = &self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };
        let Some(active_user) = &self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        let user = repo.get(&active_user.username)?;

        Ok(user.encrypted_password.is_some())
    }

    /// Generates a new receiver address (based on selected currency in the config) for the wallet.
    ///
    /// # Returns
    ///
    /// Returns the generated address as a `String` if successful, otherwise returns an `Error`.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::UserRepoNotInitialized`] - If there is an error initializing the repository.
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    /// * [`crate::Error::MissingConfig`] - If the sdk config is missing.
    pub async fn generate_new_address(&mut self, pin: &EncryptionPin) -> Result<String> {
        info!("Generating new wallet address");
        self.verify_pin(pin).await?;
        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };
        let Some(active_user) = &mut self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };
        let network = self.network.clone().ok_or(crate::Error::MissingNetwork)?;
        let config = self.config.as_mut().ok_or(crate::Error::MissingConfig)?;
        let wallet = active_user
            .wallet_manager
            .try_get(config, &self.access_token, repo, network.clone(), pin)
            .await?;

        let address = wallet.get_address().await?;

        // if there is an access token, push the generated address to the backend
        if let Some(access_token) = self.access_token.as_ref() {
            put_user_address(config, access_token, network.key, &address).await?;
        }
        debug!("Generated address: {address}");
        Ok(address)
    }

    /// Get the balance of the user
    ///
    /// Fetches the balance of the user from the wallet.
    ///
    /// # Returns
    ///
    /// Returns the balance as a `f64` if successful, otherwise returns an `Error`.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    /// * [`WalletError::WalletNotInitialized`] - If there is an error initializing the wallet.
    pub async fn get_balance(&mut self, pin: &EncryptionPin) -> Result<CryptoAmount> {
        info!("Fetching balance");
        self.verify_pin(pin).await?;
        let wallet = self.try_get_active_user_wallet(pin).await?;
        let balance = wallet.get_balance().await?;
        debug!("Balance: {balance:?}");
        Ok(balance)
    }

    /// wallet transaction list
    ///
    /// Returns paginated list of wallet transaction list.
    ///
    /// # Arguments
    ///
    /// * `start` - The starting index of transactions to fetch.
    /// * `limit` - The number of transactions per page.
    ///
    /// # Returns
    ///
    /// Returns a `WalletTxInfoList` containing paginated history of wallet transactions if the outputs are claimed successfully, otherwise returns an `Error`.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    /// * [`WalletError::WalletNotInitialized`] - If there is an error initializing the wallet.
    pub async fn get_wallet_tx_list(
        &mut self,
        pin: &EncryptionPin,
        start: usize,
        limit: usize,
    ) -> Result<WalletTxInfoList> {
        info!("Wallet getting list of transactions");
        self.verify_pin(pin).await?;

        let network = self.network.clone().ok_or(crate::Error::MissingNetwork)?;
        let user = self.get_user().await?;
        let wallet = self.try_get_active_user_wallet(pin).await?;

        let tx_list = match network.network_type {
            crate::types::networks::NetworkType::EvmErc20 {
                node_urls: _,
                chain_id: _,
                contract_address: _,
            } => wallet.get_wallet_tx_list(start, limit).await?,
            crate::types::networks::NetworkType::Evm {
                node_urls: _,
                chain_id: _,
                contract_address: _,
            } => {
                // We retrieve the transaction list from the wallet,
                // then synchronize selected transactions (by fetching their current status from the network),
                // and finally, save the refreshed list back to the wallet
                let mut wallet_transactions = user.wallet_transactions;

                for transaction in wallet_transactions.iter_mut().skip(start).take(limit) {
                    let synchronized_transaction = wallet.get_wallet_tx(&transaction.transaction_id).await;
                    match synchronized_transaction {
                        Ok(stx) => *transaction = stx,
                        Err(e) => {
                            // On error, return historical (cached) transaction data
                            log::debug!("[sync_transactions] could not retrieve data about transaction from the network, transaction: {:?}, error: {:?}", transaction.clone(), e);
                        }
                    }
                }

                let Some(repo) = &mut self.repo else {
                    return Err(crate::Error::UserRepoNotInitialized);
                };

                let _ = repo.set_wallet_transactions(&user.username, wallet_transactions.clone());

                WalletTxInfoList {
                    transactions: wallet_transactions,
                }
            }
            api_types::api::networks::ApiProtocol::Stardust {} => wallet.get_wallet_tx_list(start, limit).await?,
        };

        Ok(tx_list)
    }

    /// wallet transaction
    ///
    /// Returns the wallet transaction details.
    ///
    /// # Arguments
    ///
    /// * `tx_id` - The transaction id of particular transaction.
    ///
    /// # Returns
    ///
    /// Returns `WalletTxInfo` detailed report of particular wallet transaction if the outputs are claimed successfully, otherwise returns an `Error`.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    /// * [`WalletError::WalletNotInitialized`] - If there is an error initializing the wallet.
    pub async fn get_wallet_tx(&mut self, pin: &EncryptionPin, tx_id: &str) -> Result<WalletTxInfo> {
        info!("Wallet getting details of particular transactions");
        self.verify_pin(pin).await?;
        let wallet = self.try_get_active_user_wallet(pin).await?;
        let wallet_tx = wallet.get_wallet_tx(tx_id).await?;
        Ok(wallet_tx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::core_testing_utils::handle_error_test_cases;
    use crate::testing_utils::{
        example_api_networks, example_get_user, example_wallet_tx_info, set_config, ADDRESS, AUTH_PROVIDER,
        BACKUP_PASSWORD, HEADER_X_APP_NAME, MNEMONIC, PIN, SALT, TOKEN, TX_INDEX, USERNAME,
    };
    use crate::types::users::UserEntity;
    use crate::{
        core::Sdk,
        types::users::KycType,
        user::MockUserRepo,
        wallet_manager::{MockWalletManager, WalletBorrow},
        wallet_user::MockWalletUser,
    };
    use api_types::api::dlt::SetUserAddressRequest;
    use api_types::api::viviswap::detail::SwapPaymentDetailKey;
    use mockito::Matcher;
    use rstest::rstest;
    use rust_decimal_macros::dec;
    use std::sync::LazyLock;

    const BACKUP: &[u8] = &[42, 77, 15, 203, 89, 123, 34, 56, 178, 90, 210, 33, 47, 192, 1, 17];

    #[rstest]
    #[case::success(Ok(MNEMONIC.to_string()))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[tokio::test]
    async fn test_create_wallet_from_new_mnemonic(#[case] expected: Result<String>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        match &expected {
            Ok(_) => {
                sdk.repo = Some(Box::new(MockUserRepo::new()));
                let mut mock_wallet_manager = MockWalletManager::new();
                mock_wallet_manager
                    .expect_create_wallet_from_new_mnemonic()
                    .once()
                    .returning(|_, _, _, _| Ok(MNEMONIC.to_string()));
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(mock_wallet_manager),
                });
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 0, 0).await;
            }
        }

        // Act
        let response = sdk.create_wallet_from_new_mnemonic(&PIN).await;

        // Assert
        match expected {
            Ok(resp) => {
                assert_eq!(response.unwrap(), resp);
            }
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    #[rstest]
    #[case::success(Ok(()))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[tokio::test]
    async fn test_create_wallet_from_existing_mnemonic(#[case] expected: Result<()>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        match &expected {
            Ok(_) => {
                sdk.repo = Some(Box::new(MockUserRepo::new()));
                let mut mock_wallet_manager = MockWalletManager::new();
                mock_wallet_manager
                    .expect_create_wallet_from_existing_mnemonic()
                    .once()
                    .returning(|_, _, _, _, _| Ok(()));
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(mock_wallet_manager),
                });
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 0, 0).await;
            }
        }

        // Act
        let response = sdk.create_wallet_from_existing_mnemonic(&PIN, MNEMONIC).await;

        // Assert
        match expected {
            Ok(()) => response.unwrap(),
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    #[rstest]
    #[case::success(Ok(BACKUP.to_vec()))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[tokio::test]
    async fn test_create_wallet_backup(#[case] expected: Result<Vec<u8>>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        match &expected {
            Ok(_) => {
                sdk.repo = Some(Box::new(MockUserRepo::new()));
                let mut mock_wallet_manager = MockWalletManager::new();
                mock_wallet_manager
                    .expect_create_wallet_backup()
                    .once()
                    .returning(|_, _, _, _, _| Ok(BACKUP.to_vec()));
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(mock_wallet_manager),
                });
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 0, 0).await;
            }
        }

        // Act
        let response = sdk.create_wallet_backup(&PIN, &BACKUP_PASSWORD).await;

        // Assert
        match expected {
            Ok(resp) => {
                assert_eq!(response.unwrap(), resp);
            }
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    #[rstest]
    #[case::success(Ok(()))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[tokio::test]
    async fn test_create_wallet_from_backup(#[case] expected: Result<()>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        match &expected {
            Ok(_) => {
                sdk.repo = Some(Box::new(MockUserRepo::new()));
                let mut mock_wallet_manager = MockWalletManager::new();
                mock_wallet_manager
                    .expect_create_wallet_from_backup()
                    .once()
                    .returning(|_, _, _, _, _, _| Ok(()));
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(mock_wallet_manager),
                });
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 0, 0).await;
            }
        }

        // Act
        let response = sdk.create_wallet_from_backup(&PIN, BACKUP, &BACKUP_PASSWORD).await;

        // Assert
        match expected {
            Ok(()) => response.unwrap(),
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    #[rstest]
    #[case::success(Ok(true))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[tokio::test]
    async fn test_verify_mnemonic(#[case] expected: Result<bool>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        match &expected {
            Ok(_) => {
                sdk.repo = Some(Box::new(MockUserRepo::new()));
                let mut mock_wallet_manager = MockWalletManager::new();
                mock_wallet_manager
                    .expect_check_mnemonic()
                    .once()
                    .returning(|_, _, _, _, _| Ok(true));
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(mock_wallet_manager),
                });
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 0, 0).await;
            }
        }

        // Act
        let response = sdk.verify_mnemonic(&PIN, MNEMONIC).await;

        // Assert
        match expected {
            Ok(resp) => {
                assert_eq!(response.unwrap(), resp);
            }
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    #[rstest]
    #[case::success(Ok(()))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[tokio::test]
    async fn test_delete_wallet(#[case] expected: Result<()>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        let pin = EncryptionPin::try_from_string("1234").unwrap();

        match &expected {
            Ok(_) => {
                let mut mock_user_repo = example_get_user(SwapPaymentDetailKey::Iota, false, 2, KycType::Undefined);
                mock_user_repo.expect_update().once().returning(|_| Ok(()));

                sdk.repo = Some(Box::new(mock_user_repo));

                let mut mock_wallet_manager = MockWalletManager::new();
                mock_wallet_manager
                    .expect_delete_wallet()
                    .once()
                    .returning(|_, _, _| Ok(()));
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(mock_wallet_manager),
                });

                let new_pin = EncryptionPin::try_from_string("123456").unwrap();
                sdk.change_pin(&pin, &new_pin).await.unwrap();
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 1, 0).await;
            }
        }

        // Act
        let response = sdk.delete_wallet(&PIN).await;

        // Assert
        match expected {
            Ok(()) => response.unwrap(),
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    #[rstest]
    #[case::success(Ok(()))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[case::wallet_not_initialized(Err(crate::Error::Wallet(WalletError::WalletNotInitialized(
        ErrorKind::MissingPassword
    ))))]
    #[tokio::test]
    async fn test_verify_pin(#[case] expected: Result<()>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        match &expected {
            Ok(_) => {
                let mock_user_repo = example_get_user(SwapPaymentDetailKey::Iota, false, 1, KycType::Undefined);
                sdk.repo = Some(Box::new(mock_user_repo));

                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(MockWalletManager::new()),
                });
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 1, 0).await;
            }
        }

        // Act
        let response = sdk.verify_pin(&PIN).await;

        // Assert
        match expected {
            Ok(()) => response.unwrap(),
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    #[rstest]
    #[case::success(Ok(()))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[case::wallet_not_initialized(Err(crate::Error::Wallet(WalletError::WalletNotInitialized(
        ErrorKind::MissingPassword
    ))))]
    #[tokio::test]
    async fn test_change_pin(#[case] expected: Result<()>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        match &expected {
            Ok(_) => {
                let mut mock_user_repo = example_get_user(SwapPaymentDetailKey::Iota, false, 1, KycType::Undefined);
                mock_user_repo.expect_update().once().returning(|_| Ok(()));
                sdk.repo = Some(Box::new(mock_user_repo));

                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(MockWalletManager::new()),
                });
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 1, 0).await;
            }
        }

        // Act
        let new_pin: LazyLock<EncryptionPin> = LazyLock::new(|| EncryptionPin::try_from_string("4321").unwrap());
        let response = sdk.change_pin(&PIN, &new_pin).await;

        // Assert
        match expected {
            Ok(()) => response.unwrap(),
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    #[rstest]
    #[case::success(Ok(()))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[tokio::test]
    async fn test_set_wallet_password(#[case] expected: Result<()>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        match &expected {
            Ok(_) => {
                let mut mock_user_repo = MockUserRepo::new();
                mock_user_repo.expect_get().times(1).returning(move |r1| {
                    assert_eq!(r1, USERNAME);
                    Ok(UserEntity {
                        user_id: None,
                        username: USERNAME.into(),
                        encrypted_password: None,
                        salt: SALT.into(),
                        is_kyc_verified: false,
                        kyc_type: KycType::Undefined,
                        viviswap_state: Option::None,
                        local_share: None,
                        wallet_transactions: Vec::new(),
                    })
                });
                mock_user_repo.expect_update().once().returning(|_| Ok(()));
                sdk.repo = Some(Box::new(mock_user_repo));

                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(MockWalletManager::new()),
                });
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 1, 0).await;
            }
        }

        // Act
        let response = sdk.set_wallet_password(&PIN, &BACKUP_PASSWORD.clone()).await;

        // Assert
        match expected {
            Ok(()) => response.unwrap(),
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    #[rstest]
    #[case::success(Ok(ADDRESS.to_string()))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[tokio::test]
    async fn test_generate_new_address(#[case] expected: Result<String>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();
        let mut mock_server = None;

        match &expected {
            Ok(_) => {
                let mock_user_repo = example_get_user(SwapPaymentDetailKey::Iota, false, 1, KycType::Undefined);
                sdk.repo = Some(Box::new(mock_user_repo));

                let mut mock_wallet_manager = MockWalletManager::new();
                mock_wallet_manager.expect_try_get().returning(move |_, _, _, _, _| {
                    let mut mock_wallet_user = MockWalletUser::new();
                    mock_wallet_user
                        .expect_get_address()
                        .once()
                        .returning(|| Ok(ADDRESS.to_string()));
                    Ok(WalletBorrow::from(mock_wallet_user))
                });
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(mock_wallet_manager),
                });
                sdk.access_token = Some(TOKEN.clone());
                sdk.set_networks(example_api_networks());
                sdk.set_network(String::from("IOTA")).await.unwrap();

                let mock_request = SetUserAddressRequest {
                    address: ADDRESS.into(),
                };
                let body = serde_json::to_string(&mock_request).unwrap();

                mock_server = Some(
                    srv.mock("PUT", "/api/user/address")
                        .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
                        .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
                        .match_header("content-type", "application/json")
                        .match_query(Matcher::Exact("network_key=IOTA".to_string()))
                        .match_body(Matcher::Exact(body))
                        .with_status(201)
                        .expect(1)
                        .with_header("content-type", "application/json")
                        .create(),
                );
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 1, 0).await;
            }
        }

        // Act
        let response = sdk.generate_new_address(&PIN).await;

        // Assert
        match expected {
            Ok(resp) => {
                assert_eq!(response.unwrap(), resp);
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
    // SAFETY: we know that this value is not negative
    #[case::success(Ok(unsafe { CryptoAmount::new_unchecked(dec!(25.0)) }))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[tokio::test]
    async fn test_get_balance(#[case] expected: Result<CryptoAmount>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        match &expected {
            Ok(_) => {
                let mock_user_repo = example_get_user(SwapPaymentDetailKey::Iota, false, 1, KycType::Undefined);
                sdk.repo = Some(Box::new(mock_user_repo));

                let mut mock_wallet_manager = MockWalletManager::new();
                mock_wallet_manager.expect_try_get().returning(move |_, _, _, _, _| {
                    let mut mock_wallet_user = MockWalletUser::new();
                    mock_wallet_user
                        .expect_get_balance()
                        .once()
                        // SAFETY: we know that this value is not negative
                        .returning(|| Ok(unsafe { CryptoAmount::new_unchecked(dec!(25.0)) }));
                    Ok(WalletBorrow::from(mock_wallet_user))
                });
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(mock_wallet_manager),
                });
                sdk.set_networks(example_api_networks());
                sdk.set_network(String::from("IOTA")).await.unwrap();
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 1, 0).await;
            }
        }

        // Act
        let response = sdk.get_balance(&PIN).await;

        // Assert
        match expected {
            Ok(resp) => {
                assert_eq!(response.unwrap(), resp);
            }
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    #[rstest]
    #[case::success(Ok(example_wallet_tx_info()))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[tokio::test]
    async fn test_get_wallet_tx(#[case] expected: Result<WalletTxInfo>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        match &expected {
            Ok(_) => {
                let mock_user_repo = example_get_user(SwapPaymentDetailKey::Iota, false, 1, KycType::Undefined);
                sdk.repo = Some(Box::new(mock_user_repo));

                let mut mock_wallet_manager = MockWalletManager::new();
                mock_wallet_manager.expect_try_get().returning(move |_, _, _, _, _| {
                    let mut mock_wallet_user = MockWalletUser::new();
                    mock_wallet_user
                        .expect_get_wallet_tx()
                        .once()
                        .returning(|_| Ok(example_wallet_tx_info()));
                    Ok(WalletBorrow::from(mock_wallet_user))
                });
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(mock_wallet_manager),
                });
                sdk.set_networks(example_api_networks());
                sdk.set_network(String::from("IOTA")).await.unwrap();
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 1, 0).await;
            }
        }

        // Act
        let response = sdk.get_wallet_tx(&PIN, TX_INDEX).await;

        // Assert
        match expected {
            Ok(resp) => {
                assert_eq!(response.unwrap(), resp);
            }
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    #[rstest]
    #[case::success(Ok(WalletTxInfoList { transactions: vec![example_wallet_tx_info()]}))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[tokio::test]
    async fn test_get_wallet_tx_list(#[case] expected: Result<WalletTxInfoList>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        match &expected {
            Ok(_) => {
                let mock_user_repo = example_get_user(SwapPaymentDetailKey::Iota, false, 2, KycType::Undefined);
                sdk.repo = Some(Box::new(mock_user_repo));

                let mut mock_wallet_manager = MockWalletManager::new();
                mock_wallet_manager.expect_try_get().returning(move |_, _, _, _, _| {
                    let mut mock_wallet_user = MockWalletUser::new();
                    mock_wallet_user.expect_get_wallet_tx_list().once().returning(|_, _| {
                        Ok(WalletTxInfoList {
                            transactions: vec![example_wallet_tx_info()],
                        })
                    });
                    Ok(WalletBorrow::from(mock_wallet_user))
                });
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(mock_wallet_manager),
                });
                sdk.set_networks(example_api_networks());
                sdk.set_network(String::from("IOTA")).await.unwrap();
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 2, 0).await;
            }
        }

        // Act
        let response = sdk.get_wallet_tx_list(&PIN, 0, 10).await;

        // Assert
        match expected {
            Ok(resp) => {
                assert_eq!(response.unwrap(), resp);
            }
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }
}
