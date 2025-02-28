mod utils;
use sdk::core::Sdk;

#[test]
fn get_sdk_build_info() {
    let build_info = Sdk::get_build_info();
    assert!(!build_info.is_empty());
    println!("{build_info}");
}
