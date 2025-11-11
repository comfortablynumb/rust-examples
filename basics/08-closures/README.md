# Closures

This example demonstrates Rust's closures - anonymous functions that can capture their environment.

## Concepts Covered

### 1. Basic Syntax
- Closure definition `|params| expression`
- Multiple parameters
- Multiple statements with `{}`
- No parameters `|| expression`

### 2. Type Inference
- Automatic type inference
- Optional explicit annotations
- Type fixed on first use
- Closure vs function syntax

### 3. Capturing Environment
- Immutable borrow capture
- Mutable borrow capture
- Capturing multiple variables
- Automatic capture mode selection

### 4. Move Closures
- `move` keyword transfers ownership
- Essential for threads
- Copy types vs move types
- When to use move

### 5. Function Traits
- `Fn` - can be called multiple times, immutable borrow
- `FnMut` - can be called multiple times, mutable borrow
- `FnOnce` - called once, consumes captured values
- Trait hierarchy

### 6. Closures as Parameters
- Generic functions with trait bounds
- `impl Fn` syntax
- Different trait requirements
- Flexibility with generics

### 7. Returning Closures
- `impl Fn` return type
- `Box<dyn Fn>` for dynamic dispatch
- Factory patterns
- Closure generators

### 8. With Iterators
- `map`, `filter`, `fold` with closures
- Capturing environment in iterators
- Complex transformations
- Custom operations

## Running the Example

```bash
cargo run
```

## Key Takeaways

1. **Closures capture environment** - can access outer scope
2. **Three traits** - Fn, FnMut, FnOnce for different use cases
3. **Type inference** - usually no need for annotations
4. **Move for threads** - transfer ownership to thread closures
5. **Perfect with iterators** - functional programming patterns
6. **Zero-cost** - compile to efficient code

## Common Patterns

### Basic closure
```rust
let add = |x, y| x + y;
```

### Capturing
```rust
let threshold = 10;
let filter = |x| x > threshold;
```

### Move closure
```rust
let data = vec![1, 2, 3];
thread::spawn(move || {
    println!("{:?}", data);
});
```

### As parameter
```rust
fn apply<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}
```

### Returning closure
```rust
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}
```

### With iterators
```rust
numbers
    .iter()
    .filter(|x| **x % 2 == 0)
    .map(|x| x * x)
    .collect()
```

## Closure Traits

| Trait | Captured Variables | Can Call | Use Case |
|-------|-------------------|----------|----------|
| `Fn` | Immutable borrow | Multiple times | Read-only |
| `FnMut` | Mutable borrow | Multiple times | Mutation |
| `FnOnce` | Ownership | Once | Consuming |

**Trait hierarchy:** All closures implement `FnOnce`. Closures that don't move implement `FnMut`. Closures that don't mutate implement `Fn`.

## Closure vs Function

**Closures:**
- Can capture environment
- Type inference
- Anonymous
- Different type for each closure

**Functions:**
- Cannot capture environment
- Explicit types required
- Named
- Single type per signature

## When to Use

**Use closures for:**
- Iterator operations
- Callbacks
- Functional patterns
- Thread operations
- Event handlers
- Short, local functions

**Use functions for:**
- Public APIs
- Recursive functions
- No environment capture needed
- Complex logic
