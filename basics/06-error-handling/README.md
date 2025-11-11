# Error Handling

This example demonstrates Rust's approach to error handling using Result, Option, and custom error types.

## Concepts Covered

### 1. Panic - Unrecoverable Errors
- `panic!()` macro
- When to panic vs return errors
- Safe alternatives to panicking

### 2. Result<T, E>
- `Ok(T)` for success
- `Err(E)` for errors
- Methods: `unwrap()`, `expect()`, `unwrap_or()`, `unwrap_or_else()`
- Pattern matching on Result

### 3. The ? Operator
- Early return on errors
- Automatic error conversion
- Cleaner error propagation
- Can only be used in functions returning Result or Option

### 4. Custom Error Types
- Defining custom error enums
- Implementing `Display` and `Error` traits
- Descriptive error messages

### 5. Error Propagation
- Returning errors to callers
- Using ? for concise propagation
- Chaining operations

### 6. Multiple Error Types
- Wrapping different error types
- Implementing `From` trait for conversions
- Unified error handling

### 7. Option<T>
- `Some(T)` for values
- `None` for absence
- Methods: `map()`, `and_then()`, `or()`, `unwrap_or()`
- Avoiding null pointer errors

### 8. Option and Result Together
- Converting between Option and Result
- `ok_or()` - Option to Result
- `ok()` - Result to Option
- Combining operations

## Running the Example

```bash
cargo run
```

## Key Takeaways

1. **No exceptions** - all errors are explicit in return types
2. **? operator** - clean error propagation
3. **Type safety** - compiler ensures errors are handled
4. **Option eliminates null** - explicit optional values
5. **Custom errors** - domain-specific error types
6. **Composable** - easy to chain operations

## Common Patterns

### Basic error handling
```rust
match some_operation() {
    Ok(value) => process(value),
    Err(e) => handle_error(e),
}
```

### Using ?
```rust
fn process() -> Result<T, E> {
    let value = fallible_operation()?;
    Ok(transform(value))
}
```

### Custom error type
```rust
#[derive(Debug)]
enum MyError {
    Io(io::Error),
    Parse(ParseIntError),
    Custom(String),
}

impl From<io::Error> for MyError {
    fn from(error: io::Error) -> Self {
        MyError::Io(error)
    }
}
```

### Chaining operations
```rust
let result = some_option
    .ok_or("Missing value")?
    .parse::<i32>()?
    * 2;
```

## Best Practices

1. **Return Result for recoverable errors** - don't panic
2. **Use custom error types** - more descriptive than String
3. **Implement From for conversions** - enables ? operator
4. **Use Option for optional values** - avoid null
5. **Add context with expect()** - better than unwrap
6. **Propagate errors up** - let callers decide handling

## When to Panic

- Prototype/example code
- Tests
- Unreachable states (with proof)
- Programming errors (contract violations)
- When recovery is impossible

Otherwise, prefer returning `Result`.
