mod utils;
use crate::utils::init_sdk;

use api_types::api::transactions::ApiTxStatus;
use etopay_sdk::{
    core::{Config, Sdk},
    types::{currencies::CryptoAmount, newtypes::AccessToken},
};
use rust_decimal_macros::dec;
use std::{path::Path, time::Duration};
use testing::{CleanUp, USER_HANS34, USER_HANS48, USER_SATOSHI};
use tokio::time;

#[tokio::test]
async fn it_should_get_tx_details() {
    // Arrange
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;

    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();
    sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();
    sdk.create_wallet_from_existing_mnemonic(&user.pin, &user.mnemonic)
        .await
        .unwrap();
    // this specific request is created in the `init_db` test
    let purchase_id = "94nfgd3l-a0b8-kg40-a928-bbebc401ac1b";

    // Act
    let result = sdk.get_purchase_details(purchase_id).await;

    // Assert
    let purchase_status = result.unwrap().status;
    assert_eq!(purchase_status, ApiTxStatus::Valid);
}

#[tokio::test]
async fn it_should_send_amount() {
    // Arrange
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;

    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();
    sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();
    sdk.create_wallet_from_existing_mnemonic(&user.pin, &user.mnemonic)
        .await
        .unwrap();

    let recipient_address = sdk.generate_new_address(&user.pin).await.unwrap();
    let amount = dec!(2.0).try_into().unwrap();

    // Act
    let result = sdk.send_amount(&user.pin, &recipient_address, amount, None).await;

    //Assert
    result.unwrap();
}

#[tokio::test]
async fn it_should_get_tx_list() {
    // Arrange
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;

    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();
    sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();
    sdk.create_wallet_from_existing_mnemonic(&user.pin, &user.mnemonic)
        .await
        .unwrap();

    // Act
    let result = sdk.get_tx_list(0, 10).await;

    // Assert
    let tx_list = result.unwrap();
    assert!(!tx_list.txs.is_empty());
}

#[tokio::test]
#[ignore = "There are some blockers around this and IOTA is also changing. We disable this test until we decide what to do"]
async fn it_should_create_purchase_request_and_confirm_it() {
    // Arrange
    dotenvy::dotenv().ok(); // only for this test since we load the mnemonic from .env
    let user: utils::TestUser = (*USER_HANS34).clone().into();

    /*
    TODO: define network with iota mainnet for tests
    DEPRECATED: configure sdk manually to run only this test on the iota mainnet
    */
    let existing_cleanup = CleanUp::default();
    let password = std::env::var("SATOSHI_PASSWORD").unwrap();
    let backend_url =
        std::env::var("RT_API_URL").expect("RT_API_URL should be set with the backend url for the tests to use");

    let config = Config {
        backend_url: backend_url.parse().expect("RT_API_URL must be a valid URL"),
        path_prefix: Path::new(&existing_cleanup.path_prefix).into(),
        auth_provider: "standalone".to_string(),
        log_level: log::LevelFilter::Debug,
    };

    let mut sdk = Sdk::new(config).expect("should not fail to initialize sdk"); // set the backend url if the environment variable is set

    // generate access token
    let access_token = testing::get_access_token(&user.username, &password).await.access_token;
    let access_token = AccessToken::try_from(access_token).unwrap();
    sdk.refresh_access_token(Some(access_token)).await.unwrap();

    // calling `set_network` method will automatically query the backend for the list of available networks
    // (because the network list is empty). You can manually add networks by calling `set_networks()`
    // to avoid fetching the list from the backend
    sdk.set_network(String::from("IOTA")).await.unwrap();

    /*
    rest of the test
    */

    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();
    sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();
    sdk.create_wallet_from_existing_mnemonic(&user.pin, &user.mnemonic)
        .await
        .unwrap();

    let _address = sdk.generate_new_address(&user.pin).await.unwrap(); // this is needed, otherwise the balance will be 0 and tx will fail
    let balance = sdk.get_balance(&user.pin).await.unwrap();
    println!("Current balance {:?}", balance); // print the balance to facilitate debugging

    let product_hash = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824";
    let app_data = "{\"imageUrl\":\"https://httpbin.org/\",\"imageId\":\"a846ad10-fc69-4b22-b442-5dd740ace361\"}";
    let purchase_type = "CLIK";

    // Act
    let amount = CryptoAmount::try_from(dec!(0.1)).unwrap();
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
            println!("Status: {:?}", details.status);
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

    // Act
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
                panic!("Purchase request failed!");
            }
        }
    })
    .await;
    if result.is_err() {
        panic!("Timeout reached while waiting for purchase request to complete");
    }
}

#[tokio::test]
#[ignore = "There are some blockers around this and IOTA is also changing. We disable this test until we decide what to do"]
async fn it_should_create_invalid_purchase_request_and_fail_to_confirm_it() {
    // Arrange
    let user: utils::TestUser = (*USER_HANS48).clone().into();
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;

    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();
    sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();
    sdk.create_wallet_from_existing_mnemonic(&user.pin, &user.mnemonic)
        .await
        .unwrap();

    let _address = sdk.generate_new_address(&user.pin).await.unwrap(); // this is needed, otherwise the balance will be 0 and tx will fail
    let balance = sdk.get_balance(&user.pin).await.unwrap();
    println!("Current balance {:?}", balance); // print the balance to facilitate debugging

    let product_hash = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824";
    let app_data = "{\"imageUrl\":\"https://httpbin.org/\",\"imageId\":\"a846ad10-fc69-4b22-b442-5dd740ace361\"}";
    let purchase_type = "CLIK";

    // Act
    let amount = CryptoAmount::try_from(dec!(2.0)).unwrap();
    let purchase_id = sdk
        .create_purchase_request(
            "vivi", // vivi is not verified
            amount,
            product_hash,
            app_data,
            purchase_type,
        )
        .await
        .unwrap();

    println!("Purchase_id {} ", purchase_id); // print the purchase id to facilitate debugging

    // Check for 1 min the xt status
    let _ = time::timeout(Duration::from_secs(60), async {
        loop {
            time::sleep(Duration::from_secs(5)).await;
            let details = sdk.get_purchase_details(&purchase_id).await.unwrap();
            match details.status {
                ApiTxStatus::Invalid(r) => {
                    println!("Purchase request invalid! Reason: {:?}.", r);
                    break;
                }
                ApiTxStatus::WaitingForVerification(r) => {
                    println!("Purchase request invalid! Reason: {:?}.", r);
                    break;
                }
                _ => {}
            }
        }
    })
    .await;

    // Act
    let res = sdk.confirm_purchase_request(&user.pin, &purchase_id).await;

    // Assert
    res.unwrap_err();
}
