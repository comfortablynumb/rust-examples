# Pattern Matching with match

This example demonstrates Rust's powerful pattern matching capabilities using the `match` expression.

## Concepts Covered

### 1. Basic Match
- Matching literal values
- Exhaustiveness requirement
- Using `_` as catch-all pattern
- Match expressions return values

### 2. Enum Matching
- Matching enum variants
- Extracting data from enums
- Multiple variant patterns

### 3. Destructuring
- Destructuring tuples
- Destructuring structs
- Nested destructuring
- Ignoring values with `_`

### 4. Match Guards
- Adding conditional logic with `if`
- Combining patterns with guards
- Multiple conditions

### 5. Option Matching
- Handling `Some` and `None`
- Nested Options
- Transforming Option values

### 6. Result Matching
- Handling `Ok` and `Err`
- Custom error types
- Pattern matching for error handling

### 7. Advanced Patterns
- Multiple patterns with `|`
- Range patterns (`..=`, `..`)
- Character ranges

### 8. if let and while let
- Syntactic sugar for simple matches
- `if let` for single pattern matching
- `while let` for conditional loops

## Running the Example

```bash
cargo run
```

## Key Takeaways

1. **Match is exhaustive** - you must handle all possible cases or use `_`
2. **Match returns values** - can be used in assignments and expressions
3. **Destructuring is powerful** - extract values from complex types easily
4. **Guards add flexibility** - combine patterns with conditional logic
5. **if let simplifies code** - use when only one pattern matters

## Common Patterns

### Basic matching
```rust
match value {
    1 => println!("One"),
    2 | 3 => println!("Two or three"),
    4..=10 => println!("Four through ten"),
    _ => println!("Something else"),
}
```

### Option handling
```rust
match optional {
    Some(value) => process(value),
    None => default_action(),
}
```

### if let shorthand
```rust
if let Some(value) = optional {
    // Use value
}
```

### Match with guards
```rust
match number {
    n if n < 0 => println!("Negative"),
    n if n > 0 => println!("Positive"),
    _ => println!("Zero"),
}
```
