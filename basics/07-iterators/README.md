# Iterators

This example demonstrates Rust's powerful iterator system with adapters, consumers, and custom iterators.

## Concepts Covered

### 1. Creating Iterators
- `iter()` - immutable borrow
- `into_iter()` - takes ownership
- `iter_mut()` - mutable borrow
- Range iterators

### 2. Iterator Adapters (Lazy)
- `map()` - transform elements
- `filter()` - select elements
- `take()` / `skip()` - limit elements
- `enumerate()` - add indices
- `zip()` - combine iterators
- `rev()` - reverse order
- `flat_map()` - flatten nested structures
- `take_while()` / `skip_while()` - conditional

### 3. Iterator Consumers
- `collect()` - build collection
- `sum()` / `product()` - aggregate
- `count()` - count elements
- `max()` / `min()` - find extremes
- `find()` - first matching element
- `any()` / `all()` - predicates
- `fold()` / `reduce()` - accumulate
- `for_each()` - side effects

### 4. Chaining
- Combine multiple operations
- Readable data pipelines
- Lazy evaluation until consumed

### 5. Custom Iterators
- Implement `Iterator` trait
- Define `type Item`
- Implement `next()` method
- Automatic iterator methods

### 6. Common Patterns
- Map-reduce
- Partition
- Group operations
- Different collection types

### 7. Performance
- Zero-cost abstraction
- Same performance as hand-written loops
- Lazy evaluation
- Compiler optimizations

## Running the Example

```bash
cargo run
```

## Key Takeaways

1. **Iterators are lazy** - no work until consumed
2. **Zero-cost abstraction** - compiles to efficient code
3. **Composable** - chain operations easily
4. **Type-safe** - compiler catches errors
5. **Expressive** - clearer than manual loops
6. **Functional style** - map, filter, reduce patterns

## Common Patterns

### Map-filter-collect
```rust
let result: Vec<i32> = numbers
    .iter()
    .filter(|x| *x % 2 == 0)
    .map(|x| x * x)
    .collect();
```

### Find and process
```rust
if let Some(value) = numbers.iter().find(|x| **x > 10) {
    println!("Found: {}", value);
}
```

### Custom iterator
```rust
struct Counter { count: u32 }

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        self.count += 1;
        Some(self.count)
    }
}
```

### Chaining
```rust
numbers
    .iter()
    .filter(predicate)
    .map(transform)
    .take(10)
    .collect()
```

## Iterator vs Loop

**Iterator advantages:**
- More expressive
- Less error-prone
- Easier to chain operations
- Same performance

**When to use loops:**
- Complex control flow
- Early returns
- Mutable state updates

## Performance Tips

1. **Iterators are free** - same code as loops
2. **Use iterators** - more readable, equally fast
3. **Lazy evaluation** - only computes what's needed
4. **Avoid collect()** - if you don't need collection
5. **Chain operations** - single pass through data
