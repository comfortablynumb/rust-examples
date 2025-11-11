#![allow(dead_code)]
#![allow(static_mut_refs)]
#![allow(unnecessary_transmutes)]
#![allow(clippy::approx_constant)]
#![allow(clippy::missing_safety_doc)]

// Unsafe Rust
//
// Demonstrates unsafe Rust features and when they're necessary.
// Unsafe code opts out of Rust's safety guarantees and must be
// carefully reviewed and documented.

use std::slice;

fn main() {
    println!("=== Unsafe Rust ===\n");

    // Example 1: Raw pointers
    println!("1. Raw Pointers:");
    raw_pointers_example();
    println!();

    // Example 2: Dereferencing raw pointers
    println!("2. Dereferencing Raw Pointers:");
    unsafe {
        dereference_raw_pointers();
    }
    println!();

    // Example 3: Unsafe functions
    println!("3. Unsafe Functions:");
    unsafe_functions_example();
    println!();

    // Example 4: Safe abstraction over unsafe code
    println!("4. Safe Abstraction:");
    safe_abstraction_example();
    println!();

    // Example 5: Mutable static
    println!("5. Mutable Static Variables:");
    unsafe {
        mutable_static_example();
    }
    println!();

    // Example 6: Union types
    println!("6. Union Types:");
    union_example();
    println!();

    // Example 7: FFI (Foreign Function Interface)
    println!("7. FFI (Calling C Functions):");
    ffi_example();
    println!();

    // Example 8: Transmute
    println!("8. Memory Transmutation:");
    unsafe {
        transmute_example();
    }
    println!();

    // Example 9: Unsafe traits
    println!("9. Unsafe Traits:");
    unsafe_trait_example();
    println!();

    println!("=== Unsafe Superpowers ===\n");
    println!("Unsafe Rust allows you to:");
    println!("  1. Dereference raw pointers");
    println!("  2. Call unsafe functions");
    println!("  3. Access mutable static variables");
    println!("  4. Implement unsafe traits");
    println!("  5. Access fields of unions");
    println!();

    println!("Remember: unsafe doesn't disable the borrow checker!");
    println!("It only allows the 5 actions listed above.");
}

// Example 1: Creating and working with raw pointers
fn raw_pointers_example() {
    let mut num = 42;

    // Create raw pointers (safe to create, unsafe to dereference)
    let r1 = &num as *const i32;  // Immutable raw pointer
    let r2 = &mut num as *mut i32;  // Mutable raw pointer

    // Can create raw pointers to arbitrary memory (dangerous!)
    let address = 0x012345usize;
    let _r3 = address as *const i32;

    println!("  Immutable raw pointer: {:p}", r1);
    println!("  Mutable raw pointer: {:p}", r2);
    println!("  Creating raw pointers is safe!");
    println!("  (But dereferencing them requires unsafe)");
}

// Example 2: Dereferencing raw pointers (must be in unsafe block)
unsafe fn dereference_raw_pointers() {
    let mut num = 42;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // Dereference raw pointers (unsafe!)
    println!("  Value through immutable pointer: {}", *r1);

    // Modify through mutable pointer
    *r2 = 100;
    println!("  Modified through mutable pointer: {}", *r2);
    println!("  Original value: {}", num);
}

// Example 3: Unsafe functions
unsafe fn dangerous_function() {
    println!("  Inside an unsafe function!");
    println!("  Entire function body is unsafe context");
}

fn unsafe_functions_example() {
    println!("  Calling an unsafe function:");
    unsafe {
        dangerous_function();
    }

    // Using unsafe to split a slice
    let mut values = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut values, 3);

    println!("  Split slice:");
    println!("    Left: {:?}", left);
    println!("    Right: {:?}", right);
}

// Example 4: Safe abstraction over unsafe code
// This is how std::slice::split_at_mut is implemented
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn safe_abstraction_example() {
    let mut data = vec![1, 2, 3, 4, 5];
    let (left, right) = data.split_at_mut(2);

    left[0] = 10;
    right[0] = 20;

    println!("  Safe wrapper over unsafe code:");
    println!("  Modified data: {:?}", data);
}

// Example 5: Mutable static variables
static mut COUNTER: u32 = 0;

unsafe fn increment_counter() {
    COUNTER += 1;
}

unsafe fn get_counter() -> u32 {
    COUNTER
}

unsafe fn mutable_static_example() {
    increment_counter();
    increment_counter();
    increment_counter();

    println!("  Counter value: {}", get_counter());
    println!("  (Accessing mutable statics requires unsafe)");
}

// Example 6: Union types
#[repr(C)]
union MyUnion {
    int_value: i32,
    float_value: f32,
}

fn union_example() {
    let mut u = MyUnion { int_value: 42 };

    unsafe {
        println!("  As integer: {}", u.int_value);

        u.float_value = 3.14;
        println!("  As float: {}", u.float_value);

        // Reading int_value now is undefined behavior!
        // println!("  As integer (UB!): {}", u.int_value);
    }

    println!("  (Accessing union fields requires unsafe)");
}

// Example 7: FFI (Foreign Function Interface)
extern "C" {
    fn abs(input: i32) -> i32;
    fn strlen(s: *const i8) -> usize;
}

