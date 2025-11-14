# Bare Metal Rust Examples

This directory contains comprehensive examples of bare metal (no_std) Rust programming for embedded systems and low-level development.

## Overview

Bare metal Rust programming means running code without an operating system or standard library. This is essential for:

- **Embedded systems**: Microcontrollers and IoT devices
- **Operating system development**: Writing your own OS
- **Bootloaders**: Low-level system initialization
- **Real-time systems**: Deterministic, predictable behavior
- **Resource-constrained devices**: Minimal RAM/flash usage

## Examples

### 1. [No-Std Basics](./01-no-std-basics/)
**Difficulty**: Beginner
**Target**: Generic (any no_std target)

Introduction to `#![no_std]` programming:
- `#![no_std]` and `#![no_main]` attributes
- Custom entry points
- Panic handlers
- Using `core` library
- What's available without `std`

**Key concepts**: Entry points, panic handling, core library

### 2. [Custom Allocator](./02-custom-allocator/)
**Difficulty**: Intermediate
**Target**: Generic (any no_std target)

Memory allocation in bare metal:
- Bump allocator (fast, no free)
- Linked-list allocator (general purpose)
- `#[global_allocator]` attribute
- Using `alloc` crate (Vec, String, Box)
- `#[alloc_error_handler]`

**Key concepts**: Memory management, heap allocation, allocator traits

### 3. [Embedded HAL](./03-embedded-hal/)
**Difficulty**: Intermediate
**Target**: Generic (portable)

Hardware abstraction layer for portable embedded code:
- GPIO traits (OutputPin, InputPin)
- SPI and I2C communication
- Delay trait
- Writing platform-agnostic drivers
- Mock implementations for testing

**Key concepts**: Abstraction, portability, driver development

### 4. [ARM Cortex-M](./04-arm-cortex-m/)
**Difficulty**: Intermediate
**Target**: `thumbv7em-none-eabihf` (ARM Cortex-M4F/M7F)

ARM Cortex-M specific features:
- `cortex-m` and `cortex-m-rt` crates
- SysTick timer
- Interrupt handling
- Exception handlers
- Core peripherals (NVIC, SCB, DWT)
- WFI and low-power modes

**Key concepts**: ARM architecture, interrupts, real hardware

**Build**: Requires ARM target
```bash
rustup target add thumbv7em-none-eabihf
cargo build --manifest-path 04-arm-cortex-m/Cargo.toml
```

### 5. [x86_64 Bootloader](./05-bootloader/)
**Difficulty**: Advanced
**Target**: `x86_64-unknown-none`

Minimal OS kernel for x86_64:
- Bootloader crate (BIOS/UEFI boot)
- VGA text mode output
- IDT (Interrupt Descriptor Table)
- Exception handling
- Control registers (CR3, etc.)
- Creating bootable disk images

**Key concepts**: OS development, x86_64 architecture, boot process

**Build**: Requires nightly Rust and bootimage
```bash
rustup default nightly
cargo install bootimage
cargo bootimage --manifest-path 05-bootloader/Cargo.toml
```

**Run in QEMU**:
```bash
qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/release/bootimage-bootloader_example.bin
```

### 6. [RISC-V Bare Metal](./06-riscv-baremetal/)
**Difficulty**: Intermediate
**Target**: `riscv32imac-unknown-none-elf`

RISC-V programming:
- `riscv` and `riscv-rt` crates
- CSRs (Control and Status Registers)
- Trap handling (interrupts + exceptions)
- UART communication
- Machine mode programming
- Privilege levels

**Key concepts**: RISC-V ISA, CSRs, trap handling

**Build**: Requires RISC-V target
```bash
rustup target add riscv32imac-unknown-none-elf
cargo build --manifest-path 06-riscv-baremetal/Cargo.toml
```

**Run in QEMU**:
```bash
qemu-system-riscv32 -machine virt -bios none \
    -kernel target/riscv32imac-unknown-none-elf/release/riscv_baremetal \
    -nographic
```

### 7. [Heapless Collections](./07-heapless-collections/)
**Difficulty**: Intermediate
**Target**: Generic (any no_std target)

Data structures without heap allocation:
- `Vec<T, N>` - Fixed-capacity vector
- `String<N>` - Fixed-capacity string
- `Deque<T, N>` - Ring buffer queue
- `IndexMap<K, V, N>` - Hash map
- `HistoryBuffer<T, N>` - Circular buffer
- Lock-free queues (MPMC, SPSC)
- Memory pool patterns

**Key concepts**: Static allocation, bounded collections, real-time safe

## Quick Start

### Prerequisites

```bash
# Install Rust (stable or nightly depending on example)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install targets
rustup target add thumbv7em-none-eabihf      # ARM Cortex-M4F/M7F
rustup target add riscv32imac-unknown-none-elf  # RISC-V 32-bit
rustup target add x86_64-unknown-none        # x86_64 bare metal

# For bootloader example (nightly)
rustup default nightly
cargo install bootimage

# For testing (QEMU)
# Ubuntu/Debian:
sudo apt install qemu-system-arm qemu-system-riscv32 qemu-system-x86

# macOS:
brew install qemu
```

### Building Examples

Each example is a standalone Cargo project:

```bash
# Navigate to an example
cd 01-no-std-basics

# Build (uses .cargo/config.toml for target)
cargo build

# Build for specific target
cargo build --target thumbv7em-none-eabihf

# Build optimized for size
cargo build --release
```

