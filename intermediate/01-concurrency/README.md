# Concurrency

This example demonstrates Rust's fearless concurrency - safe concurrent programming prevented by the type system.

## Concepts Covered

### 1. Creating Threads
- `thread::spawn()` to create threads
- `join()` to wait for completion
- Thread handles
- Multiple threads

### 2. Moving Data
- `move` keyword for ownership transfer
- Returning data from threads
- Thread results with `join()`

### 3. Message Passing
- Channels with `mpsc::channel()`
- Sending and receiving messages
- `send()` and `recv()`
- Ownership transfer through channels

### 4. Multiple Producers
- Cloning sender with `clone()`
- Multiple threads sending to one receiver
- Producer-consumer pattern
- Dropping senders to signal completion

### 5. Shared State with Mutex
- `Mutex<T>` for mutual exclusion
- `lock()` to acquire lock
- Automatic unlock when guard dropped
- Interior mutability

### 6. Arc<Mutex<T>>
- `Arc` for thread-safe reference counting
- Sharing mutable state across threads
- Common pattern for concurrent access
- Thread-safe shared data structures

### 7. Thread Pool
- Worker threads waiting for jobs
- Job queue with channel
- Efficient task distribution
- Graceful shutdown

### 8. Parallel Processing
- Map-reduce pattern
- Chunking data for threads
- Barrier synchronization
- Concurrent computation

## Running the Example

```bash
cargo run
```

## Key Takeaways

1. **No data races** - prevented at compile time
2. **Message passing** - "communicate by sharing memory"
3. **Shared state** - safe with Mutex/Arc
4. **Thread safety** - guaranteed by type system
5. **Fearless concurrency** - compiler catches errors
6. **Zero-cost** - no runtime overhead for safety

## Common Patterns

### Basic thread
```rust
let handle = thread::spawn(|| {
    // thread code
});
handle.join().unwrap();
```

### Move data to thread
```rust
let data = vec![1, 2, 3];
thread::spawn(move || {
    println!("{:?}", data);
});
```

### Channel communication
```rust
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send(42).unwrap();
});

let value = rx.recv().unwrap();
```

### Shared mutable state
```rust
let counter = Arc::new(Mutex::new(0));
let counter_clone = Arc::clone(&counter);

thread::spawn(move || {
    *counter_clone.lock().unwrap() += 1;
});
```

### Multiple producers
```rust
let (tx, rx) = mpsc::channel();

for i in 0..10 {
    let tx = tx.clone();
    thread::spawn(move || {
        tx.send(i).unwrap();
    });
}
```

## Synchronization Primitives

| Type | Purpose | Use Case |
|------|---------|----------|
| `Mutex<T>` | Mutual exclusion | Shared mutable data |
| `RwLock<T>` | Read-write lock | Many readers, few writers |
| `Arc<T>` | Atomic reference count | Share across threads |
| `mpsc::channel()` | Message passing | Send data between threads |
| `Barrier` | Synchronization point | Wait for all threads |
| `Condvar` | Condition variable | Wait for condition |

## Message Passing vs Shared State

**Message Passing (Channels):**
- ✓ Clear ownership transfer
- ✓ No shared state
- ✓ Easier to reason about
- ✗ Copying overhead for large data

**Shared State (Arc<Mutex<T>>):**
- ✓ Efficient for large data
- ✓ Direct access
- ✗ Lock contention
- ✗ More complex reasoning

## Thread Safety Types

**Send trait:**
- Type can be transferred across threads
- Most types implement Send
- `Rc<T>` does NOT implement Send

**Sync trait:**
- Type can be referenced from multiple threads
- `&T` is Send if `T` is Sync
- `RefCell<T>` does NOT implement Sync

## Performance Tips

1. **Minimize lock time** - hold locks briefly
2. **Prefer message passing** - when possible
3. **Use RwLock** - for read-heavy workloads
4. **Avoid lock contention** - distribute work
5. **Thread pools** - reuse threads
6. **Batch operations** - reduce synchronization

## Common Pitfalls

**Deadlocks:**
- Acquire locks in consistent order
- Use timeouts with `try_lock()`
- Minimize lock scope

**Starvation:**
- Fair scheduling
- Avoid long critical sections
- Use RwLock for readers

**Performance:**
- Too many threads (overhead)
- Too much synchronization
- False sharing
- Lock contention

## Best Practices

1. **Prefer message passing** - clearer ownership
2. **Keep critical sections small** - hold locks briefly
3. **Use thread pools** - for many short tasks
4. **Avoid blocking** - in critical sections
5. **Test thoroughly** - concurrency bugs are hard
6. **Use higher-level abstractions** - when available

## When to Use Concurrency

**Good use cases:**
- CPU-bound parallel tasks
- I/O-bound operations
- Responsive UIs
- Server request handling

**Avoid for:**
- Simple sequential tasks
- Tight coupling needed
- Excessive communication overhead
- Debugging complexity not worth it
