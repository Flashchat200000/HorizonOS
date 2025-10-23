// build.rs

use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/kernel/arch/aarch64/boot.S");
    println!("cargo:rerun-if-changed=kernel.ld");

    let status = Command::new("as")
        .args(&[
            "-o",
            "target/boot.o",
            "src/kernel/arch/aarch64/boot.S",
        ])
        .status()
        .expect("Failed to execute 'as'");

    if !status.success() {
        panic!("'as' command failed");
    }

    println!("cargo:rustc-link-arg=target/boot.o");
}
