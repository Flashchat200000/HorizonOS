/*
 * SPDX-License-Identifier: Unlicense
 *
 * mod.rs: Architecture-specific module root.
 *
 * This file declares and re-exports architecture-specific submodules,
 * providing a unified interface to the rest of the kernel.
 */

// On aarch64 builds, include and expose the aarch64-specific implementation.
#[cfg(target_arch = "aarch64")]
pub mod aarch64;
