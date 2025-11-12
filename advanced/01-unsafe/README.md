# Unsafe Rust

This example demonstrates unsafe Rust features and when they're necessary. Unsafe code opts out of Rust's safety guarantees and requires manual verification of correctness.

## Concepts Covered

### Unsafe Superpowers
The five things you can do in unsafe Rust:
1. Dereference raw pointers
2. Call unsafe functions
3. Access or modify mutable static variables
4. Implement unsafe traits
5. Access fields of unions

### Raw Pointers
- Creating raw pointers (`*const T`, `*mut T`)
- Dereferencing raw pointers
- Pointer arithmetic
- Null pointers
- Converting between pointers and integers

### Unsafe Functions
- Defining unsafe functions
- Calling unsafe functions
- Safety contracts and documentation
- Creating safe abstractions

### FFI (Foreign Function Interface)
- Calling C functions
- Declaring external functions
- Working with C types
- String handling across FFI boundary

### Advanced Features
- Mutable static variables
- Union types
- Memory transmutation
- Unsafe traits
- UnsafeCell (interior mutability primitive)

## Running the Example

```bash
cargo run
```

Run tests:
```bash
cargo test
```

## Key Takeaways

1. **Unsafe doesn't disable safety checks** - Borrow checker still works
2. **Minimize unsafe code** - Encapsulate in safe abstractions
3. **Document safety requirements** - Use `# Safety` sections
4. **Careful code review** - Unsafe code needs extra scrutiny
5. **Undefined behavior** - Invalid unsafe usage = UB (very bad!)

## The Five Unsafe Superpowers

### 1. Dereference Raw Pointers

```rust
let mut num = 42;
let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

unsafe {
    println!("r1: {}", *r1);  // Read
    *r2 = 100;                // Write
}
```

### 2. Call Unsafe Functions

```rust
unsafe fn dangerous() {
    // unsafe code here
}

// Must call in unsafe block
unsafe {
    dangerous();
}
```

### 3. Access Mutable Static Variables

```rust
static mut COUNTER: u32 = 0;

unsafe {
    COUNTER += 1;
    println!("Counter: {}", COUNTER);
}
```

### 4. Implement Unsafe Traits

```rust
unsafe trait UnsafeTrait {
    fn method(&self);
}

unsafe impl UnsafeTrait for MyType {
    fn method(&self) {
        // implementation
    }
}
```

### 5. Access Union Fields

```rust
union MyUnion {
    i: i32,
    f: f32,
}

let u = MyUnion { i: 42 };

unsafe {
    println!("{}", u.i);
}
```

## Common Patterns

### Safe abstraction over unsafe code

```rust
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
```

### FFI to C functions

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

let result = unsafe { abs(-42) };
```

### Pointer arithmetic

```rust
let arr = [1, 2, 3, 4, 5];
let ptr = arr.as_ptr();

unsafe {
    for i in 0..5 {
        println!("{}", *ptr.add(i));
    }
}
```

### UnsafeCell for interior mutability

```rust
use std::cell::UnsafeCell;

struct MyCell {
    value: UnsafeCell<i32>,
}

impl MyCell {
    fn get(&self) -> i32 {
        unsafe { *self.value.get() }
    }

    fn set(&self, value: i32) {
        unsafe { *self.value.get() = value }
    }
}
```

## Safety Documentation

Always document safety requirements:

```rust
/// # Safety
///
/// The caller must ensure that:
/// - `ptr` is valid and properly aligned
/// - `ptr` points to at least `len` initialized elements
/// - The memory is not accessed by other code during this call
unsafe fn process_array(ptr: *const i32, len: usize) {
    // implementation
}
```

## When to Use Unsafe

**Valid reasons:**
- Interfacing with C/C++ code (FFI)
- Implementing low-level data structures
- Performance-critical code (with benchmarks!)
- Platform-specific operations
- Building safe abstractions (like Vec, Box)

**Invalid reasons:**
- "It's faster" (without proof)
- Avoiding the borrow checker
- Convenience
- "I know what I'm doing" (famous last words)

## Unsafe Doesn't Mean...

**Unsafe does NOT:**
- Disable the borrow checker
- Allow data races (in safe code)
- Make all code in the block unsafe
- Mean you should avoid it entirely

**It DOES mean:**
- You're responsible for upholding invariants
- The compiler can't verify safety
- Extra care and review required
- Document safety contracts

## Common Unsafe Pitfalls

### 1. Dereferencing dangling pointers
```rust
let r;
{
    let x = 42;
    r = &x as *const i32;
}
// x is dropped, r is dangling!
unsafe {
    println!("{}", *r);  // UB!
}
```

### 2. Creating multiple mutable references
```rust
let mut num = 42;
let r1 = &mut num as *mut i32;
let r2 = &mut num as *mut i32;

