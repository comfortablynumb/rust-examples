# ARM Cortex-M Bare Metal Example

This example demonstrates bare metal programming for ARM Cortex-M microcontrollers using the `cortex-m` and `cortex-m-rt` crates.

## Overview

ARM Cortex-M is a family of 32-bit RISC processors designed for embedded systems:

- **Cortex-M0/M0+**: Ultra low power, simple (ARMv6-M)
- **Cortex-M3**: Performance and efficiency (ARMv7-M)
- **Cortex-M4/M4F**: DSP and optional FPU (ARMv7E-M)
- **Cortex-M7**: High performance with FPU (ARMv7E-M)
- **Cortex-M33**: TrustZone security (ARMv8-M)

## Key Components

### cortex-m

The `cortex-m` crate provides:
- Access to core peripherals (NVIC, SCB, SysTick, etc.)
- Interrupt management
- Assembly instructions (WFI, NOP, etc.)
- Critical sections
- Atomic operations

### cortex-m-rt

The `cortex-m-rt` crate provides:
- Startup code and runtime
- Vector table
- Reset handler
- Memory initialization
- Exception handlers

## Memory Layout

ARM Cortex-M uses a fixed memory map:

```
0x0000_0000  ┌─────────────┐
             │    Code     │  Flash memory
             │  (FLASH)    │  Read-only, persistent
0x0800_0000  ├─────────────┤  (address varies)
             │             │
0x2000_0000  ├─────────────┤
             │    SRAM     │  Random Access Memory
             │    (RAM)    │  Read-write, volatile
             ├─────────────┤
             │   Stack     │  Grows downward
             └─────────────┘
0xE000_0000  ┌─────────────┐
             │ Peripherals │  Memory-mapped I/O
             └─────────────┘
```

### memory.x

The `memory.x` file defines the memory regions:

```ld
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 256K
  RAM   : ORIGIN = 0x20000000, LENGTH = 64K
}
```

**Important**: Adjust these values for your specific MCU!

## Interrupts and Exceptions

### Core Exceptions

Built into all Cortex-M processors:

```rust
#[exception]
fn SysTick() {
    // System tick timer interrupt
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    // Hardware fault (illegal memory access, etc.)
}

#[exception]
fn DefaultHandler(irqn: i16) {
    // Catch-all for unhandled interrupts
}
```

### Peripheral Interrupts

Device-specific interrupts (defined by MCU vendor):

```rust
#[interrupt]
fn TIM2() {
    // Timer 2 interrupt
}

#[interrupt]
fn USART1() {
    // UART interrupt
}
```

### NVIC - Nested Vectored Interrupt Controller

```rust
use cortex_m::peripheral::NVIC;

// Enable an interrupt
unsafe {
    NVIC::unmask(Interrupt::TIM2);
}

// Set interrupt priority (0 = highest)
unsafe {
    let mut nvic = NVIC::steal();
    nvic.set_priority(Interrupt::TIM2, 1);
}

// Pend (trigger) an interrupt
NVIC::pend(Interrupt::TIM2);
```

## Core Peripherals

### SysTick Timer

24-bit countdown timer available on all Cortex-M:

```rust
use cortex_m::peripheral::syst::SystClkSource;

fn setup_systick(syst: &mut SYST) {
    syst.set_reload(8_000_000 - 1);  // 1 second at 8MHz
    syst.clear_current();
    syst.set_clock_source(SystClkSource::Core);
    syst.enable_counter();
    syst.enable_interrupt();
}
```

### SCB - System Control Block

```rust
use cortex_m::peripheral::SCB;

// Software reset
SCB::sys_reset();

// Set vector table offset
scb.vtor.write(0x0800_0000);
```

### DWT - Data Watchpoint and Trace

Cycle counter for precise timing:

```rust
use cortex_m::peripheral::DWT;

// Enable cycle counter
unsafe {
    dwt.enable_cycle_counter();
}

// Measure cycles
let start = DWT::cycle_count();
do_work();
let cycles = DWT::cycle_count() - start;
```

## Assembly Instructions

The `cortex-m::asm` module provides inline assembly:

```rust
use cortex_m::asm;

// Wait for interrupt (low power)
asm::wfi();

// Wait for event
asm::wfe();

// Send event
asm::sev();

// No operation
asm::nop();

// Breakpoint (for debugging)
asm::bkpt();

// Data synchronization barrier
asm::dsb();

// Instruction synchronization barrier
asm::isb();
```

## Critical Sections

Safely access shared mutable data:

```rust
use cortex_m::interrupt;

static mut SHARED: u32 = 0;

fn access_shared() {
    interrupt::free(|_| {
        unsafe {
            SHARED += 1;
        }
    });
}
```

Or use `Mutex`:

