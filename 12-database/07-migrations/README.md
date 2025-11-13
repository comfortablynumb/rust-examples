# Database Migrations

Version control for database schema changes.

## Why Migrations?

- **Version Control**: Track schema changes over time
- **Reproducibility**: Same schema across environments
- **Collaboration**: Team members stay in sync
- **Rollback**: Undo problematic changes
- **Documentation**: Clear history of changes

## Migration Files

### Naming Convention

```
migrations/
├── 001_create_users.sql
├── 002_create_posts.sql
└── 003_add_user_fields.sql
```

### Migration Structure

```sql
-- Up migration (apply changes)
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL
);

-- Optionally, down migration (in separate file)
DROP TABLE users;
```

## Using sqlx-cli

### Install

```bash
cargo install sqlx-cli
```

### Commands

```bash
# Create migration
sqlx migrate add create_users

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert

# Check migration status
sqlx migrate info
```

## Programmatic Migrations

### Embed Migrations in Binary

```rust
sqlx::migrate!("./migrations")
    .run(&pool)
    .await?;
```

### Build-time Checking

Add to `.cargo/config.toml`:

```toml
[env]
DATABASE_URL = "sqlite://./dev.db"
```

## Migration Best Practices

### 1. Never Modify Existing Migrations

```
✗ Bad: Edit 001_create_users.sql
✓ Good: Create 004_modify_users.sql
```

### 2. Make Migrations Reversible

```sql
-- Up
ALTER TABLE users ADD COLUMN age INTEGER;

-- Down (in separate file)
ALTER TABLE users DROP COLUMN age;
```

### 3. Test Migrations

```bash
# Test up
sqlx migrate run

# Test down
sqlx migrate revert

# Verify
sqlx migrate run
```

### 4. Use Transactions

```sql
BEGIN;

CREATE TABLE users (...);
CREATE INDEX idx_users_email ON users(email);

COMMIT;
```

### 5. Handle Data

```sql
-- Add column with default
ALTER TABLE users
ADD COLUMN status TEXT NOT NULL DEFAULT 'active';

-- Then update existing rows if needed
UPDATE users SET status = 'active' WHERE status IS NULL;
```

## Common Migration Patterns

### Adding a Table

```sql
CREATE TABLE posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### Adding a Column

```sql
ALTER TABLE users ADD COLUMN bio TEXT;
```

### Adding an Index

```sql
CREATE INDEX idx_users_email ON users(email);
```

### Adding a Foreign Key

```sql
CREATE TABLE posts (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

### Renaming (SQLite requires workaround)

```sql
-- Create new table with correct name
CREATE TABLE users_new (...);

-- Copy data
INSERT INTO users_new SELECT * FROM users;

-- Drop old table
DROP TABLE users;

-- Rename
ALTER TABLE users_new RENAME TO users;
```

## CI/CD Integration

```yaml
# GitHub Actions example
- name: Run migrations
  run: |
    sqlx database create
    sqlx migrate run
  env:
    DATABASE_URL: ${{ secrets.DATABASE_URL }}
```

## Running

```bash
cargo run
```

## Migration Tracking

SQLx creates a `_sqlx_migrations` table:

```sql
SELECT * FROM _sqlx_migrations;
```

Stores:
- Version number
- Description
- Checksum
- Execution time

## References

- [SQLx Migrations](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#migrations)
- [Database Migration Best Practices](https://www.prisma.io/dataguide/types/relational/migration-strategies)