fn ffi_example() {
    unsafe {
        let result = abs(-42);
        println!("  C abs(-42) = {}", result);
    }

    // Using strlen from C
    let c_string = b"Hello from Rust!\0";
    unsafe {
        let len = strlen(c_string.as_ptr() as *const i8);
        println!("  C strlen = {}", len);
    }

    println!("  (Calling foreign functions requires unsafe)");
}

// Example 8: Transmute (reinterpret bytes)
unsafe fn transmute_example() {
    // Convert between types of same size
    let a: f32 = 3.14;
    let b: u32 = std::mem::transmute(a);

    println!("  Float 3.14 as u32 bits: 0x{:08x}", b);

    // Convert back
    let c: f32 = std::mem::transmute(b);
    println!("  Back to float: {}", c);

    println!("  (transmute is very unsafe - use only when necessary!)");
}

// Example 9: Unsafe traits
// Unsafe traits are implemented when the implementer must
// uphold invariants that the compiler can't verify
unsafe trait UnsafeTrait {
    fn do_unsafe_thing(&self);
}

struct MyType {
    value: i32,
}

// Implementing an unsafe trait requires unsafe
unsafe impl UnsafeTrait for MyType {
    fn do_unsafe_thing(&self) {
        println!("  Performing unsafe operation with value: {}", self.value);
    }
}

fn unsafe_trait_example() {
    let my_type = MyType { value: 42 };
    my_type.do_unsafe_thing();
    println!("  (Implementing unsafe traits requires unsafe)");
}

// Additional examples

// Example 10: Pointer arithmetic
#[allow(clippy::unnecessary_cast)]
fn pointer_arithmetic_example() {
    let arr = [1, 2, 3, 4, 5];
    let ptr = arr.as_ptr();

    unsafe {
        println!("\n10. Pointer Arithmetic:");
        for i in 0..5 {
            let value = *ptr.add(i);
            println!("  Element {}: {}", i, value);
        }
    }
}

// Example 11: Inline assembly (x86_64 only, commented out)
// #[cfg(target_arch = "x86_64")]
// fn inline_assembly_example() {
//     use std::arch::asm;
//
//     let result: u64;
//     unsafe {
//         asm!(
//             "mov {}, 42",
//             out(reg) result,
//         );
//     }
//     println!("  Assembly result: {}", result);
// }

// Example 12: Creating a raw pointer from nothing
fn null_pointer_example() {
    // Creating a null pointer is safe
    let _null_ptr: *const i32 = std::ptr::null();
    let _null_mut_ptr: *mut i32 = std::ptr::null_mut();

    println!("\n11. Null Pointers:");
    println!("  Can safely create null pointers");
    println!("  (But dereferencing them is UB!)");
}

// Example 13: Unsafe cell (interior mutability)
use std::cell::UnsafeCell;

struct MyCell {
    value: UnsafeCell<i32>,
}

impl MyCell {
    fn new(value: i32) -> Self {
        MyCell {
            value: UnsafeCell::new(value),
        }
    }

    fn get(&self) -> i32 {
        unsafe { *self.value.get() }
    }

    fn set(&self, value: i32) {
        unsafe {
            *self.value.get() = value;
        }
    }
}

fn unsafe_cell_example() {
    println!("\n12. UnsafeCell (Interior Mutability):");
    let cell = MyCell::new(42);
    println!("  Initial value: {}", cell.get());

    cell.set(100);
    println!("  After set: {}", cell.get());
    println!("  (Building block for Cell and RefCell)");
}

// Example 14: Invariants and safety contracts
/// # Safety
///
/// The caller must ensure that:
/// - `ptr` is valid and properly aligned
/// - `ptr` points to at least `len` initialized elements
/// - The memory referenced by `ptr` is not accessed by other code
unsafe fn sum_array(ptr: *const i32, len: usize) -> i32 {
    let mut sum = 0;
    for i in 0..len {
        sum += *ptr.add(i);
    }
    sum
}

fn safety_contracts_example() {
    println!("\n13. Safety Contracts:");
    let arr = [1, 2, 3, 4, 5];
    let ptr = arr.as_ptr();

    unsafe {
        let total = sum_array(ptr, arr.len());
        println!("  Sum of array: {}", total);
    }

    println!("  (Document safety requirements!)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_at_mut() {
        let mut data = vec![1, 2, 3, 4, 5];
        let (left, right) = split_at_mut(&mut data, 2);

        assert_eq!(left, &[1, 2]);
        assert_eq!(right, &[3, 4, 5]);

        left[0] = 10;
        right[0] = 30;

        assert_eq!(data, vec![10, 2, 30, 4, 5]);
    }

    #[test]
    fn test_raw_pointer_creation() {
        let num = 42;
        let ptr = &num as *const i32;

        unsafe {
            assert_eq!(*ptr, 42);
        }
    }

    #[test]
    fn test_union_access() {
        let u = MyUnion { int_value: 42 };

        unsafe {
            assert_eq!(u.int_value, 42);
        }
    }

    #[test]
    fn test_unsafe_cell() {
        let cell = MyCell::new(10);
        assert_eq!(cell.get(), 10);

        cell.set(20);
        assert_eq!(cell.get(), 20);
    }

    #[test]
    fn test_ffi_abs() {
        unsafe {
            assert_eq!(abs(-42), 42);
            assert_eq!(abs(42), 42);
            assert_eq!(abs(0), 0);
        }
    }
}
