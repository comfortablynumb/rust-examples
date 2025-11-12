# x86_64 Bootloader Example

This example demonstrates creating a minimal operating system kernel for x86_64 using the `bootloader` crate.

## Overview

This project creates a bootable disk image that:
1. Boots on real or virtual x86_64 hardware
2. Displays output via VGA text mode
3. Handles CPU exceptions
4. Runs in 64-bit long mode
5. Has paging enabled

## The Boot Process

### 1. BIOS/UEFI Boot

When the computer starts:
1. BIOS/UEFI loads the bootloader from disk
2. Bootloader switches CPU from real mode → protected mode → long mode
3. Bootloader sets up initial page tables
4. Bootloader loads the kernel and calls `kernel_main`

### 2. Bootloader Crate

The `bootloader` crate (v0.11) handles:
- **BIOS boot**: Legacy 16-bit boot
- **UEFI boot**: Modern boot
- **Memory initialization**: Sets up identity mapping
- **Kernel loading**: Loads ELF kernel binary
- **Long mode**: Switches to 64-bit mode
- **BootInfo**: Provides memory map and other info to kernel

## VGA Text Mode

### Memory Layout

VGA text buffer is at physical address `0xB8000`:
- 80 columns × 25 rows
- Each character is 2 bytes:
  - Byte 0: ASCII character
  - Byte 1: Color attribute

### Color Format

```
Byte 1: 0xBF
        || |
        || +-- Foreground (0-15)
        |+---- Background (0-15)
        +----- Blink (optional)
```

Common colors:
- `0x0F`: White on black
- `0x0A`: Light green on black
- `0x0C`: Light red on black
- `0x4F`: White on red (error)

### Printing

```rust
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

fn print_char(row: usize, col: usize, character: u8, color: u8) {
    let offset = (row * 80 + col) * 2;
    unsafe {
        *VGA_BUFFER.add(offset) = character;
        *VGA_BUFFER.add(offset + 1) = color;
    }
}
```

## Interrupt Handling

### IDT - Interrupt Descriptor Table

The IDT maps interrupt numbers to handler functions:

```rust
use x86_64::structures::idt::InterruptDescriptorTable;

let mut idt = InterruptDescriptorTable::new();
idt.breakpoint.set_handler_fn(breakpoint_handler);
idt.load();
```

### Exception Handlers

CPU exceptions use the `x86-interrupt` calling convention:

```rust
extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame
) {
    // Handle breakpoint
}
```

### Common Exceptions

| Exception | Number | Description |
|-----------|--------|-------------|
| Division Error | 0 | Division by zero |
| Debug | 1 | Debug trap |
| Breakpoint | 3 | INT3 instruction |
| Overflow | 4 | INTO instruction |
| Invalid Opcode | 6 | Invalid instruction |
| Double Fault | 8 | Exception during exception |
| Page Fault | 14 | Invalid memory access |

## x86_64 Architecture

### Control Registers

```rust
use x86_64::registers::control::{Cr0, Cr3, Cr4};

// CR0: System control flags
let cr0 = Cr0::read();

// CR3: Page table pointer
let (frame, flags) = Cr3::read();

// CR4: Extended features
let cr4 = Cr4::read();
```

### CPU Instructions

```rust
use x86_64::instructions;

// Halt until interrupt
instructions::hlt();

// Enable interrupts
instructions::interrupts::enable();

// Disable interrupts
instructions::interrupts::disable();

// No operation
instructions::nop();
```

### Port I/O

```rust
use x86_64::instructions::port::Port;

// Write to port
let mut port = Port::new(0x3F8);
unsafe { port.write(b'H') };

// Read from port
let value: u8 = unsafe { port.read() };
```

## Building and Running

### Prerequisites

Install required tools:

```bash
# Install nightly Rust (required for unstable features)
rustup default nightly

# Install x86_64 target
rustup target add x86_64-unknown-none

# Install bootimage tool
cargo install bootimage

# Install QEMU for testing
# On Ubuntu/Debian:
sudo apt install qemu-system-x86

# On macOS:
brew install qemu
```

### Build Bootable Image

```bash
# Build the kernel
cargo build --release

# Create bootable disk image
cargo bootimage --release
```

This creates `target/x86_64-unknown-none/release/bootimage-bootloader_example.bin`

### Run in QEMU

```bash
# Run with default settings
cargo run --release

# Or manually:
qemu-system-x86_64 \
    -drive format=raw,file=target/x86_64-unknown-none/release/bootimage-bootloader_example.bin
```

