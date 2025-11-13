# Tracing - Structured Logging

Application-level tracing and structured, context-aware logging.

## Features

- **Structured Logging**: Key-value pairs instead of strings
- **Spans**: Track execution context
- **Async Support**: Works with async/await
- **Performance**: Low overhead
- **Composable**: Multiple subscribers

## Log Levels

- **TRACE**: Very detailed, fine-grained
- **DEBUG**: Debug information
- **INFO**: General information
- **WARN**: Warning messages
- **ERROR**: Error messages

## Basic Usage

```rust
use tracing::{info, warn, error};

info!("Application started");
warn!(user_id = 42, "User not found");
error!(err = %e, "Failed to connect");
```

## Structured Fields

```rust
info!(
    user_id = 42,
    username = "alice",
    action = "login",
    "User action"
);
```

## Spans

Track execution context:

```rust
let span = span!(Level::INFO, "request_handler", request_id = 123);
let _enter = span.enter();

info!("Processing request"); // Automatically includes request_id
```

## Instrumentation

Automatic span creation:

```rust
#[instrument]
async fn process_request(user_id: u64) {
    info!("Processing"); // Includes function name and arguments
}

#[instrument(fields(custom_field))]
async fn custom_span(arg: &str) {
    // Custom span configuration
}
```

## Subscribers

### Console Output

```rust
let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::DEBUG)
    .finish();

tracing::subscriber::set_global_default(subscriber)?;
```

### JSON Output

```rust
let subscriber = tracing_subscriber::fmt()
    .json()
    .finish();
```

### Environment Filter

```bash
RUST_LOG=debug cargo run
RUST_LOG=my_app=trace cargo run
RUST_LOG=my_app::module=debug cargo run
```

## Running

```bash
# All logs
cargo run

# Filter by level
RUST_LOG=info cargo run

# Filter by module
RUST_LOG=tracing_example=debug cargo run
```

## tracing vs log

| Feature | tracing | log |
|---------|---------|-----|
| Structured | ✓ | ✗ |
| Spans | ✓ | ✗ |
| Async | ✓ | Limited |
| Context | ✓ | ✗ |
| Performance | Excellent | Good |

## Best Practices

1. **Use Spans**: Group related operations
2. **Add Context**: Include relevant fields
3. **Instrument Functions**: Use `#[instrument]`
4. **Choose Appropriate Levels**: TRACE for debug, INFO for important events
5. **Avoid Secrets**: Don't log passwords/tokens

## Common Patterns

### Request Tracing

```rust
#[instrument(fields(request_id))]
async fn handle_request(req: Request) {
    span::current().record("request_id", &req.id);
    // Process request
}
```

### Error Context

```rust
if let Err(e) = operation() {
    error!(error = %e, "Operation failed");
}
```

### Performance Tracking

```rust
#[instrument]
fn expensive_operation() {
    let start = Instant::now();
    // Work...
    debug!(duration_ms = start.elapsed().as_millis(), "Completed");
}
```

## References

- [tracing Documentation](https://docs.rs/tracing/)
- [Tokio Tracing Guide](https://tokio.rs/tokio/topics/tracing)
