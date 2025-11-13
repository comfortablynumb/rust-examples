# SQLx with SQLite

Lightweight embedded database with SQLx for local data storage.

## Features

- No separate database server required
- File-based or in-memory database
- ACID compliance
- Zero-configuration
- Cross-platform

## Usage

```rust
// In-memory database
let pool = SqlitePoolOptions::new()
    .connect("sqlite::memory:")
    .await?;

// File-based database
let pool = SqlitePoolOptions::new()
    .connect("sqlite:///path/to/database.db")
    .await?;
```

## Running

```bash
cargo run
```

## References

- [SQLite](https://www.sqlite.org/)
- [SQLx SQLite](https://docs.rs/sqlx/latest/sqlx/sqlite/index.html)
