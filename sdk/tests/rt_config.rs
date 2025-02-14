mod utils;
use crate::utils::init_sdk;

use sdk::core::Sdk;
use testing::USER_SATOSHI;

#[tokio::test]
async fn it_should_get_node_urls() {
    // Arrange
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;

    // create user and wallet
    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();

    // Act
    let result = sdk.get_node_urls_backend().await;

    // Assert response
    let response = result.unwrap();
    assert!(response.contains_key("IOTA"));
}

#[test]
fn get_sdk_build_info() {
    let build_info = Sdk::get_build_info();
    assert!(!build_info.is_empty());
    println!("{build_info}");
}
