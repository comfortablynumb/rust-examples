#![no_std]
#![no_main]

use core::panic::PanicInfo;
use riscv::register::{mstatus, mcause, mepc, mtvec};
use riscv_rt::{entry, interrupt};

// UART base address (for QEMU virt machine)
// On real hardware, check your SoC's memory map
const UART_BASE: usize = 0x1000_0000;

/// Entry point for RISC-V program
///
/// The riscv-rt crate:
/// - Sets up the stack pointer
/// - Initializes .data and .bss sections
/// - Calls this function
/// - Handles trap vectors
#[entry]
fn main() -> ! {
    // Initialize UART for output
    uart_init();

    // Print startup message
    uart_puts(b"\r\n");
    uart_puts(b"RISC-V Bare Metal Example\r\n");
    uart_puts(b"=========================\r\n\r\n");

    // Read and display CSRs (Control and Status Registers)
    display_csr_info();

    // Configure machine-mode interrupts
    setup_interrupts();

    uart_puts(b"\r\nInitialization complete!\r\n");
    uart_puts(b"Entering main loop...\r\n\r\n");

    let mut counter: u32 = 0;

    loop {
        counter = counter.wrapping_add(1);

        if counter % 1_000_000 == 0 {
            uart_puts(b".");
        }

        // In a real application:
        // - Process UART input
        // - Handle sensor data
        // - Manage peripherals
        // - Wait for interrupts with WFI instruction

        // Optionally use WFI (Wait For Interrupt) to save power
        // unsafe { riscv::asm::wfi(); }
    }
}

/// Display information about Control and Status Registers
fn display_csr_info() {
    uart_puts(b"CSR Information:\r\n");

    // Read mstatus (Machine Status Register)
    let mstatus_val = mstatus::read();
    uart_puts(b"  mstatus: ");
    print_hex(mstatus_val.bits());
    uart_puts(b"\r\n");

    // Check if machine interrupts are enabled
    if mstatus_val.mie() {
        uart_puts(b"    - Machine interrupts: ENABLED\r\n");
    } else {
        uart_puts(b"    - Machine interrupts: DISABLED\r\n");
    }

    // Read mtvec (Machine Trap Vector)
    let mtvec_val = mtvec::read();
    uart_puts(b"  mtvec: ");
    print_hex(mtvec_val.bits());
    uart_puts(b"\r\n");
}

/// Setup interrupt handling
fn setup_interrupts() {
    uart_puts(b"\r\nSetting up interrupts...\r\n");

    unsafe {
        // Set trap handler mode to vectored
        // In vectored mode, each interrupt has its own handler
        mtvec::write(trap_handler as usize, mtvec::TrapMode::Direct);

        // Enable machine interrupts in mstatus
        mstatus::set_mie();
    }

    uart_puts(b"Interrupts configured\r\n");
}

/// Trap handler - called for all interrupts and exceptions
///
/// RISC-V has a unified trap handler that handles both:
/// - Interrupts (asynchronous events from hardware)
/// - Exceptions (synchronous events from software)
#[no_mangle]
unsafe extern "C" fn trap_handler() {
    // Read trap cause
    let cause = mcause::read();

    uart_puts(b"\r\n[TRAP] ");

    if cause.is_interrupt() {
        uart_puts(b"Interrupt: ");
        match cause.code() {
            1 => uart_puts(b"Supervisor software interrupt\r\n"),
            3 => uart_puts(b"Machine software interrupt\r\n"),
            5 => uart_puts(b"Supervisor timer interrupt\r\n"),
            7 => uart_puts(b"Machine timer interrupt\r\n"),
            9 => uart_puts(b"Supervisor external interrupt\r\n"),
            11 => uart_puts(b"Machine external interrupt\r\n"),
            code => {
                uart_puts(b"Unknown (");
                print_hex(code);
                uart_puts(b")\r\n");
            }
        }
    } else {
        uart_puts(b"Exception: ");
        match cause.code() {
            0 => uart_puts(b"Instruction address misaligned\r\n"),
            1 => uart_puts(b"Instruction access fault\r\n"),
            2 => uart_puts(b"Illegal instruction\r\n"),
            3 => uart_puts(b"Breakpoint\r\n"),
            4 => uart_puts(b"Load address misaligned\r\n"),
            5 => uart_puts(b"Load access fault\r\n"),
            6 => uart_puts(b"Store address misaligned\r\n"),
            7 => uart_puts(b"Store access fault\r\n"),
            8 => uart_puts(b"Environment call from U-mode\r\n"),
            11 => uart_puts(b"Environment call from M-mode\r\n"),
            code => {
                uart_puts(b"Unknown (");
                print_hex(code);
                uart_puts(b")\r\n");
            }
        }
    }

    // Read exception program counter
    uart_puts(b"  mepc: ");
    print_hex(mepc::read());
    uart_puts(b"\r\n");

    // For exceptions, increment mepc to skip the faulting instruction
    // (In a real OS, you might want to handle this differently)
    if !cause.is_interrupt() {
        mepc::write(mepc::read() + 4);
    }
}

