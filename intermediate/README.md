# Intermediate Rust Examples

Advanced Rust concepts for developers who have mastered the basics. These examples cover more complex topics including smart pointers, concurrency, unsafe code, async programming, and advanced trait usage.

## Prerequisites

Before working through these examples, you should be comfortable with:
- Ownership, borrowing, and lifetimes
- Traits and basic generics
- Error handling
- Basic Rust syntax and patterns

## Examples

### [01. Smart Pointers](01-smart-pointers/)
Comprehensive coverage of Rust's smart pointer types:
- **Box<T>** - Heap allocation and recursive types
- **Rc<T>** / **Weak<T>** - Reference counting and cycle breaking
- **Arc<T>** - Thread-safe reference counting
- **RefCell<T>** / **Cell<T>** - Interior mutability
- **Cow<T>** - Clone-on-write optimization
- **Mutex<T>** / **RwLock<T>** - Thread-safe mutation
- Custom smart pointers and performance considerations

### [02. Concurrency](02-concurrency/)
Thread-based concurrency patterns:
- Creating and managing threads
- Message passing with channels (mpsc)
- Shared state with Arc<Mutex<T>>
- Thread pools and parallel processing
- Synchronization primitives
- Deadlock prevention

### [03. Unsafe Rust](03-unsafe/)
Working with unsafe Rust when necessary:
- Raw pointers and dereferencing
- Unsafe functions and blocks
- FFI (Foreign Function Interface)
- Union types
- Memory transmutation
- Unsafe traits
- Safety contracts and documentation

### [04. Async/Await](04-async/)
Asynchronous programming with async/await:
- Async functions and blocks
- The Future trait
- Tokio runtime
- Spawning tasks and task management
- Async channels (mpsc)
- join! and select! macros
- Timeouts and delays
- Async file I/O
- Error handling in async contexts

### [05. Traits and Generics](05-traits-generics/)
Advanced trait and generic patterns:
- Generic functions, structs, and enums
- Trait bounds and where clauses
- Associated types
- Generic traits
- Trait objects and dynamic dispatch
- Static vs dynamic dispatch
- impl Trait syntax
- Trait inheritance and supertraits
- Blanket implementations
- Marker traits (Send, Sync, Copy, Sized)
- Advanced patterns (newtype, type state)

### [06. Testing](06-testing/)
Comprehensive testing strategies and tools:
- Unit tests and test organization
- Integration tests
- Documentation tests
- Property-based testing with proptest
- Mocking with mockall
- Benchmarking with Criterion
- Test fixtures and helpers
- Parameterized tests
- Error testing and panic handling
- Test best practices

## Learning Path

We recommend working through these examples in order:

1. **Smart Pointers** - Understanding memory management patterns
2. **Concurrency** - Thread-based parallel programming
3. **Async/Await** - Modern asynchronous programming
4. **Traits and Generics** - Advanced type system features
5. **Testing** - Comprehensive testing strategies and tools
6. **Unsafe Rust** - When and how to break the safety rules

## Running Examples

Each example can be run independently:

```bash
cd intermediate/01-smart-pointers
cargo run
cargo test
```

## Key Concepts

### When to Use Each Pattern

**Smart Pointers:**
- Use `Box<T>` for heap allocation and trait objects
- Use `Rc<T>` for shared ownership (single-threaded)
- Use `Arc<T>` for shared ownership (multi-threaded)
- Use `RefCell<T>` for interior mutability (single-threaded)
- Use `Mutex<T>` or `RwLock<T>` for interior mutability (multi-threaded)

**Concurrency vs Async:**
- Use threads for CPU-bound parallel work
- Use async for I/O-bound concurrent work
- Async has lower overhead for many concurrent operations
- Threads are simpler but more resource-intensive

**Generics vs Trait Objects:**
- Generics: static dispatch, monomorphization, faster but larger binary
- Trait objects: dynamic dispatch, runtime polymorphism, smaller binary

**When to Use Unsafe:**
- Interfacing with C/C++ (FFI)
- Implementing low-level data structures
- Performance-critical code (with proof)
- Building safe abstractions

## Common Pitfalls

### Smart Pointers
- **Reference cycles**: Use `Weak<T>` to break cycles
- **Deadlocks**: Lock order matters with multiple Mutexes
- **Performance**: Rc/Arc have overhead, use only when needed

### Concurrency
- **Data races**: Always use Arc<Mutex<T>> for shared mutation
- **Deadlocks**: Avoid nested locks or use consistent ordering
- **Panics**: Panics in threads don't crash the program

### Async
- **Blocking**: Never block in async code (use spawn_blocking)
- **CPU work**: Async is for I/O, not CPU-bound tasks
- **Runtime**: Tokio vs async-std vs others

### Unsafe
- **Undefined behavior**: Easy to cause, hard to debug
- **Minimize scope**: Keep unsafe blocks small
- **Document safety**: Always document safety requirements

## Best Practices

1. **Prefer safe Rust** - Use unsafe only when necessary
2. **Start with the simplest option** - Box before Rc, Rc before Arc
3. **Test concurrent code** - Use tools like loom for testing
4. **Profile before optimizing** - Measure, don't guess
5. **Document complexity** - Explain non-obvious patterns
6. **Use type system** - Encode invariants in types

## Testing

Each example includes comprehensive tests:

```bash
# Test all intermediate examples
for dir in intermediate/*/; do
    cd "$dir"
    cargo test
    cd -
done
```

## Further Reading

- [The Rust Book - Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [The Rust Book - Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Async Book](https://rust-lang.github.io/async-book/)
- [The Rustonomicon - Unsafe](https://doc.rust-lang.org/nomicon/)
- [Rust by Example - Generics](https://doc.rust-lang.github.io/rust-by-example/generics.html)

## Real-World Applications

These intermediate concepts are essential for:
- **Web servers** (tokio, async patterns)
- **Game engines** (unsafe for performance, ECS patterns)
- **Database drivers** (async I/O, connection pooling)
- **CLI tools** (parallel processing, channels)
- **Systems programming** (unsafe, FFI, low-level control)

## Performance Considerations

- **Smart pointers** add indirection and overhead
- **Async** has lower per-task overhead than threads
- **Generic code** is monomorphized (fast but large binary)
- **Trait objects** use dynamic dispatch (small binary but slower)
- **Unsafe** can be faster but correctness is critical

## Common Errors and Solutions

### "cannot move out of borrowed content"
- Use `.clone()` or change ownership
- Consider using `Rc<T>` or `Arc<T>`

### "cannot borrow as mutable"
- Use `RefCell<T>` for interior mutability
- Or restructure to avoid shared mutation

### "future cannot be sent between threads safely"
- Ensure captured variables are `Send`
- Or use `LocalSet` for non-Send futures

### "trait bound not satisfied"
- Add required trait bounds
- Check that types implement required traits

## Summary

The intermediate examples build on Rust fundamentals to cover:
- Advanced memory management with smart pointers
- Concurrent and parallel programming
- Asynchronous I/O and task management
- Generic programming and trait system mastery
- Safe abstractions over unsafe code

Master these concepts to write production-grade Rust applications that are fast, safe, and concurrent.
