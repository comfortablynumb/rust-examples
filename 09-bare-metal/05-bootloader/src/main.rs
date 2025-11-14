#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

// VGA text buffer constants
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

/// Entry point for the bootloader
///
/// The bootloader crate:
/// 1. Loads the kernel into memory
/// 2. Sets up initial page tables
/// 3. Switches to long mode (64-bit)
/// 4. Calls this function with BootInfo
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // Clear the screen
    clear_screen();

    // Print welcome message
    print_string(0, 0, b"Rust Bare Metal OS", 0x0F);
    print_string(1, 0, b"==================", 0x0F);

    // Display boot information
    print_string(3, 0, b"Boot Info:", 0x0A);

    // Print memory map info
    let memory_regions = boot_info.memory_regions.len();
    print_string(4, 2, b"Memory regions: ", 0x07);
    print_number(4, 18, memory_regions as u64, 0x07);

    // Initialize IDT (Interrupt Descriptor Table)
    init_idt();
    print_string(6, 0, b"IDT initialized", 0x0A);

    // Enable interrupts
    x86_64::instructions::interrupts::enable();
    print_string(7, 0, b"Interrupts enabled", 0x0A);

    // Display some system info
    print_string(9, 0, b"System Status:", 0x0E);
    print_string(10, 2, b"[OK] CPU in long mode (64-bit)", 0x02);
    print_string(11, 2, b"[OK] Paging enabled", 0x02);
    print_string(12, 2, b"[OK] Kernel loaded and running", 0x02);

    // Demonstrate some x86_64 instructions
    print_string(14, 0, b"CPU Info:", 0x0E);

    // Read CR3 (page table pointer)
    let cr3 = x86_64::registers::control::Cr3::read();
    print_string(15, 2, b"CR3 (page table): 0x", 0x07);
    print_hex(15, 22, cr3.0.start_address().as_u64(), 0x07);

    // Main loop
    print_string(17, 0, b"Entering main loop...", 0x0F);

    loop {
        // Halt until next interrupt
        x86_64::instructions::hlt();
    }
}

/// Initialize the Interrupt Descriptor Table
fn init_idt() {
    let mut idt = InterruptDescriptorTable::new();

    // Register breakpoint handler
    idt.breakpoint.set_handler_fn(breakpoint_handler);

    // Register double fault handler
    unsafe {
        idt.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(0); // Use IST (Interrupt Stack Table)
    }

    // Load the IDT
    idt.load();
}

/// Breakpoint exception handler
extern "x86-interrupt" fn breakpoint_handler(_stack_frame: InterruptStackFrame) {
    print_string(20, 0, b"BREAKPOINT!", 0x0C);
}

/// Double fault exception handler
extern "x86-interrupt" fn double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    print_string(20, 0, b"DOUBLE FAULT!", 0x4F);
    loop {
        x86_64::instructions::hlt();
    }
}

/// Clear the VGA text buffer
fn clear_screen() {
    let buffer =
        unsafe { core::slice::from_raw_parts_mut(VGA_BUFFER, BUFFER_HEIGHT * BUFFER_WIDTH * 2) };

    for i in (0..buffer.len()).step_by(2) {
        buffer[i] = b' ';
        buffer[i + 1] = 0x00; // Black background, black foreground
    }
}

/// Print a string at the specified row and column
///
/// color format: 0xBF where B is background (high 4 bits), F is foreground (low 4 bits)
/// Common colors:
/// - 0x0F: Black bg, white fg
/// - 0x0A: Black bg, light green fg
/// - 0x0C: Black bg, light red fg
/// - 0x0E: Black bg, yellow fg
fn print_string(row: usize, col: usize, text: &[u8], color: u8) {
    let offset = (row * BUFFER_WIDTH + col) * 2;
    let buffer =
        unsafe { core::slice::from_raw_parts_mut(VGA_BUFFER, BUFFER_HEIGHT * BUFFER_WIDTH * 2) };

    for (i, &byte) in text.iter().enumerate() {
        let pos = offset + i * 2;
        if pos < buffer.len() {
            buffer[pos] = byte;
            buffer[pos + 1] = color;
        }
    }
}

/// Print a number in decimal
fn print_number(row: usize, col: usize, mut num: u64, color: u8) {
    let mut buffer = [b'0'; 20];
    let mut i = 19;

    if num == 0 {
        print_string(row, col, &[b'0'], color);
        return;
    }

    while num > 0 {
        buffer[i] = b'0' + (num % 10) as u8;
        num /= 10;
        if i > 0 {
            i -= 1;
        }
    }

    print_string(row, col, &buffer[i + 1..], color);
}

/// Print a number in hexadecimal
fn print_hex(row: usize, col: usize, mut num: u64, color: u8) {
    let hex_chars = b"0123456789ABCDEF";
    let mut buffer = [b'0'; 16];

    for i in (0..16).rev() {
        buffer[i] = hex_chars[(num & 0xF) as usize];
        num >>= 4;
    }

    print_string(row, col, &buffer, color);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Print panic message to VGA buffer
    print_string(22, 0, b"KERNEL PANIC!", 0x4F);

    if let Some(location) = info.location() {
        // In a real OS, we would print the file and line number
        let _ = location;
    }

    loop {
        x86_64::instructions::hlt();
    }
}
