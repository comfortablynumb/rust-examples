# Rayon - Data Parallelism

Effortless data parallelism with automatic work-stealing thread pools.

## Key Features

- **Easy Parallelism**: Change `.iter()` to `.par_iter()`
- **Work Stealing**: Automatic load balancing
- **Zero Configuration**: Uses optimal thread count by default
- **Composable**: Chain parallel operations
- **Safe**: Data race freedom guaranteed

## Parallel Iterators

```rust
use rayon::prelude::*;

// Sequential
vec.iter().map(|x| x * 2).sum()

// Parallel (just add `par_`)
vec.par_iter().map(|x| x * 2).sum()
```

## Common Operations

### Map

```rust
let doubled: Vec<_> = numbers
    .par_iter()
    .map(|&n| n * 2)
    .collect();
```

### Filter

```rust
let evens: Vec<_> = numbers
    .par_iter()
    .filter(|&&n| n % 2 == 0)
    .copied()
    .collect();
```

### Reduce

```rust
let sum: i32 = numbers
    .par_iter()
    .sum();

let max = numbers
    .par_iter()
    .reduce(|| &0, |a, b| if a > b { a } else { b });
```

### Sort

```rust
numbers.par_sort();
numbers.par_sort_by_key(|n| -n);
```

## When to Use

**Good for:**
- Large datasets (> 10,000 items)
- CPU-bound operations
- Independent computations
- Map/reduce patterns

**Not ideal for:**
- Small datasets (overhead > benefit)
- I/O-bound operations
- Operations requiring synchronization

## Running

```bash
cargo run
```

## Thread Pool Configuration

```rust
let pool = rayon::ThreadPoolBuilder::new()
    .num_threads(8)
    .build()?;

pool.install(|| {
    // Parallel work here
});
```

## Performance Tips

1. **Chunk Size**: Adjust for small items
2. **Avoid Synchronization**: Minimize locks/atomics
3. **Use `par_bridge()`**: For sequential iterators
4. **Profile First**: Measure before optimizing

## Rayon vs Async

| Rayon | Async (Tokio) |
|-------|---------------|
| CPU-bound | I/O-bound |
| Data parallelism | Task concurrency |
| Thread pool | Single/few threads |
| Blocking OK | Must be non-blocking |

## References

- [Rayon Documentation](https://docs.rs/rayon/)
- [Rayon Book](https://github.com/rayon-rs/rayon/blob/master/README.md)
