# Operating System Examples

This directory contains examples of interacting with the operating system in Rust. Each example is an independent Cargo project demonstrating different OS-level operations.

## Examples

1. **[01-file-handling](01-file-handling/)** - File and directory operations, reading, writing, metadata
2. **[02-environment](02-environment/)** - Environment variables, command-line arguments, process management

## Key Concepts

### File System Operations
- **Files**: Reading, writing, appending, seeking
- **Directories**: Creating, listing, traversing, removing
- **Paths**: Cross-platform path handling
- **Metadata**: File size, permissions, timestamps
- **Permissions**: Unix permissions, file attributes

### Environment & Process
- **Environment Variables**: Reading and setting env vars
- **Command-Line Arguments**: Parsing program arguments
- **Process Management**: Spawning processes, pipes, exit codes
- **Working Directory**: Getting and changing current directory

## Running Examples

Each example can be run independently:

```bash
cd 01-file-handling
cargo run
```

## Rust's OS Abstractions

Rust provides excellent cross-platform abstractions for OS operations:
- **std::fs**: File system operations
- **std::path**: Path manipulation
- **std::env**: Environment and process information
- **std::process**: Process spawning and management
- **std::io**: Input/output operations

## Best Practices

1. **Error Handling**: OS operations can fail - always handle errors
2. **Path Handling**: Use `Path` and `PathBuf` for cross-platform paths
3. **Resource Cleanup**: Files are automatically closed (RAII)
4. **Permissions**: Check and set appropriate file permissions
5. **Security**: Validate paths to prevent directory traversal
6. **Atomicity**: Be aware of race conditions with file operations

## Common Patterns

### Reading a File
```rust
use std::fs;

let contents = fs::read_to_string("file.txt")?;
```

### Writing a File
```rust
use std::fs;

fs::write("file.txt", "Hello, World!")?;
```

### Iterating Directory
```rust
use std::fs;

for entry in fs::read_dir(".")? {
    let entry = entry?;
    println!("{:?}", entry.path());
}
```

### Environment Variable
```rust
use std::env;

let path = env::var("PATH")?;
```

### Command-Line Arguments
```rust
use std::env;

let args: Vec<String> = env::args().collect();
```
