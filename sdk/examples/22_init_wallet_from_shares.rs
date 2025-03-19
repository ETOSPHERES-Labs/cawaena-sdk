use etopay_sdk::{ErrorKind, WalletError};
use testing::USER_SATOSHI;
use utils::init_sdk;
mod utils;

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Initialize SDK
    let (mut sdk, _cleanup) = init_sdk().await;
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();

    // Create new user
    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();

    // Fetch networks from backend
    let networks = sdk.get_networks().await.unwrap();
    let iota_network_key = &networks.first().unwrap().key;
    sdk.set_network(iota_network_key.to_string()).await.unwrap();

    // use wallet without creating a new one first
    let output = sdk.generate_new_address(&user.pin).await;

    match output {
        Ok(_address) => {
            println!("Wallet initialized successfully");
        }

        Err(etopay_sdk::Error::Wallet(WalletError::WalletNotInitialized(ErrorKind::MissingPassword))) => {
            // Wallet requires a password, try again with the password provided
            sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();

            let result = sdk.generate_new_address(&user.pin).await;
            if result.is_ok() {
                println!("Wallet initialized successfully with password set");
            } else {
                panic!("Unexpected result after providing password: {:?}", result);
            }
        }

        Err(etopay_sdk::Error::Wallet(WalletError::WalletNotInitialized(ErrorKind::SetRecoveryShare))) => {
            // Ask user for recovery share
            let share = "<User Input>".parse().unwrap();

            sdk.set_recovery_share(share).await.unwrap();

            let result = sdk.generate_new_address(&user.pin).await;
            if result.is_ok() {
                println!("Wallet initialized successfully with recovery share");
            } else {
                panic!("Unexpected result after setting recovery share: {:?}", result);
            }
        }

        Err(etopay_sdk::Error::Wallet(WalletError::WalletNotInitialized(ErrorKind::UseMnemonic))) => {
            sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();
            sdk.create_wallet_from_existing_mnemonic(&user.pin, &user.mnemonic)
                .await
                .unwrap();

            let result = sdk.generate_new_address(&user.pin).await;
            if result.is_ok() {
                println!("Wallet initialized successfully from mnemonic");
            } else {
                panic!("Unexpected result after creating wallet from mnemonic: {:?}", result);
            }
        }
        other => panic!("unexpected result: {other:?}"),
    }

    // Ensure the wallet is functioning
    let address = sdk.generate_new_address(&user.pin).await.unwrap();
    let balance = sdk.get_balance(&user.pin).await.unwrap();
    println!("New address : {address} , Wallet balance: , {balance:?}");
}
