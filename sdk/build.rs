/// This file configures and generates metadata for the SDK build process, including information
/// about the environment, version, and other build-related properties.
#[allow(clippy::expect_used)]
fn main() {
    // Create a default deny set which excludes `CARGO_METADATA`.

    let mut deny = shadow_rs::default_deny();

    // Exclude additional unnecessary properties.
    deny.insert(shadow_rs::CARGO_TREE);
    deny.insert(shadow_rs::CARGO_MANIFEST_DIR);
    deny.insert(shadow_rs::COMMIT_AUTHOR);
    deny.insert(shadow_rs::COMMIT_EMAIL);

    shadow_rs::ShadowBuilder::builder()
        .deny_const(deny)
        .build()
        .expect("could not build shadow_rs constants");
}
