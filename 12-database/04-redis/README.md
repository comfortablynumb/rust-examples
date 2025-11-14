# Redis

In-memory data structure store used as database, cache, and message broker.

## Setup

```bash
# Install Redis
brew install redis  # macOS
sudo apt-get install redis  # Ubuntu

# Start Redis
redis-server
```

## Features

- **Strings**: Simple key-value
- **Lists**: Ordered collections
- **Sets**: Unique collections
- **Hashes**: Field-value pairs
- **Sorted Sets**: Ordered by score
- **Pub/Sub**: Messaging
- **Transactions**: Atomic operations
- **Expiration**: TTL support

## Use Cases

- Caching
- Session storage
- Real-time analytics
- Leaderboards
- Rate limiting
- Message queues

## Running

```bash
cargo run
```

## References

- [Redis](https://redis.io/)
- [redis-rs](https://docs.rs/redis/)
