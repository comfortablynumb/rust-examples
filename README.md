# Rust Examples

[![CI](https://github.com/comfortablynumb/rust-examples/workflows/CI/badge.svg)](https://github.com/comfortablynumb/rust-examples/actions)

A comprehensive collection of Rust examples organized by category. Each example is an independent Cargo project with its own README explaining the concepts demonstrated.

## Categories

### [Basics](basics/)

Core Rust language concepts and fundamental programming patterns:

1. **[Ownership](basics/01-ownership/)** - Ownership, borrowing, references, and lifetimes
2. **[Match](basics/02-match/)** - Pattern matching and match expressions
3. **[Structs](basics/03-structs/)** - Structs, methods, and associated functions
4. **[Enums](basics/04-enums/)** - Enumerations and pattern matching with enums
5. **[Traits](basics/05-traits/)** - Traits, trait bounds, and trait objects
6. **[Error Handling](basics/06-error-handling/)** - Result, Option, and error propagation
7. **[Iterators](basics/07-iterators/)** - Iterators, iterator adapters, and consumers
8. **[Closures](basics/08-closures/)** - Closures, capturing environment, and function traits
9. **[Smart Pointers](basics/09-smart-pointers/)** - Box, Rc, Arc, RefCell, and interior mutability
10. **[Concurrency](basics/10-concurrency/)** - Threads, channels, and shared state

### [Networking](networking/)

Network programming with various protocols and patterns:

1. **[UDP](networking/01-udp/)** - UDP server and client with datagram communication
2. **[TCP](networking/02-tcp/)** - TCP server and client with stream-based communication
3. **[TLS](networking/03-tls/)** - TLS encrypted connections with rustls
4. **[HTTP](networking/04-http/)** - HTTP server and client from scratch
5. **[DNS](networking/05-dns/)** - DNS server and client with hickory-dns

## Getting Started

### Prerequisites

- Rust and Cargo installed ([rustup.rs](https://rustup.rs/))
- Basic understanding of programming concepts

### Running Examples

Each example is self-contained and can be run independently:

```bash
cd basics/01-ownership
cargo run
```

Or to run all examples in a category:

```bash
cd basics
for dir in */; do
  echo "Running $dir"
  cd "$dir"
  cargo run
  cd ..
done
```

## Structure

Each example follows this structure:

```
example-name/
├── Cargo.toml          # Project configuration
├── README.md           # Detailed explanation of concepts
└── src/
    └── main.rs         # Example code with comments
```

## Learning Path

For beginners, we recommend following the basics category in order:

1. Start with **Ownership** to understand Rust's core concept
2. Learn **Match** for control flow and pattern matching
3. Study **Structs** and **Enums** for data organization
4. Understand **Traits** for shared behavior
5. Master **Error Handling** for robust programs
6. Explore **Iterators** and **Closures** for functional patterns
7. Learn **Smart Pointers** for advanced memory management
8. Finally, tackle **Concurrency** for parallel programming

## Contributing

Each example should:
- Be self-contained and independent
- Include comprehensive comments
- Have a detailed README explaining concepts
- Compile without errors
- Demonstrate best practices

## License

MIT License - see LICENSE file for details
