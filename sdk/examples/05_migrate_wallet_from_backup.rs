use etopay_sdk::types::newtypes::PlainPassword;
mod utils;
use testing::USER_SATOSHI;
use utils::init_sdk;

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[tokio::main]
async fn main() {
    // Initialize SDK
    let (mut sdk, _cleanup) = init_sdk().await;
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();

    // Create new user
    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();

    // Create new wallet
    sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();
    sdk.create_wallet_from_new_mnemonic(&user.pin).await.unwrap();

    // Create wallet backup and delete it
    let backup_password = PlainPassword::try_from_string("backup_password").unwrap();
    let backup = sdk.create_wallet_backup(&user.pin, &backup_password).await.unwrap();
    sdk.delete_wallet(&user.pin).await.unwrap();

    // Migrate wallet from backup
    sdk.create_wallet_from_backup(&user.pin, &backup, &backup_password)
        .await
        .unwrap();

    // Fetch networks from backend
    let networks = sdk.get_networks().await.unwrap();
    let iota_network_key = &networks.first().unwrap().key;
    sdk.set_network(iota_network_key.to_string()).await.unwrap();

    // use wallet
    let _address = sdk.generate_new_address(&user.pin).await.unwrap();
}
