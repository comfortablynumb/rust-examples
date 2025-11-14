# SQLx with PostgreSQL

Async PostgreSQL database access with compile-time checked queries using SQLx.

## Concepts

- Async database operations
- Connection pooling
- Compile-time query verification
- Transactions
- Type-safe queries

## Setup

### Install PostgreSQL

```bash
# macOS
brew install postgresql

# Ubuntu/Debian
sudo apt-get install postgresql

# Start service
pg_ctl -D /usr/local/var/postgres start
```

### Create Database

```bash
createdb testdb
```

### Set Environment Variable

```bash
export DATABASE_URL="postgresql://user:password@localhost/testdb"
```

## Running

```bash
cargo run
```

## Key Features

### Connection Pool

```rust
let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await?;
```

### Compile-Time Checked Queries

```rust
// With sqlx-cli installed, queries are checked at compile time
sqlx::query!("SELECT * FROM users WHERE id = $1", user_id)
    .fetch_one(&pool)
    .await?;
```

### Type Mapping

```rust
#[derive(FromRow)]
struct User {
    id: i32,
    username: String,
    email: String,
}

let users = sqlx::query_as::<_, User>("SELECT * FROM users")
    .fetch_all(&pool)
    .await?;
```

### Transactions

```rust
let mut tx = pool.begin().await?;

sqlx::query("INSERT INTO users ...")
    .execute(&mut *tx)
    .await?;

tx.commit().await?;
```

## References

- [SQLx Documentation](https://docs.rs/sqlx/)
- [PostgreSQL](https://www.postgresql.org/)
