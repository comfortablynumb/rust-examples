# Rust Examples

[![CI](https://github.com/comfortablynumb/rust-examples/workflows/CI/badge.svg)](https://github.com/comfortablynumb/rust-examples/actions)

A comprehensive collection of Rust examples organized by category. Each example is an independent Cargo project with its own README explaining the concepts demonstrated.

## Categories

### [01. Basics](01-basics/)

Core Rust language concepts and fundamental programming patterns:

1. **[Ownership](01-basics/01-ownership/)** - Ownership, borrowing, references, and lifetimes
2. **[Match](01-basics/02-match/)** - Pattern matching and match expressions
3. **[Structs](01-basics/03-structs/)** - Structs, methods, and associated functions
4. **[Enums](01-basics/04-enums/)** - Enumerations and pattern matching with enums
5. **[Traits](01-basics/05-traits/)** - Traits, trait bounds, and trait objects
6. **[Error Handling](01-basics/06-error-handling/)** - Result, Option, and error propagation
7. **[Iterators](01-basics/07-iterators/)** - Iterators, iterator adapters, and consumers
8. **[Closures](01-basics/08-closures/)** - Closures, capturing environment, and function traits
9. **[Modules](01-basics/09-modules/)** - Module system, visibility, organization, and best practices

### [02. Intermediate](02-intermediate/)

Advanced Rust concepts for experienced developers:

1. **[Concurrency](02-intermediate/01-concurrency/)** - Threads, channels, shared state, synchronization, and parallel processing
2. **[Async/Await](02-intermediate/02-async/)** - Asynchronous programming with tokio, futures, and async patterns
3. **[Traits & Generics](02-intermediate/03-traits-generics/)** - Advanced traits, generics, associated types, and type-level programming
4. **[Testing](02-intermediate/04-testing/)** - Unit tests, integration tests, property-based testing, mocking, and benchmarking
5. **[Cargo](02-intermediate/05-cargo/)** - Cargo workspaces, custom build scripts, and project organization
6. **[Standard Library](02-intermediate/06-stdlib/)** - Collections, strings, time, memory utilities, and common traits

### [03. Advanced](03-advanced/)

Expert-level Rust programming and advanced language features:

1. **[Unsafe Rust](03-advanced/01-unsafe/)** - Unsafe blocks, raw pointers, FFI, unions, and safety contracts
2. **[Smart Pointers](03-advanced/02-smart-pointers/)** - Box, Rc, Arc, RefCell, Mutex, RwLock, Weak, Cow, and custom smart pointers
3. **[Macros](03-advanced/03-macros/)** - Declarative macros (macro_rules!) and metaprogramming
4. **[Advanced Traits](03-advanced/04-advanced-traits/)** - Associated types, GATs, trait objects, and advanced trait patterns
5. **[Advanced Types](03-advanced/05-advanced-types/)** - Type aliases, newtype pattern, DSTs, and advanced type system features
6. **[Advanced Closures](03-advanced/06-advanced-closures/)** - Closure internals, Fn traits, and advanced closure patterns

### [04. Networking](04-networking/)

Network programming with various protocols and patterns:

1. **[UDP](04-networking/01-udp/)** - UDP server and client with datagram communication
2. **[TCP](04-networking/02-tcp/)** - TCP server and client with stream-based communication
3. **[TLS](04-networking/03-tls/)** - TLS encrypted connections with rustls
4. **[HTTP](04-networking/04-http/)** - HTTP server and client from scratch
5. **[DNS](04-networking/05-dns/)** - DNS server and client with hickory-dns

### [05. OS](05-os/)

Operating system interaction and system programming:

1. **[File Handling](05-os/01-file-handling/)** - File I/O, metadata, directories, and path handling
2. **[Environment](05-os/02-environment/)** - Command-line arguments, environment variables, and process management

### [06. Libraries](06-libraries/)

Popular Rust libraries and frameworks:

1. **[Actix Web](06-libraries/01-actix-web/)** - High-performance async web framework
2. **[Axum](06-libraries/02-axum/)** - Ergonomic web framework built on Tokio
3. **[Rocket](06-libraries/03-rocket/)** - Web framework with focus on ease of use
4. **[Serde](06-libraries/04-serde/)** - Serialization/deserialization framework (JSON, YAML, TOML, CSV)
5. **[Clap](06-libraries/05-clap/)** - Command-line argument parser with derive macros
6. **[Reqwest](06-libraries/06-reqwest/)** - HTTP client with async support
7. **[SQLx](06-libraries/07-sqlx/)** - Async SQL toolkit with compile-time query validation

### [07. CLI](07-cli/)

Command-line interface applications and TUI frameworks:

1. **[Clap Advanced](07-cli/01-clap-advanced/)** - Advanced CLI argument parsing with subcommands, validators, and config files
2. **[Ratatui](07-cli/02-ratatui/)** - Terminal UI dashboard with widgets, charts, and real-time updates
3. **[Crossterm](07-cli/03-crossterm/)** - Direct terminal manipulation, colors, cursor control, and raw mode
4. **[Indicatif](07-cli/04-indicatif/)** - Progress bars, spinners, and multi-progress for parallel tasks
5. **[Dialoguer](07-cli/05-dialoguer/)** - Interactive prompts, menus, and user input validation

### [08. 3D Graphics](08-3d/)

3D graphics programming with wgpu (WebGPU):

1. **[Triangle](08-3d/01-triangle/)** - Basic wgpu setup and rendering pipeline
2. **[Buffers & Indices](08-3d/02-buffers-indices/)** - Vertex and index buffers with interleaved data
3. **[Textures](08-3d/03-textures/)** - Texture loading, samplers, and UV mapping
4. **[3D Cube](08-3d/04-3d-cube/)** - 3D transformations with MVP matrices and depth testing
5. **[Camera](08-3d/05-camera/)** - Interactive camera with WASD movement and mouse look
6. **[Lighting](08-3d/06-lighting/)** - Phong shading with ambient, diffuse, and specular components
7. **[Multiple Objects](08-3d/07-multiple-objects/)** - GPU instancing for rendering many objects efficiently
8. **[Compute Shader](08-3d/08-compute-shader/)** - Particle systems using compute shaders

### [09. Bare Metal](09-bare-metal/)

Embedded systems and OS development without standard library (no_std):

1. **[No-Std Basics](09-bare-metal/01-no-std-basics/)** - Introduction to no_std programming and panic handlers
2. **[Custom Allocator](09-bare-metal/02-custom-allocator/)** - Implementing bump and linked-list allocators
3. **[Embedded HAL](09-bare-metal/03-embedded-hal/)** - Hardware abstraction layer for portable embedded code
4. **[ARM Cortex-M](09-bare-metal/04-arm-cortex-m/)** - ARM programming with interrupts and peripherals
5. **[Bootloader](09-bare-metal/05-bootloader/)** - x86_64 OS kernel with VGA output and IDT
6. **[RISC-V](09-bare-metal/06-riscv-baremetal/)** - RISC-V bare metal with CSRs and trap handling
7. **[Heapless Collections](09-bare-metal/07-heapless-collections/)** - Fixed-capacity data structures without heap allocation

## Getting Started

### Prerequisites

- Rust and Cargo installed ([rustup.rs](https://rustup.rs/))
- Basic understanding of programming concepts

### Running Examples

Each example is self-contained and can be run independently:

```bash
cd 01-basics/01-ownership
cargo run
```

Or to run all examples in a category:

```bash
cd 01-basics
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

### 01. Basics (Start Here)

For beginners, we recommend following the basics category in order:

1. Start with **Ownership** to understand Rust's core concept
2. Learn **Match** for control flow and pattern matching
3. Study **Structs** and **Enums** for data organization
4. Understand **Traits** for shared behavior
5. Master **Error Handling** for robust programs
6. Explore **Iterators** and **Closures** for functional patterns
7. Learn **Modules** for code organization and project structure

### 02. Intermediate (Level Up)

After mastering the basics, dive into intermediate topics:

1. **Concurrency** - Thread-based parallel programming
2. **Async/Await** - Modern asynchronous programming
3. **Traits & Generics** - Advanced type system features
4. **Testing** - Comprehensive testing strategies
5. **Cargo** - Workspaces and project organization
6. **Standard Library** - Collections, strings, time, and memory utilities

### 03. Advanced (Expert Level)

For experienced Rustaceans ready to master the language:

1. **Unsafe Rust** - Deep dive into unsafe blocks, raw pointers, FFI, and safety contracts
2. **Smart Pointers** - Advanced memory management with custom smart pointers
3. **Macros** - Metaprogramming and code generation with macro_rules!
4. **Advanced Traits** - Associated types, GATs, and complex trait patterns
5. **Advanced Types** - Type system internals and advanced type features
6. **Advanced Closures** - Closure internals and the Fn trait family

### 04-09. Specialized Domains

Apply your skills with specialized domains and technologies:

- **04. Networking** - UDP, TCP, TLS, HTTP, DNS protocols
- **05. OS** - System programming and file handling
- **06. Libraries** - Web frameworks (Actix, Axum, Rocket), serialization (Serde), CLI (Clap), HTTP (Reqwest), databases (SQLx)
- **07. CLI** - Terminal UIs (ratatui), argument parsing (clap), progress bars (indicatif), interactive prompts (dialoguer)
- **08. 3D Graphics** - GPU programming with wgpu, shaders, lighting, compute shaders
- **09. Bare Metal** - Embedded systems (ARM, RISC-V), OS development, no_std programming

## Contributing

Each example should:
- Be self-contained and independent
- Include comprehensive comments
- Have a detailed README explaining concepts
- Compile without errors
- Demonstrate best practices

## License

MIT License - see LICENSE file for details
