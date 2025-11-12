# Enums and Pattern Matching

This example demonstrates Rust's powerful enum types, which can hold data and represent algebraic data types.

## Concepts Covered

### 1. Basic Enums
- Simple enum variants
- Pattern matching on enums
- Enum values as function parameters

### 2. Enums with Data
- Variants holding different types of data
- Tuple-like variants
- Struct-like variants
- Mixed variant types in single enum

### 3. Methods on Enums
- `impl` blocks for enums
- Methods that match on `self`
- State transitions with enums

### 4. Option<T>
- Rust's replacement for null
- `Some(T)` and `None` variants
- Option methods: `is_some()`, `is_none()`, `unwrap_or()`, `map()`, etc.
- Chaining operations

### 5. Result<T, E>
- Error handling with `Ok(T)` and `Err(E)`
- Pattern matching for error handling
- Result methods: `unwrap_or()`, `map()`, `unwrap_or_else()`, etc.

### 6. Complex Variants
- Mixing unit, tuple, and struct variants
- Real-world examples (web events)
- Pattern matching complex structures

### 7. Enums in Data Structures
- Using enums in vectors and structs
- State machines with enums
- Counting and categorizing enum variants

## Running the Example

```bash
cargo run
```

## Key Takeaways

1. **Enums are powerful** - can hold different types of data per variant
2. **Type safety** - compiler ensures all cases are handled
3. **Option eliminates null** - no null pointer exceptions
4. **Result for errors** - explicit error handling
5. **Pattern matching required** - can't forget edge cases

## Common Patterns

### Option handling
```rust
let value = some_option
    .map(|x| x * 2)
    .filter(|x| x > &10)
    .unwrap_or(0);
```

### Result handling
```rust
match some_result {
    Ok(value) => process(value),
    Err(e) => handle_error(e),
}
```

### Enum with methods
```rust
enum State {
    Active,
    Inactive,
}

impl State {
    fn is_active(&self) -> bool {
        matches!(self, State::Active)
    }
}
```

### State machine
```rust
enum Connection {
    Disconnected,
    Connecting,
    Connected { address: String },
}

impl Connection {
    fn connect(self, addr: String) -> Self {
        match self {
            Connection::Disconnected => Connection::Connecting,
            other => other,
        }
    }
}
```

## Advantages Over Other Languages

1. **Data in variants** - unlike C enums, can store data
2. **Type safety** - each variant is a distinct type
3. **No null** - Option<T> is explicit and safe
4. **Exhaustive matching** - compiler ensures all cases handled
5. **Zero-cost abstraction** - compiled to efficient code
