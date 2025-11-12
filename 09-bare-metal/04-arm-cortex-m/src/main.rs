#![no_std]
#![no_main]

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use panic_halt as _;

/// Static mutable variable for demonstration
/// In Cortex-M, we can use `cortex_m::interrupt::Mutex` for safe sharing
static mut COUNTER: u32 = 0;

/// Entry point - called by cortex-m-rt
///
/// The #[entry] macro:
/// - Sets up the stack pointer
/// - Calls this function from reset handler
/// - Never returns
#[entry]
fn main() -> ! {
    // Get access to core peripherals
    // These are the ARM Cortex-M core peripherals, available on all Cortex-M chips
    let mut core = cortex_m::Peripherals::take().unwrap();

    // Configure the SysTick timer to generate interrupts
    // SysTick is a simple 24-bit countdown timer built into all Cortex-M processors
    configure_systick(&mut core.SYST);

    // Enable interrupts globally
    unsafe {
        cortex_m::peripheral::NVIC::unmask(cortex_m::peripheral::nvic::Type::SYSTICK);
    }

    // Main loop
    loop {
        // Wait for interrupt (low power mode)
        cortex_m::asm::wfi();

        // After waking from interrupt, do some work
        unsafe {
            if COUNTER >= 1000 {
                // Do something every 1000 ticks
                COUNTER = 0;
            }
        }
    }
}

/// Configure SysTick timer
fn configure_systick(syst: &mut cortex_m::peripheral::SYST) {
    // Set reload value (number of clock cycles between interrupts)
    // Assuming 8MHz clock, this gives ~1ms interrupts
    syst.set_reload(8_000 - 1);

    // Clear current value
    syst.clear_current();

    // Use processor clock as source
    syst.set_clock_source(SystClkSource::Core);

    // Enable counter and interrupt
    syst.enable_counter();
    syst.enable_interrupt();
}

/// SysTick exception handler
///
/// This is called automatically when SysTick counter reaches zero
#[exception]
fn SysTick() {
    unsafe {
        COUNTER += 1;

        // In a real application, you might:
        // - Update a real-time clock
        // - Implement a scheduler
        // - Trigger periodic tasks
        // - Debounce buttons
    }
}

/// Hard Fault exception handler
///
/// This is called when a hardware fault occurs (e.g., illegal memory access)
#[exception]
unsafe fn HardFault(_ef: &cortex_m_rt::ExceptionFrame) -> ! {
    // In a real application, you might:
    // 1. Log the fault information
    // 2. Blink an LED in an error pattern
    // 3. Attempt recovery
    // 4. Reset the system

    loop {
        cortex_m::asm::nop();
    }
}

/// Default exception handler
///
/// This is called for any exception that doesn't have a specific handler
#[exception]
unsafe fn DefaultHandler(_irqn: i16) {
    loop {
        cortex_m::asm::nop();
    }
}

// Note: Additional Cortex-M features available:
//
// 1. NVIC - Nested Vectored Interrupt Controller
//    - Enable/disable interrupts
//    - Set interrupt priorities
//    - Pend/clear interrupts
//
// 2. SCB - System Control Block
//    - Vector table offset
//    - Sleep configuration
//    - System reset
//
// 3. MPU - Memory Protection Unit (if available)
//    - Memory region protection
//    - Access permissions
//
// 4. FPU - Floating Point Unit (on Cortex-M4F/M7)
//    - Hardware floating point
//
// 5. ITM/DWT - Instrumentation/Debug
//    - Debug printing
//    - Cycle counting
//    - Performance monitoring
