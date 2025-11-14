#![no_std]
#![no_main]

use core::panic::PanicInfo;
use heapless::{mpmc::Q8, Deque, FnvIndexMap, HistoryBuffer, String, Vec};

/// Entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Demonstrate all heapless collection types
    demonstrate_vec();
    demonstrate_string();
    demonstrate_deque();
    demonstrate_map();
    demonstrate_history_buffer();
    demonstrate_queue();
    demonstrate_pool();

    loop {
        core::hint::spin_loop();
    }
}

/// Heapless Vec - fixed-capacity vector
///
/// Like std::vec::Vec but with compile-time capacity
/// No dynamic allocation, all memory is on the stack
fn demonstrate_vec() {
    // Create a Vec with capacity for 8 elements
    // Type: Vec<i32, 8>
    let mut vec: Vec<i32, 8> = Vec::new();

    // Push elements (returns Result because it can fail if full)
    vec.push(1).ok();
    vec.push(2).ok();
    vec.push(3).ok();

    // Iterate
    for item in &vec {
        let _ = item;
    }

    // Access by index
    if let Some(&first) = vec.get(0) {
        let _ = first;
    }

    // Pop elements
    let _last = vec.pop();

    // Check capacity and length
    let _cap = vec.capacity(); // Always 8
    let _len = vec.len(); // Current number of elements

    // Try to push when full
    let mut full_vec: Vec<u8, 4> = Vec::new();
    for i in 0..4 {
        full_vec.push(i).ok();
    }

    // This will fail
    match full_vec.push(5) {
        Ok(_) => {
            // Success
        }
        Err(_) => {
            // Vec is full!
        }
    }

    // Clear the vec
    vec.clear();

    // Create from array
    let from_array: Vec<u8, 16> = Vec::from_slice(&[1, 2, 3, 4]).unwrap();
    let _ = from_array;
}

/// Heapless String - fixed-capacity string
///
/// Like std::string::String but with compile-time capacity
fn demonstrate_string() {
    // Create a String with capacity for 32 bytes
    let mut s: String<32> = String::new();

    // Push a character
    s.push('H').ok();
    s.push('i').ok();

    // Push a string slice
    s.push_str(" there").ok();

    // Convert to &str
    let as_str: &str = &s;
    let _ = as_str;

    // Check if full
    if !s.is_full() {
        s.push('!').ok();
    }

    // Create from string literal
    let hello: String<16> = String::from("Hello, world!");
    let _ = hello;

    // Clear
    s.clear();

    // Format into string (requires std::fmt::Write)
    use core::fmt::Write;
    let mut formatted: String<64> = String::new();
    write!(&mut formatted, "Number: {}, Hex: {:#x}", 42, 255).ok();
    let _ = formatted;
}

/// Heapless Deque - double-ended queue
///
/// Efficient insertion/removal at both ends
/// Ring buffer implementation
fn demonstrate_deque() {
    // Create a deque with capacity for 16 elements
    let mut deque: Deque<u8, 16> = Deque::new();

    // Push to back
    deque.push_back(1).ok();
    deque.push_back(2).ok();

    // Push to front
    deque.push_front(0).ok();

    // Pop from back
    let _back = deque.pop_back(); // Some(2)

    // Pop from front
    let _front = deque.pop_front(); // Some(0)

    // Peek without removing
    let _peek_front = deque.front(); // Some(&1)
    let _peek_back = deque.back(); // Some(&1)

    // Use as a queue (FIFO)
    let mut queue: Deque<u32, 8> = Deque::new();
    queue.push_back(1).ok(); // Enqueue
    queue.push_back(2).ok();
    let _item = queue.pop_front(); // Dequeue

    // Use as a stack (LIFO)
    let mut stack: Deque<u32, 8> = Deque::new();
    stack.push_back(1).ok(); // Push
    stack.push_back(2).ok();
    let _item = stack.pop_back(); // Pop
}

/// Heapless IndexMap - hash map with fixed capacity
///
/// Like std::collections::HashMap but with compile-time capacity
fn demonstrate_map() {
    // Create a map with capacity for 8 entries
    // Uses FNV hashing (fast for small keys)
    let mut map: FnvIndexMap<&str, u32, 8> = FnvIndexMap::new();

    // Insert key-value pairs
    map.insert("one", 1).ok();
    map.insert("two", 2).ok();
    map.insert("three", 3).ok();

    // Get values
    if let Some(&value) = map.get("two") {
        let _ = value; // 2
    }

    // Check if key exists
    if map.contains_key("one") {
        // Key exists
    }

    // Remove entries
    let _removed = map.remove("one");

    // Iterate over entries
    for (key, value) in &map {
        let _ = (key, value);
    }

    // Update existing entry
    if let Some(value) = map.get_mut("two") {
        *value = 200;
    }

    // Try to insert when full
    let mut small_map: FnvIndexMap<u8, u8, 2> = FnvIndexMap::new();
    small_map.insert(1, 10).ok();
    small_map.insert(2, 20).ok();

    match small_map.insert(3, 30) {
        Ok(_) => {
            // Success
        }
        Err(_) => {
            // Map is full!
        }
    }
}

