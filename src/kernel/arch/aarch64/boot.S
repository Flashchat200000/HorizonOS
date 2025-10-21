/*
 * SPDX-License-Identifier: Unlicense
 *
 * boot.S: Initial boot code for HorizonOS/Quantum on aarch64.
 * This is the very first code to be executed by the CPU.
 */

// Import symbols from our Rust code and the linker script.
// These symbols do not exist yet, but the assembler needs to know they will.
.extern kmain
.extern __bss_start
.extern __bss_end

// Declare our entry point as a global symbol for the linker.
.global _start

// The section for executable boot code.
.section ".text.boot"

//
// _start: The absolute entry point of the kernel.
//
_start:
    // Step 1: Identify which core is executing this code.
    // In a multi-core system, all cores may start here. We only want
    // one primary core to initialize the system.
    mrs     x1, mpidr_el1       // Read the processor/core ID register.
    and     x1, x1, #0xFF       // Mask to get only the core ID (bits 0-7).
    cbz     x1, .primary_core_boot // If core ID is 0, proceed with boot.

.park_secondary_core:
    // Step 1a: Secondary cores go into a holding pattern.
    // They will wait here until the kernel explicitly wakes them up later.
    wfe                         // Wait For Event.
    b       .park_secondary_core

.primary_core_boot:
    // Step 2: For the primary core, clear the BSS section.
    // BSS is where uninitialized static variables live. The Rust compiler
    // expects this memory region to be zeroed out before its code runs.
    ldr     x1, =__bss_start    // Load the start address of the BSS section.
    ldr     x2, =__bss_end      // Load the end address of the BSS section.

.bss_loop:
    cmp     x1, x2              // While start address < end address...
    b.ge    .bss_done           // ...exit loop if done.
    str     xzr, [x1], #8       // ...store a zero (from the xzr register) and advance pointer by 8 bytes.
    b       .bss_loop           // Repeat.

.bss_done:
    // Step 3: Set up the initial stack pointer (SP).
    // No function can be called without a valid stack. We point the SP
    // to the top of the stack region, as the stack grows downwards.
    // The `_stack_start` address must be provided by the linker script.
    ldr     x1, =_stack_start
    mov     sp, x1

    // Step 4: Branch to the Rust kernel's main function.
    // The low-level setup is complete.
    bl      kmain

    // Step 5: Safety halt.
    // The `kmain` function is marked as diverging (`-> !`), so it should
    // never return. If it does, something has gone catastrophically wrong.
    // We must catch the CPU here to prevent it from executing garbage.
.halt:
    wfi                         // Wait For Interrupt in a loop.
    b       .halt
