//! This module links Swift to the Rust native library
//! It parses the Rust source files for #[swift_bridge::bridge] procedural macro headers and generates the corresponding Swift files
//! It then writes the auto-generated files to a single Swift file and all of the generated C headers to a single header file.
//! We ignore the generated files in .gitignore
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from("./include/generated");

    let bridges = vec!["src/lib.rs"];
    for path in &bridges {
        println!("cargo:rerun-if-changed={}", path);
    }
    println!("cargo:rerun-if-changed=include/generated/SwiftBridgeCore.swift");

    swift_bridge_build::parse_bridges(bridges).write_all_concatenated(out_dir, env!("CARGO_PKG_NAME"));
}
