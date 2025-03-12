mod utils;
use testing::{IOTA_NETWORK_ID, USER_SATOSHI};
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

    // Fetch networks from backend
    let networks = sdk.get_networks().await.unwrap();
    let iota_network_id = &networks.first().unwrap().id;
    sdk.set_network(iota_network_id.to_string()).await.unwrap();

    // Get exchange rate
    let exchange_rate = sdk.get_exchange_rate().await.unwrap();
    println!("Exchange rate: {}", exchange_rate);
}
