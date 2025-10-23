// build.rs

use std::process::Command;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    println!("cargo:rerun-if-changed=src/kernel/arch/aarch64/boot.S");
    println!("cargo:rerun-if-changed=kernel.ld");

    let status = Command::new("aarch64-linux-gnu-as")
        .args(&[
            "-o",
            &format!("{}/boot.o", out_dir),
            "src/kernel/arch/aarch64/boot.S",
        ])
        .status()
        .expect("Failed to execute 'as'");

    if !status.success() {
        panic!("'as' command failed");
    }

    println!("cargo:rustc-link-arg={}/boot.o", out_dir);
}
