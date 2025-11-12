# RISC-V Bare Metal Programming

This example demonstrates bare metal programming for RISC-V processors using the `riscv` and `riscv-rt` crates.

## RISC-V Overview

RISC-V is an open-source instruction set architecture (ISA) designed for:
- Modularity: Choose only the extensions you need
- Simplicity: Clean, regular instruction encoding
- Extensibility: Add custom instructions
- Scalability: From microcontrollers to supercomputers

### ISA Naming Convention

Example: `RV32IMAC`
- **RV**: RISC-V
- **32**: 32-bit (also 64, 128)
- **I**: Integer base ISA (required)
- **M**: Multiply/divide extension
- **A**: Atomic operations
- **C**: Compressed instructions (16-bit)

Other common extensions:
- **F**: Single-precision floating point
- **D**: Double-precision floating point
- **G**: General purpose (IMAFD)
- **V**: Vector operations

## Privilege Levels

RISC-V defines three privilege levels:

```
┌─────────────────────────────┐
│  M-mode (Machine)           │  <- Highest privilege
│  - Firmware, bootloader     │     Full hardware access
│  - Interrupt handling       │
├─────────────────────────────┤
│  S-mode (Supervisor)        │  <- OS kernel
│  - Operating system         │
│  - Memory management        │
├─────────────────────────────┤
│  U-mode (User)              │  <- Applications
│  - User applications        │     Limited access
└─────────────────────────────┘
```

This example runs in **M-mode** (bare metal).

## Control and Status Registers (CSRs)

### Machine-Mode CSRs

| CSR | Name | Description |
|-----|------|-------------|
| `mstatus` | Machine Status | Interrupt enable, privilege mode |
| `mie` | Machine Interrupt Enable | Enable specific interrupts |
| `mip` | Machine Interrupt Pending | Pending interrupts |
| `mtvec` | Machine Trap Vector | Trap handler address |
| `mepc` | Machine Exception PC | PC when trap occurred |
| `mcause` | Machine Cause | Trap cause code |
| `mtval` | Machine Trap Value | Additional trap info |
| `mhartid` | Machine Hart ID | Hardware thread ID |

### Reading/Writing CSRs

```rust
use riscv::register::*;

// Read CSR
let mstatus_val = mstatus::read();

// Write CSR
unsafe {
    mstatus::write(0x1800);
}

// Set bits
unsafe {
    mstatus::set_mie();  // Enable interrupts
}

// Clear bits
unsafe {
    mstatus::clear_mie();  // Disable interrupts
}
```

## Interrupt Handling

### Trap Vector

The `mtvec` register points to the trap handler:

```rust
use riscv::register::mtvec;

unsafe {
    // Direct mode: all traps go to same handler
    mtvec::write(trap_handler as usize, mtvec::TrapMode::Direct);

    // Vectored mode: each interrupt has its own handler
    mtvec::write(base_address, mtvec::TrapMode::Vectored);
}
```

### Trap Handler

```rust
#[no_mangle]
unsafe extern "C" fn trap_handler() {
    let cause = mcause::read();

    if cause.is_interrupt() {
        // Handle interrupt
        match cause.code() {
            7 => handle_timer_interrupt(),
            11 => handle_external_interrupt(),
            _ => {}
        }
    } else {
        // Handle exception
        match cause.code() {
            2 => handle_illegal_instruction(),
            _ => {}
        }
    }
}
```

### Interrupt Codes

**Interrupts** (asynchronous):
- 1: Supervisor software interrupt
- 3: Machine software interrupt
- 5: Supervisor timer interrupt
- 7: Machine timer interrupt
- 9: Supervisor external interrupt
- 11: Machine external interrupt

**Exceptions** (synchronous):
- 0: Instruction address misaligned
- 1: Instruction access fault
- 2: Illegal instruction
- 3: Breakpoint
- 4: Load address misaligned
- 5: Load access fault
- 6: Store address misaligned
- 7: Store access fault
- 8: Environment call from U-mode
- 11: Environment call from M-mode

## Assembly Instructions

```rust
use riscv::asm;

// Wait for interrupt (low power mode)
unsafe { asm::wfi(); }

// Environment call (system call)
unsafe { asm::ecall(); }

// Breakpoint
unsafe { asm::ebreak(); }

// Memory fence (synchronization)
unsafe { asm::fence(); }

// No operation
unsafe { asm::nop(); }
```

## UART Communication

RISC-V doesn't define standard peripherals, but most SoCs use 16550 UART:

```rust
const UART_BASE: usize = 0x1000_0000;  // QEMU virt

fn uart_putc(c: u8) {
    unsafe {
        let uart_tx = UART_BASE as *mut u8;
        core::ptr::write_volatile(uart_tx, c);
    }
}

fn uart_getc() -> Option<u8> {
    unsafe {
        let uart_rx = UART_BASE as *const u8;
        let lsr = (UART_BASE + 5) as *const u8;

        // Check if data is ready (LSR bit 0)
        if core::ptr::read_volatile(lsr) & 1 != 0 {
            Some(core::ptr::read_volatile(uart_rx))
        } else {
            None
        }
    }
}
```

## Memory Layout

### QEMU virt Machine

