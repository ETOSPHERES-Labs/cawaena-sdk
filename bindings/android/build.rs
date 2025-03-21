// This comes from the README of `cargo-ndk` as an example of how to link to and copy
// the libc++_shared library to the output folder automatically.

#![allow(clippy::unwrap_used)]
use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    // generate jni java bindings
    println!("cargo:rerun-if-changed=src/lib.rs");
    jnigen_build::generate("src/lib.rs", "etopaysdk", "tests/src/main/java");
    jnigen_build::generate("src/lib.rs", "etopaysdk", "src/main/java");

    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "android" {
        android();
    }
}
fn android() {
    println!("cargo:rustc-link-lib=c++_shared");

    if let Ok(output_path) = env::var("CARGO_NDK_OUTPUT_PATH") {
        let sysroot_libs_path = PathBuf::from(env::var_os("CARGO_NDK_SYSROOT_LIBS_PATH").unwrap());
        let lib_path = sysroot_libs_path.join("libc++_shared.so");
        let out_folder = Path::new(&output_path).join(env::var("CARGO_NDK_ANDROID_TARGET").unwrap());

        // make sure the output folder exists before copying
        std::fs::create_dir_all(&out_folder).unwrap();

        std::fs::copy(lib_path, out_folder.join("libc++_shared.so")).unwrap();
    } else {
        panic!("Could not copy libc++_shared.so since `CARGO_NDK_OUTPUT_PATH` was not set");
    }
}