// ============================================================================
// UART Driver
// ============================================================================

/// Initialize UART
fn uart_init() {
    // For QEMU virt machine, UART is pre-configured
    // On real hardware, you would:
    // 1. Set baud rate
    // 2. Configure data bits, parity, stop bits
    // 3. Enable TX/RX
}

/// Send a single byte over UART
fn uart_putc(c: u8) {
    unsafe {
        // Write to UART transmit register
        // For 16550 UART (used by QEMU virt):
        // Offset 0: Transmit/Receive buffer
        let uart_tx = UART_BASE as *mut u8;
        core::ptr::write_volatile(uart_tx, c);
    }
}

/// Send a string over UART
fn uart_puts(s: &[u8]) {
    for &byte in s {
        uart_putc(byte);
    }
}

/// Print a hexadecimal number
fn print_hex(mut num: usize) {
    let hex_chars = b"0123456789ABCDEF";

    uart_puts(b"0x");

    // Print 8 hex digits (32-bit value)
    for i in (0..8).rev() {
        let digit = ((num >> (i * 4)) & 0xF) as usize;
        uart_putc(hex_chars[digit]);
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    uart_puts(b"\r\n\r\n");
    uart_puts(b"========== PANIC ==========\r\n");

    if let Some(location) = info.location() {
        uart_puts(b"Location: ");
        uart_puts(location.file().as_bytes());
        uart_puts(b":");
        // Would print line number here if we had number printing
        uart_puts(b"\r\n");
    }

    if let Some(message) = info.message() {
        uart_puts(b"Message: ");
        // Would print message here if we had formatting
        uart_puts(b"\r\n");
    }

    uart_puts(b"===========================\r\n");

    loop {
        // Halt
        unsafe {
            riscv::asm::wfi();
        }
    }
}

// Notes on RISC-V Architecture:
//
// 1. Privilege Levels:
//    - M-mode (Machine): Highest privilege, full hardware access
//    - S-mode (Supervisor): OS kernel level
//    - U-mode (User): User applications
//
// 2. CSRs (Control and Status Registers):
//    - mstatus: Global interrupt enable, privilege mode
//    - mie: Machine interrupt enable (specific interrupts)
//    - mip: Machine interrupt pending
//    - mtvec: Trap vector (interrupt/exception handler address)
//    - mepc: Exception program counter
//    - mcause: Trap cause (interrupt or exception type)
//    - mtval: Trap value (additional info about trap)
//
// 3. Important Instructions:
//    - WFI: Wait for interrupt (low power)
//    - ECALL: Environment call (system call)
//    - EBREAK: Breakpoint
//    - MRET: Return from machine-mode trap
//    - FENCE: Memory ordering
//    - CSR instructions: Read/write CSRs
//
// 4. Memory Ordering:
//    - RISC-V has a weak memory model
//    - Use FENCE instructions for synchronization
//    - Atomics: LR/SC (Load Reserved/Store Conditional)
//               or AMO (Atomic Memory Operations)
