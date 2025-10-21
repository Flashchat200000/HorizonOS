/*
 * SPDX-License-Identifier: Unlicense
 *
 * cpu.rs: Low-level CPU state management functions for aarch64.
 *
 * This module provides essential, architecture-specific functions for controlling
 * the CPU's execution state, such as halting or waiting for events.
 */

#![allow(dead_code)] // Allow unused functions for now.

use core::arch::asm;

/// Halts the current CPU core indefinitely.
///
/// This function is the ultimate final action for an unrecoverable error or
/// after a core has completed its shutdown sequence. It enters a low-power
/// state and will not resume execution.
#[inline]
pub fn halt() -> ! {
    loop {
        unsafe {
            // The `wfi` (Wait For Interrupt) instruction puts the core into a
            // low-power sleep state until an interrupt occurs. In a panic scenario
            // or shutdown, interrupts are typically disabled, making this an
            // effective and power-efficient halt.
            asm!("wfi", options(nomem, nostack, preserves_flags));
        }
    }
}

/// Enters a busy-wait loop.
///
/// This provides a simple spin loop, which can be used to wait for a condition
/// in situations where yielding or sleeping is not possible or desired (e.g.,
/// within a spinlock implementation on an already-panicking system).
#[inline]
pub fn spin() -> ! {
    loop {
        // `yield` is a hint to the processor that it can temporarily
        // pass execution to another hardware thread on the same core.
        // It's a slightly more efficient way to busy-wait than a raw loop.
        asm!("yield", options(nomem, nostack, preserves_flags));
    }
}
