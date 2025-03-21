use super::Sdk;
use crate::testing_utils::{example_api_networks, SALT, TOKEN};
use crate::types::users::UserEntity;
use crate::wallet_manager::MockWalletManager;
use crate::{
    testing_utils::{example_get_user, USERNAME},
    types::users::KycType,
    user::MockUserRepo,
};
use crate::{ErrorKind, WalletError};
use api_types::api::viviswap::detail::SwapPaymentDetailKey;

pub async fn handle_error_test_cases(
    error: &crate::Error,
    sdk: &mut Sdk,
    config_error_get_user_mock_call_times: usize,
    token_error_get_user_mock_call_times: usize,
) {
    match error {
        crate::Error::UserRepoNotInitialized => {
            sdk.repo = None;
        }
        crate::Error::UserNotInitialized => {
            sdk.repo = Some(Box::new(MockUserRepo::new()));
            sdk.active_user = None;
        }
        crate::Error::MissingConfig => {
            sdk.set_networks(example_api_networks());
            sdk.set_network(String::from("IOTA")).await.unwrap();
            let mock_user_repo = example_get_user(
                SwapPaymentDetailKey::Iota,
                false,
                config_error_get_user_mock_call_times,
                KycType::Undefined,
            );
            sdk.repo = Some(Box::new(mock_user_repo));
            sdk.active_user = Some(crate::types::users::ActiveUser {
                username: USERNAME.into(),
                wallet_manager: Box::new(MockWalletManager::new()),
            });
            sdk.access_token = Some(TOKEN.clone());
            sdk.config = None;
        }
        crate::Error::MissingNetwork => {
            let mock_user_repo = example_get_user(
                SwapPaymentDetailKey::Iota,
                false,
                config_error_get_user_mock_call_times,
                KycType::Undefined,
            );
            sdk.repo = Some(Box::new(mock_user_repo));
            sdk.active_user = Some(crate::types::users::ActiveUser {
                username: USERNAME.into(),
                wallet_manager: Box::new(MockWalletManager::new()),
            });
            sdk.set_networks(example_api_networks());
            sdk.network = None;
            sdk.access_token = Some(TOKEN.clone());
            sdk.config = None;
        }
        crate::Error::MissingAccessToken => {
            let mock_user_repo = example_get_user(
                SwapPaymentDetailKey::Iota,
                false,
                token_error_get_user_mock_call_times,
                KycType::Undefined,
            );
            sdk.repo = Some(Box::new(mock_user_repo));
            sdk.active_user = Some(crate::types::users::ActiveUser {
                username: USERNAME.into(),
                wallet_manager: Box::new(MockWalletManager::new()),
            });

            sdk.access_token = None;
        }
        crate::Error::Wallet(WalletError::WalletNotInitialized(ErrorKind::MissingPassword)) => {
            let mut mock_user_repo = MockUserRepo::new();
            mock_user_repo.expect_get().times(1).returning(move |r1| {
                assert_eq!(r1, USERNAME);
                Ok(UserEntity {
                    user_id: None,
                    username: USERNAME.to_string(),
                    encrypted_password: None,
                    salt: SALT.into(),
                    is_kyc_verified: true,
                    kyc_type: KycType::Undefined,
                    viviswap_state: None,
                    local_share: None,
                    wallet_transactions: Vec::new(),
                })
            });
            sdk.repo = Some(Box::new(mock_user_repo));
            sdk.active_user = Some(crate::types::users::ActiveUser {
                username: USERNAME.into(),
                wallet_manager: Box::new(MockWalletManager::new()),
            });
        }
        crate::Error::UserRepository(crate::user::error::UserKvStorageError::UserNotFound { .. }) => {
            let mut mock_user_repo = MockUserRepo::new();
            mock_user_repo.expect_get().times(1).returning(move |_r1| {
                Err(crate::user::error::UserKvStorageError::UserNotFound {
                    username: USERNAME.to_string(),
                })
            });
            sdk.repo = Some(Box::new(mock_user_repo));

            sdk.active_user = Some(crate::types::users::ActiveUser {
                username: USERNAME.into(),
                wallet_manager: Box::new(MockWalletManager::new()),
            });
        }
        other => panic!("Got unexpected or unhandled result: {:?}", other),
    }
}
