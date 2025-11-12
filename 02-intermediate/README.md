# Intermediate Rust Examples

Intermediate Rust concepts that bridge the gap between basics and advanced topics. These examples cover concurrency, async programming, advanced traits, testing, and practical Cargo usage.

## Prerequisites

Before working through these examples, you should be comfortable with:
- Ownership, borrowing, and lifetimes
- Traits and basic generics
- Error handling
- Basic Rust syntax and patterns

## Examples

### [01. Concurrency](01-concurrency/)
Thread-based concurrency patterns:
- Creating and managing threads
- Message passing with channels (mpsc)
- Shared state with Arc<Mutex<T>>
- Thread pools and parallel processing
- Synchronization primitives
- Deadlock prevention

### [02. Async/Await](02-async/)
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

### [03. Traits and Generics](03-traits-generics/)
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

### [04. Testing](04-testing/)
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

### [05. Cargo and Workspaces](05-cargo/)
Practical Cargo usage and project management:
- Workspace structure and organization
- Shared dependencies across crates
- Feature flags for conditional compilation
- Build scripts (build.rs)
- Custom build profiles
- Path dependencies between crates
- Binary and library crates
- Publishing and versioning
- Cargo commands and workflows

### [06. Standard Library](06-stdlib/)
Collections, strings, time, and memory utilities:
- Common collections (Vec, HashMap, HashSet, BTreeMap)
- String handling (String, &str, formatting)
- Time and duration utilities
- Memory and reference patterns
- File system and path operations
- Common traits and their implementations

## Learning Path

We recommend working through these examples in order:

1. **Concurrency** - Thread-based parallel programming
2. **Async/Await** - Modern asynchronous programming
3. **Traits and Generics** - Advanced type system features
4. **Testing** - Comprehensive testing strategies and tools
5. **Cargo and Workspaces** - Project organization and management
6. **Standard Library** - Collections, strings, and common utilities

After mastering these intermediate concepts, proceed to the **03. Advanced** category for unsafe Rust, smart pointers, macros, and more sophisticated patterns.

## Running Examples

Each example can be run independently:

```bash
cd 02-intermediate/01-concurrency
cargo run
cargo test
```

## Key Concepts

### When to Use Each Pattern

**Concurrency vs Async:**
- Use threads for CPU-bound parallel work
- Use async for I/O-bound concurrent work
- Async has lower overhead for many concurrent operations
- Threads are simpler but more resource-intensive

**Generics vs Trait Objects:**
- Generics: static dispatch, monomorphization, faster but larger binary
- Trait objects: dynamic dispatch, runtime polymorphism, smaller binary

## Common Pitfalls

### Concurrency
- **Data races**: Always use Arc<Mutex<T>> for shared mutation
- **Deadlocks**: Avoid nested locks or use consistent ordering
- **Panics**: Panics in threads don't crash the program

### Async
- **Blocking**: Never block in async code (use spawn_blocking)
- **CPU work**: Async is for I/O, not CPU-bound tasks
- **Runtime**: Tokio vs async-std vs others

## Best Practices

1. **Prefer concurrency patterns** - Choose threads vs async based on workload
2. **Test async code thoroughly** - Async bugs can be subtle
3. **Test concurrent code** - Use tools like loom for testing
4. **Profile before optimizing** - Measure, don't guess
5. **Document complexity** - Explain non-obvious patterns
6. **Use type system** - Encode invariants in types

## Testing

Each example includes comprehensive tests:

```bash
# Test all intermediate examples
for dir in 02-intermediate/*/; do
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
- Concurrent and parallel programming
- Asynchronous I/O and task management
- Generic programming and trait system mastery
- Comprehensive testing strategies
- Practical Cargo usage and project organization

Master these concepts to write production-grade Rust applications that are fast, safe, and well-tested. For more advanced topics like unsafe Rust, smart pointers, macros, and advanced type system features, proceed to the **Advanced** category.
