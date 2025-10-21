/*
 * SPDX-License-Identifier: Unlicense
 *
 * uart.rs: A basic, polling-based driver for the PL011 UART.
 *
 * This module provides the necessary functions to initialize and write
 * characters to a serial console for early kernel debugging.
 */

use core::fmt::{self, Write};
use core::ptr::{read_volatile, write_volatile};

// The PL011 UART is a memory-mapped device. Its registers are accessed
// through a specific physical memory address. For the QEMU 'virt' machine,
// this address is 0x09000000.
const UART_BASE: usize = 0x09000000;

/// Register offsets from the UART base address.
#[allow(dead_code)]
mod registers {
    pub const DATA: usize = 0x00;        // Data Register (DR)
    pub const FLAG: usize = 0x18;        // Flag Register (FR)
    pub const INT_MASK_SET: usize = 0x38; // Interrupt Mask Set/Clear Register (IMSC)
}

/// Flag Register (FR) bit definitions.
mod flags {
    pub const TXFF: u32 = 1 << 5; // Transmit FIFO full.
}

/// Represents a PL011 UART device instance.
pub struct Uart {
    base_address: usize,
}

impl Uart {
    /// Creates a new Uart instance.
    ///
    /// # Safety
    /// This function is unsafe because it deals with a raw, fixed memory address
    /// that is assumed to point to a valid UART device.
    pub const unsafe fn new(base_address: usize) -> Self {
        Self { base_address }
    }

    /// Initializes the UART.
    /// For this simple polling driver, initialization consists of disabling all interrupts.
    pub fn init(&self) {
        let imsc_addr = (self.base_address + registers::INT_MASK_SET) as *mut u32;
        unsafe {
            // Write 0 to the Interrupt Mask Set/Clear register to disable all UART interrupts.
            write_volatile(imsc_addr, 0x00);
        }
    }

    /// Writes a single byte to the serial output.
    /// This function will block until the hardware is ready to accept a new byte.
    fn write_byte(&self, byte: u8) {
        let data_addr = (self.base_address + registers::DATA) as *mut u8;
        let flag_addr = (self.base_address + registers::FLAG) as *const u32;

        // Wait until the transmit FIFO is not full. This is known as "polling".
        while unsafe { read_volatile(flag_addr) } & flags::TXFF != 0 {
            // A simple busy-wait loop.
            core::arch::asm!("nop");
        }

        // Write the byte to the data register.
        unsafe {
            write_volatile(data_addr, byte);
        }
    }
}

// Implement the `core::fmt::Write` trait for our Uart struct.
// This allows us to use Rust's standard formatting macros (e.g., `write!`, `writeln!`).
impl Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            // Serial terminals expect a carriage return (`\r`) before a line feed (`\n`).
            if byte == b'\n' {
                self.write_byte(b'\r');
            }
            self.write_byte(byte);
        }
        Ok(())
    }
}

// A mutable static instance of the UART driver for global access.
// In a multi-core context, access to this would need to be synchronized with a lock.
static mut KERNEL_UART: Uart = unsafe { Uart::new(UART_BASE) };

/// Public interface to initialize the global kernel UART.
pub fn init() {
    unsafe { KERNEL_UART.init() };
}

/// Public interface to print formatted arguments to the kernel console.
/// This is the backbone for any `println!` or `print!` macros.
///
/// # Safety
/// This function performs an unsynchronized write to a mutable static variable.
/// The caller must ensure that no concurrent writes can occur.
pub unsafe fn _print(args: fmt::Arguments) {
    KERNEL_UART.write_fmt(args).unwrap();
}
