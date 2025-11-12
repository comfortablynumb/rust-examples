/// Linked list allocator using the linked_list_allocator crate
///
/// This allocator:
/// - Can free individual allocations
/// - Maintains a linked list of free blocks
/// - More complex than bump allocator but more flexible
/// - Good for general-purpose heap allocation
///
/// Memory layout:
/// ```
/// [allocated][free block]->[free block]->[allocated]
///             ^              ^
///             |              |
///         linked list of free blocks
/// ```

use linked_list_allocator::LockedHeap;

/// Size of the heap in bytes (64 KB)
const HEAP_SIZE: usize = 64 * 1024;

/// Static heap memory
/// In a real embedded system, this might be placed in a specific
/// RAM region via the linker script
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

/// Global allocator instance
#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Initialize the heap
///
/// # Safety
/// Must be called exactly once before any allocations
pub unsafe fn init_heap() {
    ALLOCATOR.lock().init(HEAP.as_mut_ptr(), HEAP_SIZE);
}

/// Get heap statistics (if you want to implement monitoring)
pub fn heap_stats() -> (usize, usize) {
    // The linked_list_allocator doesn't provide built-in stats
    // In a real system, you might track this yourself
    (HEAP_SIZE, 0)
}

// Example of a custom implementation (for educational purposes)
#[allow(dead_code)]
mod custom_implementation {
    use core::alloc::{GlobalAlloc, Layout};
    use core::ptr;

    /// A simple free list node
    struct FreeBlock {
        size: usize,
        next: Option<*mut FreeBlock>,
    }

    /// A simple linked-list allocator implementation
    pub struct SimpleAllocator {
        head: *mut FreeBlock,
    }

    impl SimpleAllocator {
        pub const fn new() -> Self {
            Self {
                head: ptr::null_mut(),
            }
        }

        pub unsafe fn init(&mut self, heap_start: *mut u8, heap_size: usize) {
            // Create initial free block spanning entire heap
            self.head = heap_start as *mut FreeBlock;
            (*self.head).size = heap_size;
            (*self.head).next = None;
        }

        unsafe fn find_free_block(&mut self, size: usize, align: usize)
            -> Option<(*mut FreeBlock, *mut FreeBlock)> {
            let mut current = self.head;
            let mut prev: *mut FreeBlock = ptr::null_mut();

            while !current.is_null() {
                let block = &mut *current;
                let block_addr = current as usize;
                let aligned_addr = (block_addr + align - 1) & !(align - 1);
                let padding = aligned_addr - block_addr;

                if block.size >= size + padding {
                    return Some((prev, current));
                }

                prev = current;
                current = block.next.unwrap_or(ptr::null_mut());
            }

            None
        }
    }

    unsafe impl GlobalAlloc for SimpleAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            // This is simplified - a real implementation would need locking
            ptr::null_mut()
        }

        unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
            // Add block back to free list
        }
    }
}