/// HistoryBuffer - circular buffer that keeps last N items
///
/// Useful for:
/// - Moving average calculations
/// - Recent event history
/// - Signal processing
fn demonstrate_history_buffer() {
    // Create a buffer that keeps the last 8 items
    let mut history: HistoryBuffer<u32, 8> = HistoryBuffer::new();

    // Write values (automatically overwrites oldest)
    for i in 0..12 {
        history.write(i);
    }

    // Buffer now contains: [4, 5, 6, 7, 8, 9, 10, 11]
    // Oldest values (0-3) were overwritten

    // Read recent values (newest to oldest)
    let recent: Vec<u32, 8> = history.recent().cloned().collect();
    let _ = recent;

    // Read oldest values (oldest to newest)
    let oldest: Vec<u32, 8> = history.oldest_ordered().cloned().collect();
    let _ = oldest;

    // Calculate moving average
    let sum: u32 = history.recent().sum();
    let count = history.len();
    let average = if count > 0 { sum / count as u32 } else { 0 };
    let _ = average;

    // Clear the buffer
    history.clear();
}

/// Multi-producer multi-consumer queue
///
/// Lock-free queue for communication between threads/interrupts
/// Useful in embedded systems for ISR to main thread communication
fn demonstrate_queue() {
    // Create a queue with capacity for 8 messages
    static mut QUEUE: Q8<u32> = Q8::new();

    unsafe {
        // Producer side (e.g., in interrupt handler)
        QUEUE.enqueue(42).ok();
        QUEUE.enqueue(43).ok();

        // Consumer side (e.g., in main loop)
        if let Some(value) = QUEUE.dequeue() {
            let _ = value; // 42
        }

        if let Some(value) = QUEUE.dequeue() {
            let _ = value; // 43
        }
    }

    // Example: ISR communication pattern
    fn interrupt_handler() {
        unsafe {
            // Try to enqueue sensor reading
            if QUEUE.enqueue(read_sensor()).is_err() {
                // Queue full, handle overflow
            }
        }
    }

    fn main_loop() {
        unsafe {
            // Process all pending readings
            while let Some(reading) = QUEUE.dequeue() {
                process_reading(reading);
            }
        }
    }

    fn read_sensor() -> u32 {
        0
    }
    fn process_reading(_reading: u32) {}

    // Prevent unused warnings
    let _ = (interrupt_handler, main_loop);
}

/// Memory Pool pattern - pre-allocated object pool
///
/// Useful when you need dynamic allocation behavior
/// but want bounded memory usage
fn demonstrate_pool() {
    // Example: Pool of message buffers
    const POOL_SIZE: usize = 4;
    static mut POOL: [Option<MessageBuffer>; POOL_SIZE] = [None, None, None, None];

    #[derive(Debug, Copy, Clone)]
    struct MessageBuffer {
        data: [u8; 64],
        len: usize,
    }

    impl MessageBuffer {
        fn new() -> Self {
            Self {
                data: [0; 64],
                len: 0,
            }
        }
    }

    // Allocate a buffer from pool
    fn alloc_buffer() -> Option<usize> {
        unsafe {
            for (i, slot) in POOL.iter_mut().enumerate() {
                if slot.is_none() {
                    *slot = Some(MessageBuffer::new());
                    return Some(i);
                }
            }
        }
        None // Pool exhausted
    }

    // Free a buffer back to pool
    fn free_buffer(index: usize) {
        unsafe {
            if index < POOL_SIZE {
                POOL[index] = None;
            }
        }
    }

    // Use the buffer
    if let Some(index) = alloc_buffer() {
        // Use buffer
        unsafe {
            if let Some(buffer) = &mut POOL[index] {
                buffer.data[0] = 0x42;
                buffer.len = 1;
            }
        }

        // Free when done
        free_buffer(index);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

// Summary of Heapless Collections:
//
// 1. Vec<T, N>
//    - Fixed-capacity vector
//    - Like Vec but no heap allocation
//    - Good for: Lists with known maximum size
//
// 2. String<N>
//    - Fixed-capacity string
//    - Like String but no heap allocation
//    - Good for: Text with known maximum length
//
// 3. Deque<T, N>
//    - Double-ended queue (ring buffer)
//    - Efficient push/pop at both ends
//    - Good for: Queues, stacks, sliding windows
//
// 4. IndexMap<K, V, N>
//    - Fixed-capacity hash map
//    - Like HashMap but no heap allocation
//    - Good for: Key-value storage with bounded size
//
// 5. HistoryBuffer<T, N>
//    - Circular buffer keeping last N items
//    - Automatically overwrites oldest
//    - Good for: Moving averages, recent history
//
// 6. Queue (MPMC, SPSC)
//    - Lock-free queues
//    - Thread/interrupt safe
//    - Good for: ISR communication, producer-consumer
//
// All collections:
// - Zero heap allocation
// - Bounded memory usage known at compile time
// - Return Result/Option instead of panicking
// - Perfect for embedded systems!
