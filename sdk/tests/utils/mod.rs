#![allow(clippy::unwrap_used, clippy::expect_used)]
#![allow(dead_code)]

use api_types::api::networks::{ApiNetwork, ApiProtocol};
use etopay_sdk::{
    core::{Config, Sdk},
    types::newtypes::{AccessToken, EncryptionPin, PlainPassword},
};
use std::path::Path;
use testing::CleanUp;

/// initialize sdk from an existing [`CleanUp`] object as the storage path.
pub async fn init_sdk_with_cleanup(username: &str, existing_cleanup: CleanUp) -> (Sdk, CleanUp) {
    dotenvy::dotenv().ok();

    let password = std::env::var("SATOSHI_PASSWORD").unwrap();

    let backend_url =
        std::env::var("RT_API_URL").expect("RT_API_URL should be set with the backend url for the tests to use");

    // construct the config to use for the SDK
    let config = Config {
        backend_url: backend_url.parse().expect("RT_API_URL must be a valid URL"),
        path_prefix: Path::new(&existing_cleanup.path_prefix).into(),
        auth_provider: "standalone".to_string(),
        log_level: log::LevelFilter::Debug,
    };

    let mut sdk = Sdk::new(config).expect("should not fail to initialize sdk"); // set the backend url if the environment variable is set

    // set networks
    sdk.set_networks(vec![
        ApiNetwork {
            key: String::from("IOTA"),
            display_name: String::from("IOTA"),
            display_symbol: String::from("IOTA"),
            coin_type: 4218,
            node_urls: vec![String::from("https://api.testnet.iotaledger.net")],
            decimals: 16,
            can_do_purchases: true,
            protocol: ApiProtocol::Stardust {},
            block_explorer_url: String::from("https://explorer.iota.org/iota-testnet/"),
        },
        ApiNetwork {
            key: String::from("IOTA"),
            display_name: String::from("Eth Sepolia"),
            display_symbol: String::from("ETH"),
            coin_type: 60,
            node_urls: vec![String::from("https://sepolia.mode.network")],
            decimals: 16,
            can_do_purchases: true,
            protocol: ApiProtocol::Evm { chain_id: 31337 },
            block_explorer_url: String::from("https://explorer.shimmer.network/testnet/"),
        },
    ]);
    sdk.set_network(String::from("IOTA")).await.unwrap();

    // generate access token
    let access_token = testing::get_access_token(username, &password).await.access_token;
    let access_token = AccessToken::try_from(access_token).unwrap();
    sdk.refresh_access_token(Some(access_token)).await.unwrap();

    (sdk, existing_cleanup)
}

/// initialize sdk with a new [`CleanUp`] object as the storage path.
pub async fn init_sdk(username: &str) -> (Sdk, CleanUp) {
    init_sdk_with_cleanup(username, CleanUp::default()).await
}

/// A copy of the [`testing::TestUser`] that uses the newtype wrappers for sensitive values for
/// easier use in the examples.
#[derive(Debug)]
pub struct TestUser {
    pub username: String,
    pub password: PlainPassword,
    pub pin: EncryptionPin,
    pub mnemonic: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub date_of_birth: String,
    pub iban: String,
}

impl From<testing::TestUser> for TestUser {
    fn from(value: testing::TestUser) -> Self {
        Self {
            username: value.username,
            password: PlainPassword::try_from_string(value.password).unwrap(),
            pin: EncryptionPin::try_from_string(value.pin).unwrap(),
            mnemonic: value.mnemonic,
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            date_of_birth: value.date_of_birth,
            iban: value.iban,
        }
    }
}
