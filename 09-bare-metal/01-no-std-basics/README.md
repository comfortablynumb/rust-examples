# No-Std Basics

This example demonstrates the fundamental concepts of bare metal Rust programming using `#![no_std]`.

## Key Concepts

### What is `no_std`?

When you write `#![no_std]` at the top of your Rust file, you're telling the compiler not to link the standard library (`std`). Instead, your code only has access to the `core` library, which contains platform-agnostic primitives.

### Why Use `no_std`?

- **Embedded systems**: Microcontrollers without an operating system
- **Bootloaders**: Code that runs before an OS is loaded
- **Kernel development**: Operating system kernels
- **Size constraints**: When you need the smallest possible binary
- **Resource-constrained environments**: Systems with limited RAM/storage

### Required Components

1. **`#![no_std]`**: Disables the standard library
2. **`#![no_main]`**: Disables the standard main function
3. **Entry point**: Custom entry point (usually `_start`)
4. **Panic handler**: Required function to handle panics

## What's Available in `core`?

The `core` library provides:

- ✅ Primitive types (`u8`, `i32`, `bool`, etc.)
- ✅ Slices and arrays
- ✅ `Option` and `Result`
- ✅ Iterators
- ✅ Traits (`Copy`, `Clone`, `Debug`, etc.)
- ✅ Formatting (`core::fmt`)
- ✅ Atomics
- ✅ SIMD types
- ✅ `const fn` evaluation

## What's NOT Available Without `std`?

- ❌ File I/O
- ❌ Network I/O
- ❌ Threading (unless platform provides it)
- ❌ Dynamic memory allocation (unless you provide an allocator)
- ❌ `Vec`, `String`, `HashMap` (need allocator)
- ❌ `println!` macro (need platform-specific output)
- ❌ Environment variables
- ❌ Command-line arguments

## Building

This example doesn't target a specific platform. To build for different targets:

### For x86_64 (won't run but will compile)
```bash
cargo build --target x86_64-unknown-none
```

### For ARM Cortex-M (example)
```bash
rustup target add thumbv7em-none-eabihf
cargo build --target thumbv7em-none-eabihf
```

### For RISC-V (example)
```bash
rustup target add riscv32imac-unknown-none-elf
cargo build --target riscv32imac-unknown-none-elf
```

## Understanding the Code

### Entry Point

```rust
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Your code here
    loop {}
}
```

- `#[no_mangle]`: Prevents name mangling so linker can find it
- `extern "C"`: Uses C calling convention
- `_start`: Common entry point name (can vary by platform)
- `-> !`: Never returns (diverging function)

### Panic Handler

```rust
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
```

- Required in all `no_std` binaries
- Called when `panic!()` is invoked
- Must never return (hence `-> !`)
- Receives `PanicInfo` with panic details

## Common Patterns

### Infinite Loops

Bare metal programs typically run forever:

```rust
loop {
    // Do work
}
```

### Volatile Access

When working with hardware registers:

```rust
use core::ptr::{read_volatile, write_volatile};

unsafe {
    write_volatile(0x4000_0000 as *mut u32, 0x1234);
    let value = read_volatile(0x4000_0000 as *const u32);
}
```

### No Heap Allocations

Use stack allocation or static variables:

```rust
// Stack allocation
let buffer = [0u8; 1024];

// Static allocation
static mut BUFFER: [u8; 1024] = [0; 1024];
```

## Next Steps

See the other examples in this directory:
- `02-custom-allocator`: Add dynamic memory allocation
- `03-embedded-hal`: Write portable embedded code
- `04-arm-cortex-m`: Target real ARM hardware
- `05-bootloader`: Create a minimal OS
- `06-riscv-baremetal`: RISC-V bare metal programming
- `07-heapless-collections`: Data structures without heap

## References

- [The Embedded Rust Book](https://rust-embedded.github.io/book/)
- [Rust core library documentation](https://doc.rust-lang.org/core/)
- [no_std documentation](https://docs.rust-embedded.org/book/intro/no-std.html)
