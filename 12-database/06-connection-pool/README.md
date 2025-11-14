# Database Connection Pooling

Efficient management of database connections for concurrent access.

## Why Connection Pooling?

**Without pooling:**
- Create new connection for each query
- High overhead (TCP handshake, authentication)
- Slow performance
- Resource exhaustion

**With pooling:**
- Reuse existing connections
- Fast query execution
- Controlled resource usage
- Better concurrency

## Configuration

```rust
let pool = SqlitePoolOptions::new()
    .max_connections(5)                     // Max concurrent connections
    .min_connections(2)                     // Keep 2 connections ready
    .acquire_timeout(Duration::from_secs(3)) // Wait max 3s for connection
    .idle_timeout(Duration::from_secs(600))  // Close idle after 10 min
    .max_lifetime(Duration::from_secs(1800)) // Max connection age: 30 min
    .connect(&database_url)
    .await?;
```

## Usage

### Basic Query

```rust
// Pool is cloneable and cheap to clone
let pool = pool.clone();

// Automatically acquires and releases connection
sqlx::query("SELECT * FROM users")
    .fetch_all(&pool)
    .await?;
```

### Explicit Connection

```rust
// Acquire connection from pool
let mut conn = pool.acquire().await?;

// Use connection
sqlx::query("INSERT INTO users ...")
    .execute(&mut *conn)
    .await?;

// Connection returned to pool when dropped
```

### Transactions

```rust
let mut tx = pool.begin().await?;

sqlx::query("INSERT ...")
    .execute(&mut *tx)
    .await?;

tx.commit().await?;
```

## Parallel Execution

```rust
let tasks: Vec<_> = (0..100)
    .map(|i| {
        let pool = pool.clone();
        tokio::spawn(async move {
            query_data(&pool, i).await
        })
    })
    .collect();

join_all(tasks).await;
```

## Pool Monitoring

```rust
// Current pool size
let size = pool.size();

// Number of idle connections
let idle = pool.num_idle();

// Check if closed
let closed = pool.is_closed();
```

## Best Practices

1. **Size Appropriately**
   - Too small: Contention and timeouts
   - Too large: Memory waste
   - Rule of thumb: CPU cores * 2-4

2. **Set Timeouts**
   - Prevent deadlocks
   - Fail fast on issues

3. **Handle Errors**
   - Connection acquisition can fail
   - Always check timeout errors

4. **Close Gracefully**
   ```rust
   pool.close().await;
   ```

5. **Share Pool**
   - Create pool once
   - Clone and share across app

## Common Patterns

### Web Server

```rust
#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

async fn handler(state: State<AppState>) -> Result<String> {
    let users = get_users(&state.pool).await?;
    Ok(format!("{} users", users.len()))
}
```

### Worker Pool

```rust
for _ in 0..worker_count {
    let pool = pool.clone();
    tokio::spawn(async move {
        loop {
            process_job(&pool).await;
        }
    });
}
```

## Running

```bash
cargo run
```

## Comparison

| Without Pool | With Pool |
|-------------|-----------|
| New connection per query | Reuse connections |
| 100-500ms overhead | <1ms overhead |
| Limited concurrency | High concurrency |
| Resource exhaustion | Controlled resources |

## References

- [SQLx Pool](https://docs.rs/sqlx/latest/sqlx/pool/index.html)
- [Connection Pooling Best Practices](https://github.com/brettwooldridge/HikariCP/wiki/About-Pool-Sizing)
