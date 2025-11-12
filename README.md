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
9. **[Unsafe Rust](basics/11-unsafe/)** - Unsafe blocks, raw pointers, and when to use unsafe code
10. **[Standard Library](basics/12-stdlib/)** - Collections, strings, time, memory utilities, and common traits
11. **[Modules](basics/13-modules/)** - Module system, visibility, organization, and best practices

### [Intermediate](intermediate/)

Advanced Rust concepts for experienced developers:

1. **[Concurrency](intermediate/01-concurrency/)** - Threads, channels, shared state, synchronization, and parallel processing
2. **[Async/Await](intermediate/02-async/)** - Asynchronous programming with tokio, futures, and async patterns
3. **[Traits & Generics](intermediate/03-traits-generics/)** - Advanced traits, generics, associated types, and type-level programming
4. **[Testing](intermediate/04-testing/)** - Unit tests, integration tests, property-based testing, mocking, and benchmarking
5. **[Cargo](intermediate/05-cargo/)** - Cargo workspaces, custom build scripts, and project organization

### [Advanced](advanced/)

Expert-level Rust programming and advanced language features:

1. **[Unsafe Rust](advanced/01-unsafe/)** - Unsafe blocks, raw pointers, FFI, unions, and safety contracts
2. **[Smart Pointers](advanced/02-smart-pointers/)** - Box, Rc, Arc, RefCell, Mutex, RwLock, Weak, Cow, and custom smart pointers
3. **[Macros](advanced/03-macros/)** - Declarative macros (macro_rules!) and metaprogramming
4. **[Advanced Traits](advanced/04-advanced-traits/)** - Associated types, GATs, trait objects, and advanced trait patterns
5. **[Advanced Types](advanced/05-advanced-types/)** - Type aliases, newtype pattern, DSTs, and advanced type system features
6. **[Advanced Closures](advanced/06-advanced-closures/)** - Closure internals, Fn traits, and advanced closure patterns

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
7. Introduction to **Unsafe Rust** basics
8. Explore **Standard Library** for collections, strings, and common utilities
9. Learn **Modules** for code organization and project structure

### Intermediate (Level Up)

After mastering the basics, dive into intermediate topics:

1. **Concurrency** - Thread-based parallel programming
2. **Async/Await** - Modern asynchronous programming
3. **Traits & Generics** - Advanced type system features
4. **Testing** - Comprehensive testing strategies
5. **Cargo** - Workspaces and project organization

### Advanced (Expert Level)

For experienced Rustaceans ready to master the language:

1. **Unsafe Rust** - Deep dive into unsafe blocks, raw pointers, FFI, and safety contracts
2. **Smart Pointers** - Advanced memory management with custom smart pointers
3. **Macros** - Metaprogramming and code generation with macro_rules!
4. **Advanced Traits** - Associated types, GATs, and complex trait patterns
5. **Advanced Types** - Type system internals and advanced type features
6. **Advanced Closures** - Closure internals and the Fn trait family

### Real-World Applications

Apply your skills with popular libraries and specialized domains:

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
