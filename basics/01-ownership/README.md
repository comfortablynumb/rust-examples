# Ownership, Borrowing, and Lifetimes

This example demonstrates Rust's core ownership system, which is what makes Rust memory-safe without garbage collection.

## Concepts Covered

### 1. Ownership Rules
- Each value has a single owner
- When the owner goes out of scope, the value is dropped
- Ownership can be transferred (moved)

### 2. Move Semantics
- Non-Copy types (like `String`) are moved when assigned or passed to functions
- After a move, the original variable is no longer valid
- Copy types (like integers) are copied instead of moved

### 3. Borrowing
- References allow you to refer to a value without taking ownership
- Immutable references (`&T`): multiple allowed, read-only access
- Mutable references (`&mut T`): only one allowed, exclusive write access
- Can't have mutable and immutable references simultaneously

### 4. Lifetimes
- Lifetimes ensure references are always valid
- Prevent dangling references
- Explicitly annotated with `'a` syntax when compiler needs help
- Lifetime elision rules reduce annotation burden

### 5. String Types
- `String`: owned, heap-allocated, growable
- `&str`: borrowed string slice, immutable reference
- String literals are `&str` with `'static` lifetime

## Running the Example

```bash
cargo run
```

## Key Takeaways

1. **Ownership prevents data races at compile time** - only one mutable reference OR multiple immutable references
2. **No null pointers** - references are always valid
3. **No use-after-free** - can't use a value after it's been moved or dropped
4. **Zero-cost abstraction** - no runtime overhead for these safety guarantees

## Common Patterns

### Borrowing vs Ownership
```rust
fn process_borrowed(s: &String) { }  // Can use s after this call
fn process_owned(s: String) { }      // s is consumed, can't use after
```

### Returning References
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

### Structs with References
```rust
struct MyStruct<'a> {
    data: &'a str,
}
```
