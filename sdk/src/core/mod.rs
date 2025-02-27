//! Main SDK module.

/// Account module.
#[cfg(feature = "billing")]
pub mod account;
/// Config module.
pub mod config;

/// Postident module.
#[cfg(feature = "postident")]
pub mod postident;

/// Transaction module.
pub mod transaction;

/// User module.
pub mod user;
/// Viviswap module.
pub mod viviswap;
/// Wallet module.
pub mod wallet;

/// Exchange module.
pub mod exchange;

/// Share module.
pub mod share;

/// Testing utils in sdk core
#[cfg(test)]
pub(crate) mod core_testing_utils;

use crate::build;
use crate::error::Result;
use crate::types::currencies::Currency;
use crate::types::networks::Network;
use crate::types::newtypes::{AccessToken, EncryptionPin};
use crate::types::users::ActiveUser;
use crate::user::UserRepo;
use crate::wallet_manager::WalletBorrow;
pub use config::Config;
use log::debug;

pub(crate) type UserRepoT = Box<dyn UserRepo + Send + Sync + 'static>;

/// Struct representing the SDK and its core components including configuration, user management, and storage options.
pub struct Sdk {
    /// Contains SDK configuration.
    config: Option<Config>,
    /// Contains the initialized active user.
    active_user: Option<ActiveUser>,
    /// Contains the access token for various SDK operations.
    access_token: Option<AccessToken>,
    /// Contains the user repository for storing and loading different users.
    repo: Option<UserRepoT>,
    /// The currently active coin currency
    currency: Option<Currency>,
    /// The currently active network
    network: Option<Network>,
}

impl Drop for Sdk {
    /// Drop implementation for SDK
    fn drop(&mut self) {
        debug!("Dropping SDK");
    }
}

impl Default for Sdk {
    /// Default implementation for SDK
    fn default() -> Self {
        Self {
            config: None,
            active_user: None,
            access_token: None,
            repo: None,
            currency: None,
            network: None,
        }
    }
}

impl Sdk {
    /// Initialize an SDK instance from a config
    pub fn new(config: Config) -> Result<Self> {
        debug!("Configuration: {:?}", config);
        let mut s = Self::default();
        s.set_config(config)?;
        Ok(s)
    }

    /// Set network
    pub fn set_network(&mut self, network: Network) {
        debug!("Selected Network: {:?}", network);
        self.network = Some(network);
    }

    /// Tries to get the wallet of the currently active user. Or returns an error if no user is
    /// initialized, or if creating the wallet fails.
    ///
    /// Note: this will borrow `self` as mutable, and thus is not usable in cases when you want
    /// to call functions that take `&mut self` as receiver while holding on to the
    /// [`WalletBorrow`]
    async fn try_get_active_user_wallet(&mut self, pin: &EncryptionPin) -> Result<WalletBorrow<'_>> {
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
            .try_get(config, &self.access_token, repo, network, pin)
            .await?;
        Ok(wallet)
    }

    /// A function that returns a multi-line String containing:
    /// * Branch name       (e.g. main)
    /// * Commit hash       (e.g. 92cedead),
    /// * Build time        (e.g. 2024-10-29 12:10:09 +00:00),
    /// * Rust version      (e.g. 1.80.1 (3f5fd8dd4 2024-08-06))
    /// * Toolchain channel (e.g. stable-x86_64-unknown-linux-gnu)
    pub fn get_build_info() -> String {
        build::CLAP_LONG_VERSION.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::core_testing_utils::handle_error_test_cases;
    use crate::testing_utils::example_network;
    use crate::{
        core::Sdk,
        error::Result,
        testing_utils::{example_wallet_borrow, set_config, PIN, USERNAME},
        user::MockUserRepo,
        wallet_manager::WalletBorrow,
        wallet_user::MockWalletUser,
    };
    use rstest::rstest;

    #[rstest]
    #[case::success(Ok(WalletBorrow::from(MockWalletUser::new())))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[tokio::test]
    async fn test_try_get_active_user_wallet(#[case] expected: Result<WalletBorrow<'_>>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        match &expected {
            Ok(_) => {
                sdk.repo = Some(Box::new(MockUserRepo::new()));
                let mock_wallet_manager = example_wallet_borrow();
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(mock_wallet_manager),
                });
                sdk.set_network(example_network(crate::types::currencies::Currency::Iota));
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 0, 0).await;
            }
        }

        // Act
        let response = Sdk::try_get_active_user_wallet(&mut sdk, &PIN).await;

        // Assert
        match expected {
            Ok(_wallet_borrow) => {
                response.unwrap();
            }
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    #[test]
    fn test_get_build_info() {
        let build_info = Sdk::get_build_info();
        assert!(!build_info.is_empty());
        println!("{build_info}");
    }
}
