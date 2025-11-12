# Rust Basics Examples

This directory contains comprehensive examples of core Rust language concepts. Each example is an independent Cargo project with its own README explaining the concepts demonstrated.

## Examples

1. **[01-ownership](01-ownership/)** - Ownership, borrowing, references, and lifetimes
2. **[02-match](02-match/)** - Pattern matching and match expressions
3. **[03-structs](03-structs/)** - Structs, methods, and associated functions
4. **[04-enums](04-enums/)** - Enumerations and pattern matching with enums
5. **[05-traits](05-traits/)** - Traits, trait bounds, and trait objects
6. **[06-error-handling](06-error-handling/)** - Result, Option, and error propagation
7. **[07-iterators](07-iterators/)** - Iterators, iterator adapters, and consumers
8. **[08-closures](08-closures/)** - Closures, capturing environment, and function traits
9. **[09-smart-pointers](09-smart-pointers/)** - Box, Rc, Arc, RefCell, and interior mutability
10. **[10-concurrency](10-concurrency/)** - Threads, channels, and shared state
11. **[11-unsafe](11-unsafe/)** - Introduction to unsafe Rust and raw pointers
12. **[12-stdlib](12-stdlib/)** - Standard library collections, algorithms, and utilities
13. **[13-modules](13-modules/)** - Module system, visibility, and code organization

## Running Examples

Each example can be run independently:

```bash
cd 01-ownership
cargo run
```

Or build and run all examples:

```bash
for dir in */; do
  cd "$dir"
  cargo run
  cd ..
done
```
