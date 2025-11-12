# Traits - Shared Behavior

This example demonstrates Rust's trait system, which enables polymorphism and generic programming.

## Concepts Covered

### 1. Basic Traits
- Defining traits with method signatures
- Implementing traits for types
- Calling trait methods

### 2. Default Implementations
- Providing default method implementations
- Overriding default implementations
- Calling other trait methods from defaults

### 3. Traits as Parameters
- `impl Trait` syntax
- Generic type parameters with trait bounds
- Accepting any type that implements a trait

### 4. Trait Bounds
- Restricting generic types
- Single trait bounds
- Using generics with traits

### 5. Multiple Trait Bounds
- Combining traits with `+`
- `where` clauses for complex bounds
- Readability with where clauses

### 6. Associated Types
- Defining associated types in traits
- Differences from generics
- Cleaner syntax for trait implementations

### 7. Trait Objects
- Dynamic dispatch with `dyn Trait`
- `Box<dyn Trait>` and `&dyn Trait`
- Heterogeneous collections
- Performance trade-offs

### 8. Derivable Traits
- `#[derive(...)]` attribute
- Common derivable traits:
  - `Debug` - formatting with `{:?}`
  - `Clone` - explicit copying
  - `Copy` - implicit copying
  - `PartialEq`, `Eq` - equality comparison
  - `PartialOrd`, `Ord` - ordering

### 9. Operator Overloading
- Implementing `Add`, `Sub`, etc.
- Custom `Display` and `Debug`
- Making types work with operators

## Running the Example

```bash
cargo run
```

## Key Takeaways

1. **Traits define shared behavior** - like interfaces but more powerful
2. **Generics with trait bounds** - type safety with flexibility
3. **Default implementations** - reduce boilerplate
4. **Trait objects enable polymorphism** - at runtime cost
5. **Derivable traits save code** - automatic implementations
6. **Associated types** - cleaner than generics for single implementations

## Common Patterns

### Defining a trait
```rust
trait Draw {
    fn draw(&self);

    fn description(&self) -> String {
        String::from("A drawable object")
    }
}
```

### Implementing a trait
```rust
impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing circle");
    }
}
```

### Trait bounds
```rust
fn process<T: Draw + Clone>(item: T) {
    item.draw();
}

// Or with where clause
fn process<T>(item: T)
where
    T: Draw + Clone,
{
    item.draw();
}
```

### Trait objects
```rust
let shapes: Vec<Box<dyn Draw>> = vec![
    Box::new(Circle { radius: 5 }),
    Box::new(Square { size: 10 }),
];

for shape in shapes {
    shape.draw();
}
```

## Static vs Dynamic Dispatch

**Static dispatch (generics)**:
- Monomorphization - separate code for each type
- No runtime cost
- Cannot store different types in same collection

**Dynamic dispatch (trait objects)**:
- Virtual table (vtable) lookup at runtime
- Small performance cost
- Can store different types together

## Standard Library Traits

Common traits you'll use:
- `Display`, `Debug` - formatting
- `Clone`, `Copy` - duplication
- `Iterator` - iteration
- `From`, `Into` - conversion
- `Drop` - cleanup
- `Deref`, `DerefMut` - smart pointers
- `Add`, `Sub`, etc. - operators
