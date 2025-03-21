use api_types::api::transactions::ApiTxStatus;
use etopay_sdk::types::{currencies::CryptoAmount, newtypes::AccessToken};
use rust_decimal_macros::dec;
use std::time::Duration;
use testing::USER_HANS34;
mod utils;
use tokio::time;
use utils::init_sdk;

#[allow(clippy::unwrap_used)]
#[tokio::main]
async fn main() {
    // Initialize SDK for sender, create new user and migrate wallet
    let (mut sdk, _cleanup) = init_sdk().await;
    let user: utils::TestUser = (*USER_HANS34).clone().into();

    // the `init_sdk()` function generates an access token for `satoshi`.
    // in this example we use `hans34` user. Therefore, we generate a new access token for the `hans34` user.
    let access_token = testing::get_access_token(&user.username, user.password.as_str())
        .await
        .access_token;
    let access_token = AccessToken::try_from(access_token).unwrap();
    sdk.refresh_access_token(Some(access_token)).await.unwrap();

    // Create new user
    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();

    // Create / init new wallet from mnemonic
    sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();
    sdk.create_wallet_from_existing_mnemonic(&user.pin, &user.mnemonic)
        .await
        .unwrap();

    // Fetch networks from backend
    let networks = sdk.get_networks().await.unwrap();
    let iota_network_key = &networks.first().unwrap().key;
    sdk.set_network(iota_network_key.to_string()).await.unwrap();

    // Generate address and get balance
    let address = sdk.generate_new_address(&user.pin).await.unwrap(); // this is needed, otherwise the balance will be 0 and tx will fail
    let balance = sdk.get_balance(&user.pin).await.unwrap();
    println!("Balance: {:#?} on address {}", balance, address);

    // Create purchase request
    let product_hash = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824";
    let app_data = "{\"imageUrl\":\"https://httpbin.org/\",\"imageId\":\"a846ad10-fc69-4b22-b442-5dd740ace361\"}";
    let purchase_type = "CLIK";

    let amount = CryptoAmount::try_from(dec!(2.0)).unwrap();
    let purchase_id = sdk
        .create_purchase_request("alice", amount, product_hash, app_data, purchase_type)
        .await
        .unwrap();
    println!("Purchase_id {} ", purchase_id); // print the purchase id to facilitate debugging

    // Wait 3 min while tx status becomes valid
    let result = time::timeout(Duration::from_secs(3 * 60), async {
        loop {
            time::sleep(Duration::from_secs(5)).await;
            let details = sdk.get_purchase_details(&purchase_id).await.unwrap();
            match details.status {
                ApiTxStatus::Valid => {
                    println!("Purchase request valid, moving on...");
                    break;
                }
                ApiTxStatus::Invalid(r) => {
                    panic!("Purchase request invalid! Reason: {:?}. Exiting", r);
                }
                ApiTxStatus::WaitingForVerification(r) => {
                    panic!("Purchase request waiting for verification! Reason: {:?}.", r);
                }
                _ => {}
            }
        }
    })
    .await;
    if result.is_err() {
        panic!("Timeout reached while waiting for purchase request to become valid");
    }

    // Step 4: Confirm purchase request (perform actual wallet transaction)
    sdk.confirm_purchase_request(&user.pin, &purchase_id).await.unwrap();

    // Wait 3 min while tx status becomes completed
    let result = time::timeout(Duration::from_secs(3 * 60), async {
        loop {
            time::sleep(Duration::from_secs(5)).await;
            let status = sdk.get_purchase_details(&purchase_id).await.unwrap().status;
            println!(" - Status: {:?}", status);
            if status == ApiTxStatus::Completed {
                println!("Purchase request completed, done!");
                break;
            } else if status == ApiTxStatus::Failed {
                panic!("Purchase request failed");
            }
        }
    })
    .await;
    if result.is_err() {
        panic!("Timeout reached while waiting for purchase request to complete");
    }

    // Check new balance
    let balance = sdk.get_balance(&user.pin).await.unwrap();
    println!("New Balance: {:#?}", balance);
}
