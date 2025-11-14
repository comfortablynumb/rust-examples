// Disable the standard library - we're in a bare metal environment
#![no_std]
// Disable the standard main function - we'll define our own entry point
#![no_main]

use core::panic::PanicInfo;

/// This is the entry point for our bare metal program
/// The #[no_mangle] attribute prevents the Rust compiler from mangling the name
/// so the linker can find it as "_start"
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // In a real bare metal application, you would:
    // 1. Initialize hardware (clocks, RAM, etc.)
    // 2. Set up interrupt vectors
    // 3. Configure peripherals
    // 4. Run your main application logic

    // Example: Simple counter that demonstrates no_std code execution
    let mut counter: u32 = 0;

    loop {
        counter = counter.wrapping_add(1);

        // In a real application, you might:
        // - Toggle an LED
        // - Send data over UART
        // - Process sensor data

        // Prevent optimization from removing the loop
        core::hint::black_box(counter);
    }
}

/// Panic handler - required in no_std environments
/// This function is called when a panic occurs
/// Since we don't have std::println! or file I/O, we need to handle panics differently
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // In a real embedded system, you might:
    // 1. Log panic information to UART or flash memory
    // 2. Blink an LED in a specific pattern to indicate error
    // 3. Reset the system
    // 4. Enter a low-power state

    // For now, we just loop forever
    loop {
        // Halt execution
        core::hint::spin_loop();
    }
}

// Note: In the core library (no_std), you still have access to:
// - Basic types: u8, i32, bool, char, etc.
// - References and slices: &T, &mut T, &[T]
// - Core traits: Copy, Clone, Debug, etc.
// - Iterators
// - Option and Result
// - Formatting (core::fmt)
// - Atomics (core::sync::atomic)
// - SIMD (core::arch)
// - Intrinsics (core::intrinsics)
//
// You do NOT have:
// - File I/O
// - Network I/O
// - Threading (unless provided by the platform)
// - Dynamic memory allocation (unless you provide an allocator)
// - Environment variables
// - Command-line arguments
// - println! macro (but you can use core::fmt)

/// Example of using core library features
fn core_library_examples() {
    // Option and Result work fine
    let x: Option<u32> = Some(42);
    let y: Result<u32, &str> = Ok(42);

    // Iterators are available
    let arr = [1, 2, 3, 4, 5];
    let sum: u32 = arr.iter().sum();

    // Formatting works (though printing requires platform support)
    use core::fmt::Write;

    // Prevent unused variable warnings
    let _ = (x, y, sum);
}

/// Example of a simple data structure in no_std
#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn distance_squared(&self, other: &Point) -> i32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
}

/// Example of const evaluation in no_std
const ORIGIN: Point = Point::new(0, 0);
const POINT_A: Point = Point::new(3, 4);