```
0x0000_0000  ┌─────────────┐
             │   Debug     │
0x0010_0000  ├─────────────┤
             │  MROM (64K) │
0x0100_0000  ├─────────────┤
             │   Test      │
0x0200_0000  ├─────────────┤
             │   CLINT     │  Core Local Interruptor
0x0C00_0000  ├─────────────┤
             │   PLIC      │  Platform-Level Interrupt Controller
0x1000_0000  ├─────────────┤
             │   UART      │
0x4000_0000  ├─────────────┤
             │ VirtIO      │
0x8000_0000  ├─────────────┤
             │    RAM      │  Main memory
             │   (128MB)   │
             └─────────────┘
```

## Building and Running

### Prerequisites

```bash
# Install RISC-V target
rustup target add riscv32imac-unknown-none-elf   # 32-bit
rustup target add riscv64imac-unknown-none-elf   # 64-bit
rustup target add riscv64gc-unknown-none-elf     # 64-bit with FPU

# Install QEMU
# Ubuntu/Debian:
sudo apt install qemu-system-riscv32 qemu-system-riscv64

# macOS:
brew install qemu
```

### Build

```bash
cargo build --release
```

### Run in QEMU

```bash
# For 32-bit
qemu-system-riscv32 -machine virt -bios none \
    -kernel target/riscv32imac-unknown-none-elf/release/riscv_baremetal \
    -nographic

# For 64-bit (if targeting riscv64)
qemu-system-riscv64 -machine virt -bios none \
    -kernel target/riscv64imac-unknown-none-elf/release/riscv_baremetal \
    -nographic
```

### QEMU Options

```bash
# More memory
-m 256M

# Multiple cores
-smp 4

# GDB debugging
-s -S

# Exit when powered off
-no-reboot
```

### Exit QEMU

Press `Ctrl+A` then `X`

## Debugging

### GDB Debugging

```bash
# Terminal 1: Start QEMU with GDB server
qemu-system-riscv32 -machine virt -bios none \
    -kernel target/.../riscv_baremetal \
    -nographic -s -S

# Terminal 2: Connect GDB
riscv32-unknown-elf-gdb target/.../riscv_baremetal
(gdb) target remote :1234
(gdb) break main
(gdb) continue
```

### Useful GDB Commands

```gdb
# Show registers
info registers

# Show CSRs
print $mstatus
print $mepc
print $mcause

# Disassemble
disassemble main

# Step instruction
stepi
```

## Target Selection

| Target | Bits | Extensions | Float |
|--------|------|------------|-------|
| `riscv32i-unknown-none-elf` | 32 | I | No |
| `riscv32imac-unknown-none-elf` | 32 | IMAC | No |
| `riscv32imc-unknown-none-elf` | 32 | IMC | No |
| `riscv64imac-unknown-none-elf` | 64 | IMAC | No |
| `riscv64gc-unknown-none-elf` | 64 | GC (IMAFD) | Yes |

Choose based on your hardware:
- **I**: Always included (base integer ISA)
- **M**: If you need multiply/divide
- **A**: If you need atomics
- **C**: If you want smaller code size
- **F/D**: If you have floating point unit

## Real Hardware Examples

### SiFive HiFive1

```toml
[dependencies]
hifive1 = "0.10"
e310x-hal = "0.10"
```

```rust
use hifive1::hal::prelude::*;
use hifive1::hal::DeviceResources;

let dr = DeviceResources::take().unwrap();
let p = dr.peripherals;
let gpio = dr.pins;

// Configure LED
let mut led = gpio.pin1.into_output();
led.set_high();
```

### ESP32-C3

```toml
[dependencies]
esp32c3-hal = "0.13"
```

```rust
use esp32c3_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*};

let peripherals = Peripherals::take();
let system = peripherals.SYSTEM.split();
let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
```

## Common Patterns

### Busy Delay

```rust
fn delay_cycles(cycles: u32) {
    for _ in 0..cycles {
        unsafe { riscv::asm::nop(); }
    }
}
```

### Reading Hart ID

```rust
use riscv::register::mhartid;

let hart_id = mhartid::read();
```

### Safe Interrupt-Free Section

```rust
use riscv::interrupt;

interrupt::free(|_| {
    // Critical section
    // Interrupts are disabled here
});
```

## Performance Counters

RISC-V has performance monitoring CSRs:

```rust
// Cycle counter (always available)
let cycles = riscv::register::mcycle::read();

// Instruction counter
let instret = riscv::register::minstret::read();

// Measure performance
let start = mcycle::read();
do_work();
let end = mcycle::read();
let elapsed = end - start;
```

## Atomics

RISC-V provides atomic instructions (A extension):

```rust
use core::sync::atomic::{AtomicU32, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);

// Atomic increment
COUNTER.fetch_add(1, Ordering::SeqCst);

// Compare and swap
let _ = COUNTER.compare_exchange(0, 1, Ordering::SeqCst, Ordering::Relaxed);
```

## Memory Ordering

```rust
use riscv::asm;

// Full fence (all memory operations)
unsafe { asm::fence(); }

// Fence with specific ordering
// fence(predecessor, successor)
// Options: r (read), w (write), rw (both)
unsafe {
    core::arch::asm!("fence rw, rw");
}
```

## References

- [RISC-V Specifications](https://riscv.org/technical/specifications/)
- [riscv crate docs](https://docs.rs/riscv/)
- [riscv-rt crate docs](https://docs.rs/riscv-rt/)
- [The RISC-V Reader](http://www.riscvbook.com/)
- [RISC-V Assembly Programming](https://github.com/riscv/riscv-asm-manual)
