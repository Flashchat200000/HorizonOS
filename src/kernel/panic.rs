/*
 * SPDX-License-Identifier: Unlicense
 *
 * panic.rs: A multi-core safe panic handler for the Quantum kernel.
 *
 * This module provides the implementation for the `#[panic_handler]` attribute,
 * which is the entry point for handling unrecoverable kernel errors (panics).
 * The implementation is designed to be safe in a multi-processor environment
 * by using a spinlock to serialize access to the console, preventing garbled
 * output if multiple cores panic concurrently.
 */

use crate::arch::aarch64::{cpu, uart};
use core::panic::PanicInfo;
use core::sync::atomic::{AtomicBool, Ordering};

/// A simple atomic spinlock to ensure that only one core can be active
/// within the panic handler at any given time. This prevents interleaved,
/// unreadable output on the serial console.
static PANIC_LOCK: AtomicBool = AtomicBool::new(false);

/// The kernel's panic handler.
///
/// This function is invoked by the Rust runtime when a `panic!` macro is
/// called. It attempts to acquire a global lock, prints detailed diagnostic
/// information to the serial console, and then halts the system indefinitely.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Attempt to acquire the panic lock. If another core has already acquired
    // it, this core will spin until the first core halts the system. This
    // ensures panic messages are printed atomically.
    //
    // `compare_exchange` with `Acquire` ordering creates a memory barrier
    // that ensures no reads/writes are reordered before this point.
    while PANIC_LOCK.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
        // Hint to the CPU that we are in a busy-wait loop.
        core.hint::spin_loop();
    }

    // At this point, this core has exclusive access to the console.
    
    // Use the raw _print function to avoid further panics within the fmt machinery if possible.
    unsafe {
        uart::_print(format_args!("\n\n--- KERNEL PANIC ---\n"));

        if let Some(message) = info.message() {
            uart::_print(format_args!("Message:  {}\n", message));
        }

        if let Some(location) = info.location() {
            uart::_print(format_args!("Location: {} line {}\n", location.file(), location.line()));
        }

        uart::_print(format_args!("--------------------\n"));
    }

    // A kernel panic is an unrecoverable state. The only safe action is to
    // halt execution of the current core, and by extension, the system.
    // The lock is never released because the system will not continue.
    cpu::halt();
}
