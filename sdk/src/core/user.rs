//! This module defines methods for interacting with user-related functionality,
//! such as getting user state, creating a new user, deleting a user, and more.

use super::Sdk;
use crate::backend;
use crate::backend::kyc::check_kyc_status;
use crate::error::Result;
use crate::types::newtypes::AccessToken;
use crate::types::newtypes::EncryptionPin;
use crate::types::newtypes::EncryptionSalt;
use crate::types::users::{ActiveUser, KycType, UserEntity};
use log::{debug, info, warn};

impl Sdk {
    /// Get user entity
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the user entity (`UserEntity`) if successful, or an `Error` if an error occurs.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if there is an issue initializing the user or accessing the repository.
    pub async fn get_user(&self) -> Result<UserEntity> {
        debug!("Getting the user");
        let Some(repo) = &self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };

        // load active user
        let Some(active_user) = &self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        // load user state
        Ok(repo.get(active_user.username.as_str())?)
    }

    /// Create a new user
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the new user.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the user is created successfully, or an `Error` if an error occurs.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if there is an issue validating the configuration, initializing the repository, or creating the user.
    pub async fn create_new_user(&mut self, username: &str) -> Result<()> {
        info!("Creating a new user");
        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };

        let salt = EncryptionSalt::generate();
        let user = UserEntity {
            user_id: None,
            username: username.into(),
            encrypted_password: None,
            salt,
            is_kyc_verified: false,
            kyc_type: KycType::Undefined,
            viviswap_state: Option::None,
            local_share: None,
            wallet_transactions: Vec::new(),
        };

        repo.create(&user)?;

        Ok(())
    }

    /// Delete the currently active user and their wallet
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN of the user to be deleted.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the user is deleted successfully, or an `Error` if an error occurs.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if there is an issue verifying the PIN, initializing the repository, initiliazing the user, deleting the user, or deleting the wallet.
    pub async fn delete_user(&mut self, pin: Option<&EncryptionPin>) -> Result<()> {
        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;

        let user_entity = self.get_user().await?;

        // make sure the pin is correct before continuing, only if the wallet exists
        if user_entity.encrypted_password.is_some() {
            let pin = pin.ok_or(crate::Error::Wallet(crate::WalletError::WrongPinOrPassword))?;
            self.verify_pin(pin).await?;
            info!("Pin verified");
        }

        let Some(active_user) = &mut self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        warn!("Deleting an existing user");

        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };

        let username = &active_user.username;

        // Call the delete endpoint on the backend first to avoid ending up in an inconsistent state
        let access_token = self
            .access_token
            .as_ref()
            .ok_or(crate::error::Error::MissingAccessToken)?;
        crate::backend::user::delete_user_account(config, access_token).await?;

        // Delete the user in the repo
        repo.delete(username)?;

        // take the wallet out of the Option and try to delete it, it will then be dropped
        if let Err(e) = active_user
            .wallet_manager
            .delete_wallet(config, &self.access_token, repo)
            .await
        {
            log::error!("Error deleting the wallet: {e:?}");
        }

        Ok(())
    }

    /// Initialize an user
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the user to initialize.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the user is initialized successfully, or an `Error` if an error occurs.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if there is an issue validating the configuration, initializing the repository, or checking the KYC status.
    pub async fn init_user(&mut self, username: &str) -> Result<()> {
        info!("Initializing user {username}");
        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };
        let user = repo.get(username)?;
        let active_user = ActiveUser::from(user);

        if let Some(access_token) = &self.access_token {
            let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;
            let status = check_kyc_status(config, access_token, username).await?;
            repo.set_kyc_state(username, status.is_verified)?;
        }

        self.active_user = Some(active_user);

        Ok(())
    }

    /// Refresh access token
    ///
    /// # Arguments
    ///
    /// * `access_token` - The new access token to be set. Or `None` to unset it.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the access token is refreshed successfully, or an `Error` if an error occurs.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if there is an issue validating the configuration.
    pub async fn refresh_access_token(&mut self, access_token: Option<AccessToken>) -> Result<()> {
        self.access_token = access_token;

        Ok(())
    }

    /// Check if KYC status is verified.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the user to check KYC status for.
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if the KYC status is verified, or `Ok(false)` if it is not verified.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if there is an issue validating the configuration, initializing the repository, or checking the KYC status.
    pub async fn is_kyc_status_verified(&mut self, username: &str) -> Result<bool> {
        info!("Checking KYC status of user {username}");
        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };
        let access_token = self
            .access_token
            .as_ref()
            .ok_or(crate::error::Error::MissingAccessToken)?;

        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;
        let status = check_kyc_status(config, access_token, username).await?;
        let is_verified = status.is_verified;

        // Store user state in db if is verified
        if is_verified {
            repo.set_kyc_state(username, is_verified)?;
        }

        Ok(is_verified)
    }

    /// Set the user preferred network
    pub async fn set_preferred_network(&mut self, network_key: Option<String>) -> Result<()> {
        let Some(_user) = &self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };
        let access_token = self
            .access_token
            .as_ref()
            .ok_or(crate::error::Error::MissingAccessToken)?;
        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;
        backend::user::set_preferred_network(config, access_token, network_key).await?;
        Ok(())
    }

    /// Get the user preferred network
    pub async fn get_preferred_network(&self) -> Result<Option<String>> {
        let Some(_user) = &self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };
        let access_token = self
            .access_token
            .as_ref()
            .ok_or(crate::error::Error::MissingAccessToken)?;
        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;
        let preferred_network = backend::user::get_preferred_network(config, access_token).await?;
        Ok(preferred_network)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::core_testing_utils::handle_error_test_cases;
    use crate::testing_utils::{example_get_user, set_config, AUTH_PROVIDER, HEADER_X_APP_NAME, TOKEN, USERNAME};
    use crate::{core::Sdk, user::MockUserRepo, wallet_manager::MockWalletManager};
    use api_types::api::kyc::KycStatusResponse;
    use api_types::api::viviswap::detail::SwapPaymentDetailKey;
    use rstest::rstest;

    #[rstest]
    #[case::success(Ok(example_get_user(SwapPaymentDetailKey::Iota, false, 0, KycType::Undefined)))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[case::user_not_found(Err(crate::Error::UserRepository(crate::user::error::UserKvStorageError::UserNotFound { username: USERNAME.to_string() })))]
    #[tokio::test]
    async fn test_get_user(#[case] expected: Result<MockUserRepo>) {
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
                handle_error_test_cases(error, &mut sdk, 0, 0).await;
            }
        }

        // Act
        let response = sdk.get_user().await;

        // Assert
        match expected {
            Ok(_resp) => {
                assert!(response.is_ok());
            }
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    #[rstest]
    #[case::success(Ok(()))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[tokio::test]
    async fn test_create_new_user(#[case] expected: Result<()>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        match &expected {
            Ok(_) => {
                let mut mock_user_repo = MockUserRepo::new();
                mock_user_repo.expect_create().times(1).returning(|_| Ok(()));

                sdk.repo = Some(Box::new(mock_user_repo));

                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(MockWalletManager::new()),
                });
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 0, 0).await;
            }
        }

        // Act
        let response = sdk.create_new_user("new_user").await;

        // Assert
        match expected {
            Ok(_) => response.unwrap(),
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    #[rstest]
    #[case::success(Ok(()))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[case::unauthorized(Err(crate::Error::MissingAccessToken))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[tokio::test]
    async fn test_delete_user(#[case] expected: Result<()>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        let pin = EncryptionPin::try_from_string("1234").unwrap();

        let mut mock_server = None;
        match &expected {
            Ok(_) => {
                let mut mock_wallet_user = MockWalletManager::new();
                mock_wallet_user
                    .expect_delete_wallet()
                    .once()
                    .returning(|_, _, _| Ok(()));

                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(mock_wallet_user),
                });

                sdk.access_token = Some(TOKEN.clone());

                let mut mock_user_repo = example_get_user(SwapPaymentDetailKey::Iota, false, 3, KycType::Undefined);
                mock_user_repo.expect_update().once().returning(|_| Ok(()));
                mock_user_repo.expect_delete().once().returning(|uname| {
                    assert_eq!(uname, USERNAME);
                    Ok(())
                });
                sdk.repo = Some(Box::new(mock_user_repo));

                let new_pin = EncryptionPin::try_from_string("123456").unwrap();
                sdk.change_pin(&pin, &new_pin).await.unwrap();

                mock_server = Some(
                    srv.mock("DELETE", "/api/user")
                        .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
                        .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
                        .with_status(202) // Accepted
                        .expect(1)
                        .create(),
                );
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 0, 2).await;
            }
        }

        // Act
        let response = sdk.delete_user(Some(&pin)).await;

        // Assert
        match expected {
            Ok(_) => response.unwrap(),
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
        if let Some(m) = mock_server {
            m.assert();
        }
    }

    #[rstest]
    #[case::success(true, Ok(()))]
    #[case::repo_init_error(false, Err(crate::Error::UserRepoNotInitialized))]
    #[case::missing_config(false, Err(crate::Error::MissingConfig))]
    #[tokio::test]
    async fn test_init_user(#[case] _access_token: bool, #[case] expected: Result<()>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        let mut mock_server = None;
        match &expected {
            Ok(_) => {
                let mut mock_user_repo = example_get_user(SwapPaymentDetailKey::Iota, false, 1, KycType::Undefined);
                mock_user_repo.expect_set_kyc_state().times(1).returning(|_, _| Ok(()));
                sdk.repo = Some(Box::new(mock_user_repo));

                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(MockWalletManager::new()),
                });

                sdk.access_token = Some(TOKEN.clone());

                let mock_response = KycStatusResponse {
                    username: USERNAME.into(),
                    is_verified: true,
                };
                let body = serde_json::to_string(&mock_response).unwrap();

                mock_server = Some(
                    srv.mock("GET", "/api/kyc/check-status")
                        .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
                        .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
                        .with_status(200)
                        .with_body(body)
                        .create(),
                );
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 1, 0).await;
            }
        }

        // Act
        let response = sdk.init_user(USERNAME).await;

        // Assert
        match expected {
            Ok(_) => response.unwrap(),
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
        if let Some(m) = mock_server {
            m.assert();
        }
    }

    #[rstest]
    #[case::success(Ok(true))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::unauthorized(Err(crate::Error::MissingAccessToken))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[tokio::test]
    async fn test_is_kyc_verified(#[case] expected: Result<bool>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();
        let mut mock_server = None;

        match &expected {
            Ok(_) => {
                let mut mock_user_repo = example_get_user(SwapPaymentDetailKey::Iota, false, 0, KycType::Undefined);
                mock_user_repo.expect_set_kyc_state().times(1).returning(|_, _| Ok(()));
                sdk.repo = Some(Box::new(mock_user_repo));

                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(MockWalletManager::new()),
                });

                sdk.access_token = Some(TOKEN.clone());

                let mock_response = KycStatusResponse {
                    username: USERNAME.into(),
                    is_verified: true,
                };
                let body = serde_json::to_string(&mock_response).unwrap();

                mock_server = Some(
                    srv.mock("GET", "/api/kyc/check-status")
                        .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
                        .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
                        .with_status(200)
                        .with_body(body)
                        .create(),
                );
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 0, 0).await;
            }
        }

        // Act
        let response = sdk.is_kyc_status_verified(USERNAME).await;

        // Assert
        match expected {
            Ok(verified) => assert!(verified),
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
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[case::unauthorized(Err(crate::Error::MissingAccessToken))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[tokio::test]
    async fn test_set_preferred_network(#[case] expected: Result<()>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();
        let mut mock_server = None;

        match &expected {
            Ok(_) => {
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(MockWalletManager::new()),
                });
                sdk.access_token = Some(TOKEN.clone());

                mock_server = Some(
                    srv.mock("PUT", "/api/user/network")
                        .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
                        .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
                        .with_status(202)
                        .expect(1)
                        .create(),
                );
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 0, 0).await;
            }
        }

        // Act
        let response = sdk.set_preferred_network(Some(String::from("IOTA"))).await;

        // Assert
        match expected {
            Ok(()) => response.unwrap(),
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
        if let Some(m) = mock_server {
            m.assert();
        }
    }

    #[rstest]
    #[case::success(Ok(Some(String::from("IOTA"))))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[case::unauthorized(Err(crate::Error::MissingAccessToken))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[tokio::test]
    async fn test_get_preferred_network(#[case] expected: Result<Option<String>>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();
        let mut mock_server = None;

        match &expected {
            Ok(_) => {
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(MockWalletManager::new()),
                });
                sdk.access_token = Some(TOKEN.clone());

                mock_server = Some(
                    srv.mock("GET", "/api/user/network")
                        .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
                        .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
                        .with_status(200)
                        .with_header("content-type", "application/json")
                        .with_body("{\"network_key\":\"IOTA\"}")
                        .expect(1)
                        .create(),
                );
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 0, 0).await;
            }
        }

        // Act
        let response = sdk.get_preferred_network().await;

        // Assert
        match expected {
            Ok(network_key) => {
                assert_eq!(response.unwrap(), network_key)
            }
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
        if let Some(m) = mock_server {
            m.assert();
        }
    }
}