```rust
use cortex_m::interrupt::Mutex;
use core::cell::RefCell;

static SHARED: Mutex<RefCell<u32>> = Mutex::new(RefCell::new(0));

fn access_shared() {
    interrupt::free(|cs| {
        *SHARED.borrow(cs).borrow_mut() += 1;
    });
}
```

## Building and Flashing

### Install Target

```bash
# Cortex-M0/M0+/M1
rustup target add thumbv6m-none-eabi

# Cortex-M3
rustup target add thumbv7m-none-eabi

# Cortex-M4/M4F (soft float)
rustup target add thumbv7em-none-eabi

# Cortex-M4F/M7F (hard float)
rustup target add thumbv7em-none-eabihf

# Cortex-M33
rustup target add thumbv8m.main-none-eabi
```

### Build

```bash
cargo build --release
```

The `.cargo/config.toml` file sets the default target, so you don't need `--target`.

### Size Optimization

Check binary size:

```bash
cargo size --release -- -A
```

Optimize for size in `Cargo.toml`:

```toml
[profile.release]
opt-level = "z"      # Optimize for size
lto = true           # Link-time optimization
codegen-units = 1    # Better optimization
```

### Flashing

Different tools for different debug probes:

#### Using probe-rs (recommended)

```bash
cargo install probe-rs-tools
cargo flash --chip STM32F407VGTx --release
```

#### Using OpenOCD

```bash
# Start OpenOCD
openocd -f interface/stlink.cfg -f target/stm32f4x.cfg

# In another terminal, use GDB
arm-none-eabi-gdb target/thumbv7em-none-eabihf/release/app
(gdb) target remote :3333
(gdb) load
(gdb) continue
```

#### Using st-flash (STM32 only)

```bash
st-flash write target/thumbv7em-none-eabihf/release/app.bin 0x8000000
```

## Debugging

### GDB Debugging

```bash
# Start OpenOCD
openocd -f interface/stlink.cfg -f target/stm32f4x.cfg

# In another terminal
arm-none-eabi-gdb target/thumbv7em-none-eabihf/release/app
```

`.gdbinit`:
```gdb
target remote :3333
load
break main
continue
```

### RTT (Real-Time Transfer)

Print debug messages without UART:

```rust
use rtt_target::{rtt_init_print, rprintln};

rtt_init_print!();
rprintln!("Hello from RTT!");
```

### Semihosting

Debug printing via debugger:

```rust
use cortex_m_semihosting::hprintln;

hprintln!("Debug message").ok();
```

**Note**: Semihosting is very slow!

## Common Patterns

### Peripheral Access

```rust
// Take peripherals (can only be done once)
let dp = stm32::Peripherals::take().unwrap();
let cp = cortex_m::Peripherals::take().unwrap();

// Access registers
dp.GPIOA.odr.write(|w| w.odr5().set_bit());
```

### Delay

```rust
use cortex_m::asm::delay;

// Busy-wait delay (depends on CPU frequency)
delay(8_000_000); // ~1 second at 8MHz
```

### Interrupt-driven Design

```rust
static FLAG: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

#[interrupt]
fn EXTI0() {
    interrupt::free(|cs| {
        FLAG.borrow(cs).set(true);
    });
}

fn main() -> ! {
    loop {
        let flag = interrupt::free(|cs| FLAG.borrow(cs).get());
        if flag {
            // Handle event
            interrupt::free(|cs| FLAG.borrow(cs).set(false));
        }
        asm::wfi();
    }
}
```

## Target Selection

Choose the correct target for your MCU:

| MCU | Core | Float | Target |
|-----|------|-------|--------|
| STM32F0 | M0 | No | `thumbv6m-none-eabi` |
| STM32F1 | M3 | No | `thumbv7m-none-eabi` |
| STM32F3 | M4 | No | `thumbv7em-none-eabi` |
| STM32F4 | M4F | Yes | `thumbv7em-none-eabihf` |
| STM32F7 | M7F | Yes | `thumbv7em-none-eabihf` |
| nRF51 | M0 | No | `thumbv6m-none-eabi` |
| nRF52 | M4F | Yes | `thumbv7em-none-eabihf` |
| RP2040 | M0+ | No | `thumbv6m-none-eabi` |

## Complete Example

See the source code for a working example that:
- Configures SysTick timer
- Handles interrupts
- Implements exception handlers
- Uses low-power mode (WFI)

## References

- [cortex-m crate docs](https://docs.rs/cortex-m/)
- [cortex-m-rt crate docs](https://docs.rs/cortex-m-rt/)
- [ARM Cortex-M Programming Guide](https://developer.arm.com/documentation/)
- [The Embedded Rust Book](https://rust-embedded.github.io/book/)
- [Discovery Book](https://docs.rust-embedded.org/discovery/) - Learn with real hardware
