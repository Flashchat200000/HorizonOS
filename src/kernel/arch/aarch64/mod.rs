/*
 * SPDX-License-Identifier: Unlicense
 *
 * mod.rs: Module root for the aarch64 architecture implementation.
 *
 * This file declares the submodules that constitute the aarch64-specific
 * portion of the kernel and re-exports their public interfaces.
 */

// Declare the cpu and uart modules as part of the aarch64 module.
// This will cause the compiler to look for `cpu.rs` and `uart.rs`.
pub mod cpu;
pub mod uart;
