/*
 * SPDX-License-Identifier: Unlicense
 *
 * main.rs: The main entry point for the Quantum kernel in Rust.
 *
 * This file serves as the crate root for the kernel binary. It sets up necessary
 * crate-level attributes, declares the module hierarchy, and defines the `kmain`
 * function, which is the first Rust code to execute after the assembly bootstub.
 */

// === CRATE ATTRIBUTES ===
// These are essential for freestanding binary development.

// Do not link against the standard library. We are the operating system.
#![no_std]
// Do not use the standard `main` entry point chain. We define our own `_start`.
#![no_main]
// Enable the panic_info_message feature to get more detailed panic payloads.
#![feature(panic_info_message)]

// === MODULE DECLARATIONS ===
// This builds the module tree for the compiler, making code in other files accessible.

// Architecture-specific code (aarch64).
mod arch;
// The kernel panic handler implementation.
mod panic;


// === KERNEL ENTRY POINT ===

use crate::arch::aarch64::uart;
use core::fmt::Write;

/// The primary entry point for the kernel written in Rust.
///
/// This function is called directly from the `boot.S` assembly code after
/// the initial low-level CPU setup (BSS clearing, stack pointer setup) is complete.
/// It is responsible for initializing kernel subsystems and transitioning
/// the system to a fully operational state.
///
/// The function signature `-> !` signifies that it is a diverging function;
/// it will never return.
#[no_mangle] // Ensure the function name is not mangled by the compiler.
pub extern "C" fn kmain() -> ! {
    // Initialize the UART driver to enable console output.
    // This is the first step, as we need logging capabilities immediately.
    uart::init();

    // At this stage, the console is ready. We can print our first message.
    // Using `writeln!` requires the `core::fmt::Write` trait, which we implemented for Uart.
    // This is an unsafe block because `_print` performs an unsynchronized write to a mutable static.
    unsafe {
        uart::_print(format_args!("\n[Quantum] UART initialized.\n"));
        uart::_print(format_args!("[Quantum] Kernel Rust entry point `kmain` reached.\n"));
    }

    // --- First Kernel Test ---
    // The most fundamental test for a new kernel is to verify that its
    // panic handler works correctly. We will trigger a deliberate panic
    // to ensure the entire chain (panic -> handler -> UART -> halt) is functional.
    panic!("Quantum boot test: Deliberate panic to verify exception handling.");
}