### QEMU Options

```bash
# With more memory
qemu-system-x86_64 -m 256M -drive format=raw,file=...

# With serial output
qemu-system-x86_64 -serial stdio -drive format=raw,file=...

# Without display (for CI)
qemu-system-x86_64 -display none -drive format=raw,file=...

# Enable KVM acceleration (Linux only)
qemu-system-x86_64 -enable-kvm -drive format=raw,file=...
```

### Boot on Real Hardware

⚠️ **Warning**: This can overwrite your hard drive! Use with caution.

```bash
# Write to USB drive (replace /dev/sdX with your USB drive!)
sudo dd if=target/.../bootimage-*.bin of=/dev/sdX bs=1M

# Safely eject
sync
sudo eject /dev/sdX
```

Then boot from the USB drive.

## Memory Management

### Page Tables

x86_64 uses 4-level page tables:

```
Virtual Address (48 bits)
┌────────┬────────┬────────┬────────┬────────────────────┐
│  PML4  │  PDPT  │   PD   │   PT   │       Offset       │
│ 9 bits │ 9 bits │ 9 bits │ 9 bits │      12 bits       │
└────────┴────────┴────────┴────────┴────────────────────┘
```

The bootloader sets up identity mapping (virtual == physical).

### Adding Paging Support

```rust
use x86_64::structures::paging::{PageTable, RecursivePageTable};

// Access page tables
let (level_4_table_frame, _) = Cr3::read();
let phys = level_4_table_frame.start_address();
let virt = phys.as_u64(); // Identity mapped
let page_table_ptr: *mut PageTable = virt as *mut PageTable;
let level_4_table = unsafe { &mut *page_table_ptr };
```

## Extending the OS

### 1. Add Serial Port Output

```rust
use x86_64::instructions::port::Port;

struct SerialPort {
    port: Port<u8>,
}

impl SerialPort {
    fn new(port: u16) -> Self {
        Self { port: Port::new(port) }
    }

    fn send(&mut self, byte: u8) {
        unsafe {
            self.port.write(byte);
        }
    }
}

// COM1 port
let mut serial = SerialPort::new(0x3F8);
```

### 2. Add Keyboard Input

```rust
#[interrupt]
fn keyboard_handler() {
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    // Process scancode
}
```

### 3. Add Timer

```rust
#[interrupt]
fn timer_handler() {
    // Called on each timer tick
    // Can be used for scheduling
}
```

### 4. Add Memory Allocator

```rust
use bootloader::BootInfo;

fn init_heap(boot_info: &'static BootInfo) {
    // Find usable memory regions
    for region in boot_info.memory_regions.iter() {
        if region.kind == MemoryRegionKind::Usable {
            // Initialize allocator with this region
        }
    }
}
```

## Debugging

### QEMU Monitor

```bash
# Run with monitor on stdio
qemu-system-x86_64 -monitor stdio ...

# Or on telnet
qemu-system-x86_64 -monitor telnet:127.0.0.1:55555,server,nowait ...
```

Monitor commands:
- `info registers`: Show CPU registers
- `info mem`: Show memory mappings
- `x /10i $rip`: Disassemble at instruction pointer

### GDB Debugging

```bash
# Start QEMU with GDB server
qemu-system-x86_64 -s -S -drive format=raw,file=...

# In another terminal
gdb target/.../bootloader_example
(gdb) target remote :1234
(gdb) break kernel_main
(gdb) continue
```

### Debug Printing

Add to kernel:

```rust
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let _ = write!($crate::vga_buffer::WRITER.lock(), $($arg)*);
    });
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
```

## Common Issues

### Bootloader Version Conflicts

Ensure `bootloader = "0.11"` in `Cargo.toml`.

### Target Not Found

Install the target:
```bash
rustup target add x86_64-unknown-none
```

### Bootimage Not Working

Update rustup components:
```bash
rustup component add llvm-tools-preview
```

### QEMU Not Found

Install QEMU:
```bash
# Linux
sudo apt install qemu-system-x86

# macOS
brew install qemu
```

## Next Steps

1. Implement a heap allocator
2. Add keyboard driver
3. Implement multitasking
4. Add file system support
5. Implement userspace programs

## References

- [Writing an OS in Rust](https://os.phil-opp.com/) - Excellent tutorial series
- [bootloader crate](https://docs.rs/bootloader/)
- [x86_64 crate](https://docs.rs/x86_64/)
- [OSDev Wiki](https://wiki.osdev.org/) - Comprehensive OS development resource
- [Intel x86_64 Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)
