# Cargo Workspace Example

A comprehensive example demonstrating Cargo workspace features, dependency management, build scripts, and more.

## Workspace Structure

```
05-cargo/
├── Cargo.toml              # Workspace root configuration
├── lib-core/               # Library crate
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── bin-app/                # Binary crate (depends on lib-core)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── build-scripts/          # Build script example
    ├── Cargo.toml
    ├── build.rs            # Custom build script
    └── src/
        └── main.rs
```

## Features Demonstrated

### 1. Workspace Configuration

The root `Cargo.toml` defines:
- **Workspace members**: All crates in the workspace
- **Shared dependencies**: Common dependencies used across crates
- **Workspace metadata**: Version, edition, authors, license
- **Build profiles**: dev, release, and custom profiles

### 2. Shared Dependencies

Dependencies defined at the workspace level can be reused by member crates:

```toml
# In root Cargo.toml
[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
anyhow = "1.0"

# In member Cargo.toml
[dependencies]
anyhow = { workspace = true }
```

### 3. Cargo Features

`lib-core` demonstrates conditional compilation with features:

- **`json`** (default): JSON serialization support
- **`extra`**: Additional utility functions
- **`advanced`**: Advanced analysis features

### 4. Path Dependencies

`bin-app` depends on `lib-core` using a path dependency:

```toml
[dependencies]
lib-core = { path = "../lib-core", features = ["json", "extra"] }
```

### 5. Build Scripts

`build-scripts` includes a `build.rs` that:
- Generates Rust code at compile time
- Sets environment variables
- Provides build metadata
- Demonstrates conditional compilation flags

### 6. Build Profiles

Custom build profiles defined in the workspace root:
- **`dev`**: Standard development build (opt-level 0)
- **`release`**: Optimized release build (opt-level 3, LTO enabled)
- **`fast-dev`**: Faster development builds (opt-level 1)
- **`profiling`**: Release build with debug symbols

## Usage Examples

### Building the Workspace

```bash
# Build all workspace members
cargo build

# Build specific crate
cargo build -p lib-core
cargo build -p bin-app
cargo build -p build-scripts

# Build with specific profile
cargo build --profile fast-dev
cargo build --profile profiling
cargo build --release
```

### Running Binaries

```bash
# Run the main application
cargo run -p bin-app

# Run with specific features
cargo run -p bin-app --no-default-features
cargo run -p bin-app --all-features

# Run the build-scripts example
cargo run -p build-scripts

# Run in release mode
cargo run -p bin-app --release
```

### Testing

```bash
# Test all workspace members
cargo test

# Test specific crate
cargo test -p lib-core

# Test with specific features
cargo test -p lib-core --no-default-features
cargo test -p lib-core --features json
cargo test -p lib-core --features extra
cargo test -p lib-core --all-features
```

### Working with Features

```bash
# Build lib-core with default features (json)
cargo build -p lib-core

# Build without default features
cargo build -p lib-core --no-default-features

# Build with specific features
cargo build -p lib-core --features extra
cargo build -p lib-core --features "json,extra"
cargo build -p lib-core --features advanced

# Build with all features
cargo build -p lib-core --all-features
```

### Checking Code Quality

```bash
# Check all workspace members
cargo check

# Run clippy linter
cargo clippy

# Format code
cargo fmt

# Run clippy with all features
cargo clippy --workspace --all-features
```

### Custom Cargo Commands

```bash
# Install cargo-tree to visualize dependencies
cargo install cargo-tree
cargo tree

# Install cargo-expand to see macro expansions
cargo install cargo-expand
cargo expand -p lib-core

# Install cargo-watch for auto-recompilation
cargo install cargo-watch
cargo watch -x "test -p lib-core"

# Install cargo-audit for security audits
cargo install cargo-audit
cargo audit
```

### Dependency Management

```bash
# Show dependency tree
cargo tree

# Show dependency tree for specific crate
cargo tree -p lib-core

# Show only direct dependencies
cargo tree --depth 1

# Update dependencies
cargo update

# Update specific dependency
cargo update -p serde
```

### Documentation

```bash
# Build documentation for all crates
cargo doc

# Build and open documentation
cargo doc --open

# Build documentation with all features
cargo doc --all-features

# Build documentation for specific crate
cargo doc -p lib-core --open
```

## Key Concepts

### Workspace Benefits

1. **Unified dependency resolution**: All crates share the same `Cargo.lock`
2. **Shared build artifacts**: Compiled dependencies are reused
3. **Easier maintenance**: Update dependencies in one place
4. **Coordinated releases**: Version all crates together

### Feature Flags Best Practices

1. Use `default` feature for common functionality
2. Make features additive (adding a feature shouldn't break code)
3. Use optional dependencies with features
4. Document all features in crate documentation

### Build Script Guidelines

1. Only regenerate when inputs change (`cargo:rerun-if-changed`)
2. Write generated files to `OUT_DIR`
3. Use `cargo:` prefixed println! for Cargo communication
4. Keep build scripts fast and simple

### Profile Optimization

- **dev**: Fast compilation, poor runtime performance
- **release**: Slow compilation, excellent runtime performance
- **Custom profiles**: Balance between the two

## Common Commands Reference

| Command | Description |
|---------|-------------|
| `cargo build` | Build all workspace members |
| `cargo build -p <crate>` | Build specific crate |
| `cargo run -p <crate>` | Run binary from specific crate |
| `cargo test` | Run all tests in workspace |
| `cargo test -p <crate>` | Run tests for specific crate |
| `cargo check` | Quick syntax check |
| `cargo clippy` | Run linter |
| `cargo fmt` | Format code |
| `cargo clean` | Remove build artifacts |
| `cargo update` | Update dependencies |
| `cargo tree` | Show dependency tree |
| `cargo doc --open` | Build and open documentation |

## Example Output

### Running bin-app

```bash
$ cargo run -p bin-app
=== Cargo Workspace Example - Binary App ===

Created data items:
  Data { id: 1, name: "Alice", value: 100.0 }
  Data { id: 2, name: "Bob", value: 150.0 }
  Data { id: 3, name: "Charlie", value: 200.0 }

Validating data...
  All data valid!

Calculated values:
  Data 1: 201.0
  Data 2: 302.0
  Data 3: 403.0

=== JSON Serialization (feature: json) ===
Serialized data1:
{
  "id": 1,
  "name": "Alice",
  "value": 100.0
}

=== Extra Utilities (feature: extra) ===
Batch processed values: [201.0, 302.0, 403.0]
Maximum value: Some(200.0)
Average value: 150.0

=== Advanced Analysis (feature: advanced) ===
Analysis Results:
  Count:   3
  Sum:     450.00
  Average: 150.00
  Min:     100.00
  Max:     200.00

=== Done! ===
```

### Running build-scripts

```bash
$ cargo run -p build-scripts
=== Build Scripts Example ===

Build Profile: debug
Build Target: x86_64-unknown-linux-gnu
Build Timestamp: 2025-11-12T10:30:00+00:00

Built with profile 'debug' for target 'x86_64-unknown-linux-gnu' at 2025-11-12T10:30:00+00:00

Environment variables set by build.rs:
  BUILD_PROFILE: debug
  BUILD_TARGET: x86_64-unknown-linux-gnu

This was built in debug/dev mode!

=== Done! ===
```

## Learning Resources

- [The Cargo Book](https://doc.rust-lang.org/cargo/)
- [Cargo Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
- [Build Scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html)
- [Features](https://doc.rust-lang.org/cargo/reference/features.html)
- [Profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)

## License

MIT
