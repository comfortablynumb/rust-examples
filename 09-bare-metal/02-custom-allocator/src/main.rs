#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::panic::PanicInfo;

mod bump_allocator;
mod linked_list_alloc;

// Choose which allocator to use:
// Option 1: Simple bump allocator (fast but can't free)
// use bump_allocator::BumpAllocator;
// #[global_allocator]
// static ALLOCATOR: BumpAllocator = BumpAllocator::new();

// Option 2: Linked list allocator (can free memory)
use linked_list_alloc::ALLOCATOR;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize the heap before using any allocations
    unsafe {
        linked_list_alloc::init_heap();
    }

    // Now we can use heap-allocated types!
    demonstrate_allocations();

    loop {
        core::hint::spin_loop();
    }
}

/// Demonstrate various heap allocation patterns
fn demonstrate_allocations() {
    // Vec - dynamically sized array
    let mut numbers = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    let _ = numbers;

    // String - dynamically sized string
    let mut text = String::from("Hello, ");
    text.push_str("bare metal!");
    let _ = text;

    // Box - heap-allocated single value
    let boxed_value = Box::new(42);
    let _ = boxed_value;

    // Array on heap
    let array: Vec<u32> = (0..10).collect();
    let _ = array;

    // Nested structures
    let nested: Vec<Vec<u32>> = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let _ = nested;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    // Called when allocation fails
    // In a real system, you might:
    // - Log the error
    // - Attempt recovery
    // - Reset the system
    let _ = layout;
    panic!("Allocation error");
}
