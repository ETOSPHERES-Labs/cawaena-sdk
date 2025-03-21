use etopay_sdk::types::currencies::CryptoAmount;
use rust_decimal_macros::dec;
use testing::USER_SATOSHI;
mod utils;
use utils::init_sdk;

#[allow(clippy::unwrap_used)]
#[tokio::main]
async fn main() {
    // Initialize SDK
    let (mut sdk, _cleanup) = init_sdk().await;
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();

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

    // Generate address
    let address = sdk.generate_new_address(&user.pin).await.unwrap();
    println!("Address: {}", address);

    // Get balance
    let balance = sdk.get_balance(&user.pin).await.unwrap();
    println!("Balance: {:?}", balance);

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
}