### Running in QEMU

```bash
# ARM Cortex-M (with semi-hosting)
cargo run --manifest-path 04-arm-cortex-m/Cargo.toml

# RISC-V
qemu-system-riscv32 -machine virt -bios none \
    -kernel target/riscv32imac-unknown-none-elf/release/riscv_baremetal \
    -nographic

# x86_64 bootloader
cargo run --manifest-path 05-bootloader/Cargo.toml
```

## Architecture Support

| Architecture | Example | Target Triple |
|--------------|---------|---------------|
| ARM Cortex-M0/M1 | 04-arm-cortex-m | `thumbv6m-none-eabi` |
| ARM Cortex-M3 | 04-arm-cortex-m | `thumbv7m-none-eabi` |
| ARM Cortex-M4/M4F | 04-arm-cortex-m | `thumbv7em-none-eabihf` |
| ARM Cortex-M7F | 04-arm-cortex-m | `thumbv7em-none-eabihf` |
| RISC-V 32-bit | 06-riscv-baremetal | `riscv32imac-unknown-none-elf` |
| RISC-V 64-bit | 06-riscv-baremetal | `riscv64imac-unknown-none-elf` |
| x86_64 | 05-bootloader | `x86_64-unknown-none` |

## Key Concepts Covered

### No-Std Essentials
- Disabling standard library (`#![no_std]`)
- Custom entry points (`#![no_main]`)
- Panic handlers (`#[panic_handler]`)
- Using `core` vs `std`
- Linker scripts (`memory.x`, `link.x`)

### Memory Management
- Stack vs heap allocation
- Global allocators (`#[global_allocator]`)
- Static variables and const
- Volatile memory access
- Memory-mapped I/O

### Interrupts & Exceptions
- Interrupt handlers
- Exception handlers
- Critical sections
- Atomic operations
- Lock-free data structures

### Hardware Abstraction
- embedded-hal traits
- Platform-agnostic drivers
- GPIO, SPI, I2C, UART
- Timers and delays
- Peripheral access

### Advanced Topics
- DMA (Direct Memory Access)
- Real-time guarantees
- Power management
- Bootloaders
- Custom architectures

## Common Patterns

### Entry Point Pattern

```rust
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Your code here
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
```

### Interrupt Handler Pattern

```rust
use cortex_m_rt::interrupt;

#[interrupt]
fn TIM2() {
    // Clear interrupt flag
    // Process interrupt
}
```

### Memory-Mapped I/O Pattern

```rust
const GPIO_BASE: usize = 0x4000_0000;
const GPIO_ODR: *mut u32 = (GPIO_BASE + 0x14) as *mut u32;

unsafe {
    core::ptr::write_volatile(GPIO_ODR, 0x0020); // Set bit 5
}
```

### Static Allocation Pattern

```rust
static mut BUFFER: [u8; 1024] = [0; 1024];

fn use_buffer() {
    unsafe {
        BUFFER[0] = 0x42;
    }
}
```

## Troubleshooting

### Linker Errors

```
error: linking with `rust-lld` failed: exit code: 1
```

**Solution**: Ensure `memory.x` is in project root or check `.cargo/config.toml`

### Missing Panic Handler

```
error: `#[panic_handler]` function required, but not found
```

**Solution**: Add panic handler or use `panic-halt` crate

### Target Not Installed

```
error: target 'thumbv7em-none-eabihf' not found
```

**Solution**: Install target with `rustup target add thumbv7em-none-eabihf`

### Stack Overflow

**Symptoms**: Program hangs or crashes
**Solution**: Increase stack size in `memory.x`:

```ld
_stack_size = 0x4000;  /* 16KB stack */
```

## Resources

### Documentation
- [The Embedded Rust Book](https://rust-embedded.github.io/book/)
- [Embedonomicon](https://docs.rust-embedded.org/embedonomicon/)
- [cortex-m Quickstart](https://github.com/rust-embedded/cortex-m-quickstart)

### Crates
- [cortex-m](https://docs.rs/cortex-m/) - ARM Cortex-M support
- [riscv](https://docs.rs/riscv/) - RISC-V support
- [embedded-hal](https://docs.rs/embedded-hal/) - Hardware abstraction
- [heapless](https://docs.rs/heapless/) - Collections without heap

### Communities
- [Embedded Rust Matrix](https://matrix.to/#/#rust-embedded:matrix.org)
- [Embedded Rust on Reddit](https://www.reddit.com/r/embedded/)
- [awesome-embedded-rust](https://github.com/rust-embedded/awesome-embedded-rust)

### Hardware Platforms
- **STM32**: [stm32-rs](https://github.com/stm32-rs)
- **nRF52**: [nrf-rs](https://github.com/nrf-rs)
- **ESP32**: [esp-rs](https://github.com/esp-rs)
- **RP2040**: [rp-rs](https://github.com/rp-rs)
- **RISC-V**: [riscv-rust](https://github.com/riscv-rust)

## Next Steps

After completing these examples:

1. **Get Real Hardware**: Try examples on actual development boards
2. **Write Drivers**: Create drivers for sensors and peripherals
3. **Build Projects**: Combine concepts into real applications
4. **Contribute**: Share your drivers and projects with community
5. **Advanced Topics**: DMA, async/await, RTOS integration

## License

These examples are provided for educational purposes. See individual files for license information.

## Contributing

Found an issue or want to improve an example? Contributions are welcome!
