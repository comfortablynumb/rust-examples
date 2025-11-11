# Smart Pointers

This example demonstrates Rust's smart pointer types that provide additional capabilities beyond references.

## Concepts Covered

### 1. Box<T>
- Heap allocation
- Storing data of unknown size
- Transferring ownership
- Trait objects
- Single ownership

### 2. Recursive Types
- Box enables recursive data structures
- Linked lists
- Trees
- Known size at compile time

### 3. Rc<T>
- Reference counting (single-threaded)
- Multiple ownership
- Shared read-only data
- `Rc::clone()` increments count
- Automatic cleanup when count reaches 0

### 4. RefCell<T>
- Interior mutability pattern
- Runtime borrow checking
- `borrow()` for immutable access
- `borrow_mut()` for mutable access
- Panics if rules violated at runtime

### 5. Rc<RefCell<T>>
- Shared mutable state (single-threaded)
- Multiple owners with mutation
- Common pattern for graphs
- Enables cycles (use with care)

### 6. Arc<T>
- Atomic reference counting (thread-safe)
- Share data across threads
- `Arc<Mutex<T>>` for shared mutable state
- Thread-safe alternative to Rc<T>

### 7. Deref and Drop
- `Deref` trait for `*` operator
- Deref coercion
- `Drop` trait for cleanup
- LIFO drop order
- Custom smart pointers

## Running the Example

```bash
cargo run
```

## Key Takeaways

1. **Box for heap allocation** - single ownership on heap
2. **Rc for shared ownership** - multiple owners, single-threaded
3. **RefCell for interior mutability** - runtime borrow checking
4. **Arc for thread-safe sharing** - multiple owners across threads
5. **Combine patterns** - Rc<RefCell<T>>, Arc<Mutex<T>>
6. **Zero-cost when possible** - compile-time optimizations

## Smart Pointer Comparison

| Type | Ownership | Mutability | Thread-Safe | Borrow Check |
|------|-----------|------------|-------------|--------------|
| `Box<T>` | Single | Compile-time | Yes | Compile-time |
| `Rc<T>` | Multiple | Immutable | No | Compile-time |
| `Arc<T>` | Multiple | Immutable | Yes | Compile-time |
| `RefCell<T>` | Single | Interior | No | Runtime |
| `Mutex<T>` | Single | Interior | Yes | Runtime |

## Common Patterns

### Box for trait objects
```rust
let animals: Vec<Box<dyn Animal>> = vec![
    Box::new(Dog),
    Box::new(Cat),
];
```

### Rc for shared ownership
```rust
let data = Rc::new(value);
let data2 = Rc::clone(&data);
```

### RefCell for interior mutability
```rust
let data = RefCell::new(5);
*data.borrow_mut() += 1;
```

### Rc<RefCell<T>> for shared mutable
```rust
let shared = Rc::new(RefCell::new(vec![]));
let copy = Rc::clone(&shared);
copy.borrow_mut().push(1);
```

### Arc for threads
```rust
let data = Arc::new(value);
thread::spawn(move || {
    println!("{:?}", data);
});
```

### Arc<Mutex<T>> for shared mutable across threads
```rust
let counter = Arc::new(Mutex::new(0));
let counter_clone = Arc::clone(&counter);
thread::spawn(move || {
    *counter_clone.lock().unwrap() += 1;
});
```

## When to Use

**Box<T>:**
- Large data on heap
- Trait objects
- Recursive types
- Transfer ownership cheaply

**Rc<T>:**
- Multiple ownership needed
- Single-threaded
- Read-only sharing
- Graph data structures

**RefCell<T>:**
- Interior mutability needed
- Single ownership
- Runtime borrow checking OK
- Testing/mocking

**Arc<T>:**
- Share across threads
- Multiple ownership
- Read-only data
- Thread-safe needed

**Combinations:**
- `Rc<RefCell<T>>` - shared mutable (single-threaded)
- `Arc<Mutex<T>>` - shared mutable (multi-threaded)
- `Arc<RwLock<T>>` - multiple readers, single writer

## Borrow Rules

**Compile-time (Box, Rc, Arc):**
- Checked at compile time
- Zero runtime cost
- Safer - errors caught early

**Runtime (RefCell, Mutex):**
- Checked at runtime
- Small runtime cost
- Panics if rules violated
- More flexible

## Memory Management

All smart pointers:
- Own their data
- Automatic cleanup (Drop)
- No manual memory management
- No memory leaks (without cycles)

**Reference cycles:**
- Possible with Rc<RefCell<T>>
- Use `Weak<T>` to break cycles
- Rare in practice
