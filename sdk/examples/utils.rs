#![allow(clippy::unwrap_used, clippy::expect_used, dead_code)]

use sdk::{
    core::{Config, Sdk},
    types::{
        networks::{Network, NetworkType},
        newtypes::{AccessToken, EncryptionPin, PlainPassword},
    },
};
use std::{collections::HashMap, path::Path};
use testing::{CleanUp, USER_SATOSHI};

pub async fn init_sdk() -> (Sdk, CleanUp) {
    dotenvy::dotenv().ok();

    // for the examples we want logs to go to the console for easier troubleshooting
    env_logger::builder().filter_level(log::LevelFilter::Info).init();

    let user = &USER_SATOSHI;

    let cleanup = CleanUp::default();

    let backend_url = std::env::var("EXAMPLES_BACKEND_URL")
        .expect("EXAMPLES_BACKEND_URL environment variable need to be set to run the examples");

    // construct the config to use for the SDK
    let config = Config {
        backend_url: backend_url.parse().expect("EXAMPLES_BACKEND_URL must be a valid URL"),
        path_prefix: Path::new(&cleanup.path_prefix).into(),
        auth_provider: "standalone".to_string(),
        log_level: log::LevelFilter::Debug,
    };
    let mut sdk = Sdk::new(config).expect("should not fail to initialize sdk"); // set the backend url if the environment variable is set

    // generate access token
    let access_token = testing::get_access_token(&user.username, &user.password)
        .await
        .access_token;
    let access_token = AccessToken::try_from(access_token).unwrap();
    sdk.refresh_access_token(Some(access_token)).await.unwrap();

    sdk.set_network(String::from("67a1f08edf55756bae21e7eb"))
        .await
        .expect("should not fail to set network");

    (sdk, cleanup)
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

// This module needs a `main` function so we add an empty one here.
#[tokio::main]
async fn main() {}
