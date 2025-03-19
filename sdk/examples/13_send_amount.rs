mod utils;
use rust_decimal_macros::dec;
use testing::USER_SATOSHI;
use utils::init_sdk;

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[tokio::main]
async fn main() {
    // Initialize SDK
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();
    let (mut sdk, _cleanup) = init_sdk().await;

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

    // Generate new address
    let recipient_address = sdk.generate_new_address(&user.pin).await.unwrap();
    let balance = sdk.get_balance(&user.pin).await.unwrap();
    println!("address: {recipient_address}, balance: {balance:?}");

    // Send amount
    let amount = dec!(2.0).try_into().unwrap();
    let data = Some("test".to_string().into_bytes());
    // estimate gas
    let estimate = sdk
        .estimate_gas(&user.pin, &recipient_address, amount, data.clone())
        .await
        .unwrap();

    println!("Estimated gas: {estimate:?}");

    let tx_id = sdk
        .send_amount(&user.pin, &recipient_address, amount, data)
        .await
        .unwrap();

    println!("Success with transaction id: {tx_id}");
}
