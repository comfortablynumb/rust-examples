# Tokio Advanced Features

Advanced patterns and features of the Tokio async runtime.

## Features Covered

- Timeouts
- Select macro for racing futures
- Channels (mpsc, oneshot)
- Semaphores for rate limiting
- Task spawning
- Intervals

## Timeout

```rust
use tokio::time::{timeout, Duration};

match timeout(Duration::from_secs(1), slow_operation()).await {
    Ok(result) => println!("Completed: {:?}", result),
    Err(_) => println!("Timed out"),
}
```

## Select Macro

Race multiple futures:

```rust
select! {
    result = operation1() => {
        println!("Operation 1 finished first");
    }
    result = operation2() => {
        println!("Operation 2 finished first");
    }
}
```

## Channels

### MPSC (Multiple Producer, Single Consumer)

```rust
let (tx, mut rx) = mpsc::channel(32);

// Send
tx.send(value).await?;

// Receive
while let Some(msg) = rx.recv().await {
    println!("{}", msg);
}
```

### Oneshot

```rust
let (tx, rx) = oneshot::channel();

// Send (consumes sender)
tx.send(value)?;

// Receive
let result = rx.await?;
```

## Semaphore

Limit concurrent operations:

```rust
let semaphore = Arc::new(Semaphore::new(5)); // Max 5 concurrent

let _permit = semaphore.acquire().await?;
// Do work...
// Permit automatically released when dropped
```

## Task Spawning

```rust
let handle = tokio::spawn(async {
    // Async work
    42
});

let result = handle.await?;
```

## Intervals

```rust
let mut interval = interval(Duration::from_secs(1));

loop {
    interval.tick().await;
    println!("Tick!");
}
```

## Running

```bash
cargo run
```

## Best Practices

1. **Use `select!` Wisely**: First completed branch wins
2. **Close Channels**: Drop senders when done
3. **Bound Channels**: Prevent unbounded memory growth
4. **Semaphores for Limiting**: Control concurrent access
5. **Handle Timeouts**: Always have fallback logic

## Common Patterns

### Graceful Shutdown

```rust
let (shutdown_tx, mut shutdown_rx) = oneshot::channel();

select! {
    _ = server.run() => {},
    _ = shutdown_rx => {
        println!("Shutting down...");
    }
}
```

### Rate Limiting

```rust
let rate_limiter = Arc::new(Semaphore::new(100)); // 100 req/s

for request in requests {
    let permit = rate_limiter.clone().acquire_owned().await?;
    tokio::spawn(async move {
        process(request).await;
        drop(permit);
    });
}
```

## References

- [Tokio Documentation](https://docs.rs/tokio/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
