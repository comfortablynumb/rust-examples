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
6. **[WebSocket](04-networking/06-websocket/)** - Real-time bidirectional communication
7. **[gRPC](04-networking/07-grpc/)** - Protocol buffers and RPC with Tonic
8. **[QUIC](04-networking/08-quic/)** - Modern transport protocol with Quinn
9. **[GraphQL](04-networking/09-graphql/)** - GraphQL server with async-graphql

### [05. OS](05-os/)

Operating system interaction and system programming:

1. **[File Handling](05-os/01-file-handling/)** - File I/O, metadata, directories, and path handling
2. **[Environment](05-os/02-environment/)** - Command-line arguments, environment variables, and process management
3. **[Process Management](05-os/03-process-management/)** - Spawning processes, pipes, and stdio redirection
4. **[Signals](05-os/04-signals/)** - Unix signal handling for graceful shutdown
5. **[Filesystem Watching](05-os/05-filesystem-watching/)** - Monitor file system events with notify
6. **[Memory Mapping](05-os/06-memory-mapping/)** - Memory-mapped files and shared memory (mmap)
7. **[Permissions](05-os/07-permissions/)** - Unix file permissions and access control

### [06. Libraries](06-libraries/)

Popular Rust libraries and frameworks:

1. **[Actix Web](06-libraries/01-actix-web/)** - High-performance async web framework
2. **[Axum](06-libraries/02-axum/)** - Ergonomic web framework built on Tokio
3. **[Rocket](06-libraries/03-rocket/)** - Web framework with focus on ease of use
4. **[Serde](06-libraries/04-serde/)** - Serialization/deserialization framework (JSON, YAML, TOML, CSV)
5. **[Clap](06-libraries/05-clap/)** - Command-line argument parser with derive macros
6. **[Reqwest](06-libraries/06-reqwest/)** - HTTP client with async support
7. **[SQLx](06-libraries/07-sqlx/)** - Async SQL toolkit with compile-time query validation
8. **[Rayon](06-libraries/08-rayon/)** - Data parallelism with work-stealing thread pools
9. **[Tokio Advanced](06-libraries/09-tokio-advanced/)** - Advanced async runtime features (timeouts, select, channels)
10. **[Tracing](06-libraries/10-tracing/)** - Structured logging and diagnostics

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

### [10. WebAssembly](10-wasm/)

Compile Rust to WebAssembly for web applications:

1. **[WASM Basics](10-wasm/01-wasm-basics/)** - Basic Rust to WASM compilation and JS interop
2. **[DOM Manipulation](10-wasm/02-dom-manipulation/)** - Browser DOM interaction with web-sys
3. **[wasm-pack](10-wasm/03-wasm-pack/)** - Building and publishing WASM packages to npm
4. **[Canvas Graphics](10-wasm/04-canvas-graphics/)** - Drawing graphics on HTML canvas
5. **[Web Workers](10-wasm/05-web-workers/)** - CPU-intensive tasks in background threads
6. **[Local Storage](10-wasm/06-local-storage/)** - Browser storage APIs for persistent data

### [11. GUI](11-gui/)

Desktop GUI applications with various frameworks:

1. **[egui Basics](11-gui/01-egui-basics/)** - Immediate mode GUI basics with egui
2. **[egui App](11-gui/02-egui-app/)** - Complex egui application with tabs and plotting
3. **[iced Counter](11-gui/03-iced-counter/)** - Simple counter with iced framework
4. **[iced Todo](11-gui/04-iced-todo/)** - TodoMVC application with iced
5. **[Slint](11-gui/06-slint/)** - Declarative UI with Slint markup language

### [12. Database](12-database/)

Database integration and data persistence:

1. **[SQLx PostgreSQL](12-database/01-sqlx-postgres/)** - Async PostgreSQL with compile-time checked queries
2. **[SQLx SQLite](12-database/02-sqlx-sqlite/)** - Lightweight embedded database with SQLx
3. **[Diesel](12-database/03-diesel/)** - Type-safe ORM with query builder
4. **[Redis](12-database/04-redis/)** - In-memory data store for caching and messaging
5. **[Sled](12-database/05-sled/)** - Embedded database written in pure Rust
6. **[Connection Pooling](12-database/06-connection-pool/)** - Efficient database connection management
7. **[Migrations](12-database/07-migrations/)** - Database schema version control

### [13. Security & Encryption](13-security/)

Cryptography and security best practices:

1. **[Hashing](13-security/01-hashing/)** - SHA-256, BLAKE3, and HMAC
2. **[Password Hashing](13-security/02-password-hashing/)** - Argon2 and bcrypt for secure password storage
3. **[Symmetric Encryption](13-security/03-symmetric-encryption/)** - AES-GCM and ChaCha20-Poly1305
4. **[Asymmetric Encryption](13-security/04-asymmetric-encryption/)** - RSA public-key cryptography
5. **[Digital Signatures](13-security/05-digital-signatures/)** - Ed25519 signatures for authentication
6. **[JWT](13-security/06-jwt/)** - JSON Web Tokens for stateless authentication
7. **[TLS Certificates](13-security/07-tls-certificates/)** - X.509 certificate generation
8. **[Crypto Random](13-security/08-crypto-random/)** - Cryptographically secure random numbers

## Getting Started

### Prerequisites

- Rust and Cargo installed ([rustup.rs](https://rustup.rs/))
- Basic understanding of programming concepts

**Note:** Some examples require additional system dependencies (like GTK for GUI apps or protoc for gRPC). See [SYSTEM_DEPENDENCIES.md](SYSTEM_DEPENDENCIES.md) for details.

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

### 04-13. Specialized Domains

Apply your skills with specialized domains and technologies:

- **04. Networking** - UDP, TCP, TLS, HTTP, DNS, WebSocket, gRPC, QUIC, GraphQL
- **05. OS** - File handling, process management, signals, filesystem watching, memory mapping, permissions
- **06. Libraries** - Web frameworks (Actix, Axum, Rocket), serialization (Serde), CLI (Clap), HTTP (Reqwest), databases (SQLx), parallelism (Rayon), async runtime (Tokio), logging (Tracing)
- **07. CLI** - Terminal UIs (ratatui), argument parsing (clap), progress bars (indicatif), interactive prompts (dialoguer)
- **08. 3D Graphics** - GPU programming with wgpu, shaders, lighting, compute shaders
- **09. Bare Metal** - Embedded systems (ARM, RISC-V), OS development, no_std programming
- **10. WebAssembly** - Compile Rust to WASM, browser APIs, web workers, DOM manipulation
- **11. GUI** - Desktop applications with egui, iced, and Slint frameworks
- **12. Database** - SQL databases (PostgreSQL, SQLite), NoSQL (Redis), ORMs (Diesel, SQLx), embedded databases (Sled)
- **13. Security & Encryption** - Cryptographic hashing, password hashing, symmetric/asymmetric encryption, digital signatures, JWT

## Contributing

Each example should:
- Be self-contained and independent
- Include comprehensive comments
- Have a detailed README explaining concepts
- Compile without errors
- Demonstrate best practices

## License

MIT License - see LICENSE file for details
