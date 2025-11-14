# Filesystem Watching

Monitor filesystem changes in real-time with the `notify` crate.

## Event Types

- **Create**: File/directory created
- **Modify**: Content or metadata changed
- **Remove**: File/directory deleted
- **Rename**: File/directory renamed
- **Access**: File accessed (if available)

## Usage

```rust
let (tx, rx) = channel();

let mut watcher = RecommendedWatcher::new(
    move |res| tx.send(res).unwrap(),
    Config::default(),
)?;

watcher.watch(path, RecursiveMode::Recursive)?;

for event in rx {
    println!("Event: {:?}", event);
}
```

## Running

```bash
cargo run
```

## Use Cases

- Hot reloading in development
- Configuration file monitoring
- Log file processing
- Build systems
- File synchronization
- Backup automation

## References

- [notify Documentation](https://docs.rs/notify/)
