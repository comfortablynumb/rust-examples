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
9. **[Macros](basics/11-macros/)** - Declarative macros (macro_rules!) and metaprogramming
10. **[Standard Library](basics/13-stdlib/)** - Collections, strings, time, memory utilities, and common traits
11. **[Modules](basics/14-modules/)** - Module system, visibility, organization, and best practices

### [Intermediate](intermediate/)

Advanced Rust concepts for experienced developers:

1. **[Smart Pointers](intermediate/01-smart-pointers/)** - Box, Rc, Arc, RefCell, Mutex, RwLock, Weak, Cow, and custom smart pointers
2. **[Concurrency](intermediate/02-concurrency/)** - Threads, channels, shared state, synchronization, and parallel processing
3. **[Unsafe Rust](intermediate/03-unsafe/)** - Unsafe blocks, raw pointers, FFI, unions, and safety contracts
4. **[Async/Await](intermediate/04-async/)** - Asynchronous programming with tokio, futures, and async patterns
5. **[Traits & Generics](intermediate/05-traits-generics/)** - Advanced traits, generics, associated types, and type-level programming
6. **[Testing](intermediate/06-testing/)** - Unit tests, integration tests, property-based testing, mocking, and benchmarking

### [Libraries](libraries/)

Popular Rust libraries and frameworks:

1. **[Actix Web](libraries/01-actix-web/)** - High-performance async web framework
2. **[Axum](libraries/02-axum/)** - Ergonomic web framework built on Tokio
3. **[Rocket](libraries/03-rocket/)** - Web framework with focus on ease of use
4. **[Serde](libraries/04-serde/)** - Serialization/deserialization framework (JSON, YAML, TOML, CSV)
5. **[Clap](libraries/05-clap/)** - Command-line argument parser with derive macros
6. **[Reqwest](libraries/06-reqwest/)** - HTTP client with async support
7. **[SQLx](libraries/07-sqlx/)** - Async SQL toolkit with compile-time query validation

### [Networking](networking/)

Network programming with various protocols and patterns:

1. **[UDP](networking/01-udp/)** - UDP server and client with datagram communication
2. **[TCP](networking/02-tcp/)** - TCP server and client with stream-based communication
3. **[TLS](networking/03-tls/)** - TLS encrypted connections with rustls
4. **[HTTP](networking/04-http/)** - HTTP server and client from scratch
5. **[DNS](networking/05-dns/)** - DNS server and client with hickory-dns

### [OS](os/)

Operating system interaction and system programming:

1. **[File Handling](os/01-file-handling/)** - File I/O, metadata, directories, and path handling
2. **[Environment](os/02-environment/)** - Command-line arguments, environment variables, and process management

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

### Basics (Start Here)

For beginners, we recommend following the basics category in order:

1. Start with **Ownership** to understand Rust's core concept
2. Learn **Match** for control flow and pattern matching
3. Study **Structs** and **Enums** for data organization
4. Understand **Traits** for shared behavior
5. Master **Error Handling** for robust programs
6. Explore **Iterators** and **Closures** for functional patterns
7. Study **Macros** for metaprogramming and code generation
8. Explore **Standard Library** for collections, strings, and common utilities
9. Learn **Modules** for code organization and project structure

### Intermediate (Advanced Concepts)

After mastering the basics, dive into intermediate topics:

1. **Smart Pointers** - Advanced memory management patterns
2. **Concurrency** - Thread-based parallel programming
3. **Async/Await** - Modern asynchronous programming
4. **Traits & Generics** - Advanced type system features
5. **Testing** - Comprehensive testing strategies
6. **Unsafe Rust** - When and how to break safety rules

### Real-World Applications

Then apply your skills with popular libraries and specialized domains:

- **Libraries** - Web frameworks (Actix, Axum, Rocket), serialization (Serde), CLI (Clap), HTTP (Reqwest), databases (SQLx)
- **Networking** - UDP, TCP, TLS, HTTP, DNS protocols
- **OS** - System programming and file handling

## Contributing

Each example should:
- Be self-contained and independent
- Include comprehensive comments
- Have a detailed README explaining concepts
- Compile without errors
- Demonstrate best practices

## License

MIT License - see LICENSE file for details
