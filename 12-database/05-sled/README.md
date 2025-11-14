# Sled Embedded Database

High-performance embedded database written in pure Rust.

## Features

- **Embedded**: No separate server process
- **ACID**: Atomicity, Consistency, Isolation, Durability
- **Zero-copy reads**: Efficient memory usage
- **Lock-free**: High concurrency
- **Crash-safe**: Recovers from crashes
- **Simple API**: Easy to use

## Use Cases

- Embedded applications
- Local data storage
- Configuration management
- Caching layer
- Time-series data
- Log storage

## Key Operations

### Basic CRUD

```rust
// Insert
db.insert("key", "value")?;

// Get
if let Some(value) = db.get("key")? {
    println!("{}", String::from_utf8(value.to_vec())?);
}

// Update
db.insert("key", "new_value")?;

// Delete
db.remove("key")?;
```

### Atomic Operations

```rust
// Update and fetch
db.update_and_fetch("counter", |old| {
    let num = decode(old);
    Some(encode(num + 1))
})?;

// Compare and swap
db.compare_and_swap("key", Some(old), Some(new))?;
```

### Iteration

```rust
// All keys
for result in db.iter() {
    let (key, value) = result?;
}

// Prefix scan
for result in db.scan_prefix("user:") {
    let (key, value) = result?;
}

// Range
for result in db.range("a".."z") {
    let (key, value) = result?;
}
```

### Transactions

```rust
db.transaction(|tx_db| {
    tx_db.insert(b"key1", b"value1")?;
    tx_db.insert(b"key2", b"value2")?;
    Ok(())
})?;
```

## Serialization

Use `bincode` or `serde_json` for structured data:

```rust
use serde::{Serialize, Deserialize};
use bincode;

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
}

// Store
let user = User { name: "Alice".into(), age: 30 };
db.insert("user:1", bincode::serialize(&user)?)?;

// Retrieve
let data = db.get("user:1")?.unwrap();
let user: User = bincode::deserialize(&data)?;
```

## Running

```bash
cargo run
```

## Sled vs Other Databases

| Feature | Sled | SQLite | RocksDB |
|---------|------|--------|---------|
| Language | Rust | C | C++ |
| API | Simple | SQL | Complex |
| Transactions | Yes | Yes | Limited |
| Embedded | Yes | Yes | Yes |
| Schema | No | Yes | No |

## References

- [Sled Documentation](https://docs.rs/sled/)
- [Sled GitHub](https://github.com/spacejam/sled)
