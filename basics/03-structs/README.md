# Structs and Methods

This example demonstrates Rust's struct types and how to define methods and associated functions.

## Concepts Covered

### 1. Classic Structs
- Named fields
- Creating instances
- Accessing and modifying fields
- Mutable vs immutable structs

### 2. Tuple Structs
- Structs without named fields
- Type safety with identical field types
- Destructuring tuple structs

### 3. Unit Structs
- Structs without any fields
- Used as markers or for trait implementations
- Zero-size types

### 4. Methods
- Functions defined with `impl` blocks
- `&self` - immutable borrow
- `&mut self` - mutable borrow
- `self` - takes ownership
- Methods with additional parameters

### 5. Associated Functions
- Functions without `self` parameter
- Called with `::` syntax (like `String::new()`)
- Often used as constructors
- `Self` type alias

### 6. Multiple impl Blocks
- Can split methods across multiple blocks
- Useful for organization
- All in same namespace

### 7. Struct Update Syntax
- Creating instances from existing ones
- Using `..` to copy remaining fields
- Moves non-Copy fields

### 8. Ownership with Structs
- Structs own their data by default
- Methods borrow or take ownership
- Returning references to internal data

## Running the Example

```bash
cargo run
```

## Key Takeaways

1. **Structs own their data** - fields are owned by default
2. **Methods use impl blocks** - separate data from behavior
3. **&self vs &mut self vs self** - different ownership patterns
4. **Associated functions** - constructors and utilities using `::`
5. **Derive useful traits** - `#[derive(Debug, Clone, etc.)]` for common functionality

## Common Patterns

### Basic struct with methods
```rust
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    fn distance(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
```

### Builder pattern
```rust
impl User {
    fn new(username: String) -> Self {
        User {
            username,
            email: String::new(),
            active: false,
        }
    }

    fn email(mut self, email: String) -> Self {
        self.email = email;
        self
    }

    fn activate(mut self) -> Self {
        self.active = true;
        self
    }
}

let user = User::new("alice".to_string())
    .email("alice@example.com".to_string())
    .activate();
```

### Struct update syntax
```rust
let user2 = User {
    username: String::from("bob"),
    ..user1
};
```