unsafe {
    *r1 = 1;
    *r2 = 2;  // UB! Two mutable aliases
}
```

### 3. Reading uninitialized memory
```rust
use std::mem::MaybeUninit;

let x: i32;
unsafe {
    println!("{}", x);  // UB! Uninitialized
}
```

### 4. Invalid transmute
```rust
unsafe {
    // Different sizes - UB!
    let x: i32 = std::mem::transmute(5u64);
}
```

### 5. Violating union invariants
```rust
union U {
    i: i32,
    f: f32,
}

let u = U { i: 42 };
unsafe {
    // Reading wrong field - UB!
    let _ = u.f;
}
```

## Testing Unsafe Code

```rust
#[test]
fn test_unsafe_function() {
    let mut data = vec![1, 2, 3];
    let (left, right) = split_at_mut(&mut data, 1);

    assert_eq!(left, &[1]);
    assert_eq!(right, &[2, 3]);
}
```

### Use Miri for UB detection
```bash
cargo +nightly miri test
```

## Best Practices

1. **Minimize unsafe code** - Keep it small and isolated
2. **Create safe wrappers** - Expose safe APIs
3. **Document thoroughly** - Explain all safety requirements
4. **Review carefully** - Extra scrutiny for unsafe code
5. **Test extensively** - Unit tests and Miri
6. **Use tools** - Valgrind, AddressSanitizer, Miri
7. **Prefer safe alternatives** - Use unsafe only when necessary
8. **Keep invariants local** - Don't leak unsafe assumptions

## Unsafe in the Wild

### Standard library examples
```rust
// Vec::set_len - caller must initialize memory
vec.set_len(new_len);

// slice::from_raw_parts - caller ensures validity
let slice = slice::from_raw_parts(ptr, len);

// String::from_utf8_unchecked - caller ensures valid UTF-8
let s = String::from_utf8_unchecked(bytes);
```

### Common unsafe patterns
- `Vec` and `String` internals
- `Box::from_raw` / `Box::into_raw`
- `Rc::from_raw` / `Arc::from_raw`
- `MaybeUninit` for uninitialized memory
- `ManuallyDrop` for custom drop behavior

## Memory Safety Rules

Even in unsafe code, you must maintain:

1. **No dangling pointers** - All pointers must be valid
2. **No data races** - No concurrent mutation
3. **No invalid values** - Types must have valid bit patterns
4. **No use after free** - Don't access freed memory
5. **Proper alignment** - Respect alignment requirements

## Sanitizers and Tools

### AddressSanitizer
```bash
RUSTFLAGS="-Z sanitizer=address" cargo +nightly run
```

### ThreadSanitizer
```bash
RUSTFLAGS="-Z sanitizer=thread" cargo +nightly run
```

### Miri (UB detector)
```bash
cargo +nightly miri run
cargo +nightly miri test
```

### Valgrind
```bash
cargo build
valgrind ./target/debug/unsafe-rust
```

## Resources

- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) - The Dark Arts of Unsafe Rust
- [Rust Reference - Unsafe](https://doc.rust-lang.org/reference/unsafety.html)
- [std::ptr documentation](https://doc.rust-lang.org/std/ptr/)
- [Learning Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)
- [Miri - UB detector](https://github.com/rust-lang/miri)

## Remember

> "With great power comes great responsibility"

Unsafe Rust gives you the power to:
- Do anything (including shoot yourself in the foot)
- Interface with other languages
- Build high-performance abstractions
- Implement the impossible (safely!)

But you must ensure:
- Memory safety
- Thread safety
- Type safety
- All other invariants

When in doubt, prefer safe Rust!
