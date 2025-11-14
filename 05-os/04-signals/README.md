# Unix Signal Handling

Handle operating system signals for graceful shutdown and configuration reloading.

## Common Signals

- **SIGINT** (2): Ctrl+C - Interrupt
- **SIGTERM** (15): Termination request
- **SIGHUP** (1): Hangup / reload config
- **SIGQUIT** (3): Quit and dump core
- **SIGKILL** (9): Force kill (cannot be caught)

## Usage with tokio

```rust
tokio::select! {
    _ = signal::ctrl_c() => {
        println!("Ctrl+C received");
        // Graceful shutdown
    }
    _ = work() => {
        // Normal completion
    }
}
```

## Multiple Signals

```rust
let mut signals = Signals::new(&[SIGINT, SIGTERM, SIGHUP])?;

for sig in signals.forever() {
    match sig {
        SIGINT => { /* handle */ },
        SIGTERM => { /* handle */ },
        SIGHUP => { /* reload config */ },
        _ => {}
    }
}
```

## Running

```bash
cargo run

# In another terminal:
kill -TERM <pid>  # Send SIGTERM
kill -HUP <pid>   # Send SIGHUP
```

## Best Practices

1. **Graceful Shutdown**: Clean up resources
2. **Idempotent Handlers**: Safe to call multiple times
3. **Quick Handlers**: Don't block signal processing
4. **Configuration Reload**: Use SIGHUP
5. **Logging**: Log signal reception

## References

- [signal-hook](https://docs.rs/signal-hook/)
- [Unix Signals](https://en.wikipedia.org/wiki/Signal_(IPC))
