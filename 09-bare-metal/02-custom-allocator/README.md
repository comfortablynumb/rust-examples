# Custom Memory Allocator

This example demonstrates how to implement and use custom memory allocators in bare metal Rust environments.

## Overview

In `no_std` environments, there's no default allocator. To use heap-allocated types like `Vec`, `String`, `Box`, etc., you must provide a global allocator using `#[global_allocator]`.

## Allocator Types

### 1. Bump Allocator (Linear Allocator)

**Characteristics:**
- Extremely fast allocation (just increment a pointer)
- Cannot free individual allocations
- Only supports freeing all memory at once (reset)
- Best for temporary/per-frame allocations

**Use cases:**
- Game frame allocations
- Temporary scratch space
- Parser/compiler arena allocations

**Pros:**
- Fastest possible allocation
- Simple implementation
- No fragmentation

**Cons:**
- Cannot free individual allocations
- Memory can only be reclaimed by reset

### 2. Linked List Allocator

**Characteristics:**
- Maintains linked list of free blocks
- Can free individual allocations
- More complex than bump allocator
- General-purpose allocator

**Use cases:**
- General heap allocation
- Long-lived allocations with varying sizes
- When you need to free individual allocations

**Pros:**
- Can free memory
- Works with standard allocation patterns

**Cons:**
- Slower than bump allocator
- Can fragment over time
- Requires more bookkeeping

## Required Features

### `#![feature(alloc_error_handler)]`

Allows you to define a custom handler for allocation failures:

```rust
#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("Allocation failed: {:?}", layout);
}
```

### `extern crate alloc`

Imports the `alloc` crate, which provides heap-allocated types:
- `Vec<T>` - Dynamically sized array
- `String` - Dynamically sized string
- `Box<T>` - Heap-allocated single value
- `Arc<T>` - Atomic reference counted pointer
- `Rc<T>` - Reference counted pointer (not thread-safe)

## Implementation Details

### Global Allocator

```rust
#[global_allocator]
static ALLOCATOR: MyAllocator = MyAllocator::new();
```

The global allocator must implement the `GlobalAlloc` trait:

```rust
unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Return pointer to allocated memory
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // Free the memory
    }
}
```

### Memory Layout

In embedded systems, you typically define a static array for heap memory:

```rust
static mut HEAP: [u8; 64 * 1024] = [0; 64 * 1024];
```

In production, this might be placed in a specific memory region via linker script:

```ld
.bss.heap (NOLOAD) : {
    _heap_start = .;
    . = . + 0x10000; /* 64 KB heap */
    _heap_end = .;
} > RAM
```

## Building

This example requires nightly Rust for the `alloc_error_handler` feature:

```bash
# Build for a no_std target
cargo +nightly build --target thumbv7em-none-eabihf
```

### Installing Targets

```bash
# ARM Cortex-M4F/M7F
rustup target add thumbv7em-none-eabihf

# ARM Cortex-M0/M1
rustup target add thumbv6m-none-eabi

# RISC-V
rustup target add riscv32imac-unknown-none-elf
```

## Using the Allocator

Once you've set up a global allocator, you can use heap types:

```rust
extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;
use alloc::boxed::Box;

// Initialize heap first!
unsafe {
    init_heap();
}

// Now use heap types
let mut vec = Vec::new();
vec.push(1);
vec.push(2);

let text = String::from("Hello!");
let boxed = Box::new(42);
```

## Common Patterns

### Arena Allocation

Use bump allocator with periodic resets:

```rust
// Allocate for frame
let data = Vec::with_capacity(100);
// ... use data ...

// At end of frame
unsafe {
    ALLOCATOR.reset();
}
```

### Pool Allocation

Pre-allocate fixed-size blocks:

```rust
const BLOCK_SIZE: usize = 64;
const POOL_SIZE: usize = 100;

static mut POOL: [u8; BLOCK_SIZE * POOL_SIZE] = [0; BLOCK_SIZE * POOL_SIZE];
```

### Hybrid Approach

Use different allocators for different purposes:

```rust
#[global_allocator]
static ALLOCATOR: DualAllocator = DualAllocator::new();

struct DualAllocator {
    bump: BumpAllocator,    // For temporary allocations
    heap: LinkedListAlloc,  // For permanent allocations
}
```

## Safety Considerations

1. **Initialization**: Always initialize the heap before any allocations
2. **Thread Safety**: Allocator must be `Sync` for multi-threaded use
3. **Alignment**: Respect alignment requirements in `Layout`
4. **Null Checks**: Check for null pointers after allocation
5. **Memory Regions**: Ensure heap doesn't overlap with stack/code

## Advanced Topics

### Custom `Box` Implementation

```rust
use core::ops::{Deref, DerefMut};

struct MyBox<T> {
    ptr: *mut T,
}

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        unsafe {
            let layout = Layout::new::<T>();
            let ptr = alloc(layout) as *mut T;
            ptr.write(value);
            Self { ptr }
        }
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::new::<T>();
            ptr::drop_in_place(self.ptr);
            dealloc(self.ptr as *mut u8, layout);
        }
    }
}
```

### Memory Tracking

```rust
use core::sync::atomic::{AtomicUsize, Ordering};

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

fn track_alloc(size: usize) {
    ALLOCATED.fetch_add(size, Ordering::SeqCst);
}

fn track_dealloc(size: usize) {
    ALLOCATED.fetch_sub(size, Ordering::SeqCst);
}
```

## References

- [linked_list_allocator crate](https://docs.rs/linked_list_allocator/)
- [GlobalAlloc trait](https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html)
- [The Embedded Rust Book - Collections](https://rust-embedded.github.io/book/collections/)
- [Writing an OS in Rust - Heap Allocation](https://os.phil-opp.com/heap-allocation/)
