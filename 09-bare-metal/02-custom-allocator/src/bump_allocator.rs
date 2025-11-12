/// A simple bump allocator (also known as a linear allocator)
///
/// This allocator:
/// - Is extremely fast (just increments a pointer)
/// - Cannot free individual allocations
/// - Only frees all memory when reset
/// - Good for temporary/per-frame allocations
///
/// Memory layout:
/// ```
/// [allocated][allocated][free space...]
///            ^
///            next pointer
/// ```

use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::ptr;

/// Size of the heap in bytes (64 KB)
const HEAP_SIZE: usize = 64 * 1024;

/// A bump allocator that allocates by incrementing a pointer
pub struct BumpAllocator {
    heap: UnsafeCell<[u8; HEAP_SIZE]>,
    next: UnsafeCell<usize>,
}

impl BumpAllocator {
    /// Create a new bump allocator
    pub const fn new() -> Self {
        Self {
            heap: UnsafeCell::new([0; HEAP_SIZE]),
            next: UnsafeCell::new(0),
        }
    }

    /// Reset the allocator, freeing all allocations
    ///
    /// # Safety
    /// All previous allocations must be considered invalid after this call
    pub unsafe fn reset(&self) {
        *self.next.get() = 0;
    }

    /// Get the amount of heap space used
    pub fn used(&self) -> usize {
        unsafe { *self.next.get() }
    }

    /// Get the total heap size
    pub fn size(&self) -> usize {
        HEAP_SIZE
    }
}

unsafe impl Sync for BumpAllocator {}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let heap_start = self.heap.get() as usize;
        let heap_end = heap_start + HEAP_SIZE;

        // Get current position
        let next = self.next.get();
        let mut current = *next;

        // Align the current position to the required alignment
        let align = layout.align();
        let size = layout.size();

        // Calculate aligned start position
        let start = (heap_start + current + align - 1) & !(align - 1);
        let end = start + size;

        // Check if allocation fits
        if end > heap_end {
            // Out of memory
            return ptr::null_mut();
        }

        // Update next pointer
        *next = end - heap_start;

        start as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocator cannot free individual allocations
        // This is a no-op
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bump_allocator() {
        let allocator = BumpAllocator::new();

        unsafe {
            // Allocate some memory
            let layout = Layout::from_size_align(16, 8).unwrap();
            let ptr1 = allocator.alloc(layout);
            assert!(!ptr1.is_null());

            // Allocate more
            let ptr2 = allocator.alloc(layout);
            assert!(!ptr2.is_null());
            assert!(ptr2 > ptr1);

            // Check used space
            assert!(allocator.used() >= 32);

            // Reset and reallocate
            allocator.reset();
            assert_eq!(allocator.used(), 0);

            let ptr3 = allocator.alloc(layout);
            assert_eq!(ptr1, ptr3); // Should get same address after reset
        }
    }
}
