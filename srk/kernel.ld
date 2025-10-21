/*
 * SPDX-License-Identifier: Unlicense
 *
 * kernel.ld: Linker script for the Quantum kernel on aarch64.
 *
 * This script dictates the memory layout of the final kernel executable.
 * It defines where code and data sections are placed in memory and provides
 * symbols that the kernel code can use to understand its own layout.
 */

/* Define the entry point of our kernel. This must match the symbol in boot.S */
ENTRY(_start)

/* Define the sections of our output binary. */
SECTIONS
{
    /* The kernel will be loaded starting at this physical address.
       0x40080000 is a common load address for QEMU's 'virt' machine. */
    . = 0x40080000;

    /* The boot text section MUST come first. */
    .text.boot :
    {
        KEEP(*(.text.boot))
    }

    /* All other executable code. */
    .text :
    {
        *(.text .text.*)
    }

    /* Read-only data. */
    .rodata :
    {
        *(.rodata .rodata.*)
    }

    /* Initialized read-write data. */
    .data :
    {
        *(.data .data.*)
    }

    /* Uninitialized read-write data (BSS).
       We define symbols for the start and end of this section so our boot
       code can zero it out. The `.` symbol represents the current location. */
    .bss :
    {
        __bss_start = .;
        *(.bss .bss.*)
        __bss_end = .;
    }

    /* Define the kernel stack. We allocate 128 KiB for the initial boot stack.
       The stack grows downwards, so our start symbol points to the top. */
    . = ALIGN(16);
    .stack :
    {
        . += 128K;
        _stack_start = .;
    }

    /* Discard sections we don't need to prevent bloating the binary. */
    /DISCARD/ :
    {
        *(.comment)
        *(.note .note.*)
    }
}
