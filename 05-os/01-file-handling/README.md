# File Handling

This example demonstrates comprehensive file system operations in Rust using the standard library.

## Concepts Covered

### Basic Operations
- Reading and writing files
- Appending to files
- File creation and deletion
- Copying and renaming files

### Buffered I/O
- `BufReader` for efficient reading
- `BufWriter` for efficient writing
- Line-by-line reading
- Flushing buffers

### File Metadata
- File size and timestamps
- File type checking (file, directory, symlink)
- Permissions and attributes
- Creation and modification times

### Directory Operations
- Creating directories recursively
- Listing directory contents
- Recursive directory traversal
- Removing directories

### Path Handling
- `Path` and `PathBuf` types
- Path components (parent, file name, extension)
- Building and joining paths
- Cross-platform path handling

### Advanced Operations
- File seeking (random access)
- Hard and symbolic links (Unix)
- File metadata without reading content

## Running the Example

```bash
cargo run
```

The example creates and manipulates files, then cleans up after itself.

## Key Takeaways

1. **RAII**: Files automatically close when dropped
2. **Error handling**: All file operations return `Result`
3. **Buffering**: Use `BufReader`/`BufWriter` for efficiency
4. **Paths**: Use `Path`/`PathBuf` for cross-platform code
5. **Safety**: Rust prevents common file handling bugs

## Common Patterns

### Read entire file
```rust
use std::fs;

let contents = fs::read_to_string("file.txt")?;
```

### Write entire file
```rust
use std::fs;

fs::write("file.txt", "content")?;
```

### Buffered line reading
```rust
use std::fs::File;
use std::io::{BufRead, BufReader};

let file = File::open("file.txt")?;
let reader = BufReader::new(file);

for line in reader.lines() {
    println!("{}", line?);
}
```

### Append to file
```rust
use std::fs::OpenOptions;
use std::io::Write;

let mut file = OpenOptions::new()
    .append(true)
    .open("file.txt")?;

writeln!(file, "new line")?;
```

### Create directory
```rust
use std::fs;

fs::create_dir_all("path/to/dir")?;
```

### List directory
```rust
use std::fs;

for entry in fs::read_dir(".")? {
    let entry = entry?;
    println!("{:?}", entry.path());
}
```

### File metadata
```rust
use std::fs;

let metadata = fs::metadata("file.txt")?;
println!("Size: {}", metadata.len());
println!("Is file: {}", metadata.is_file());
```

### Path operations
```rust
use std::path::{Path, PathBuf};

let path = Path::new("dir/file.txt");
let parent = path.parent();
let file_name = path.file_name();
let extension = path.extension();

let mut buf = PathBuf::from("dir");
buf.push("file.txt");
```

## Error Handling

Common file I/O errors:
- `NotFound`: File or directory doesn't exist
- `PermissionDenied`: Insufficient permissions
- `AlreadyExists`: File already exists (when using `create_new`)
- `InvalidInput`: Invalid path or filename
- `WouldBlock`: Non-blocking operation would block
- `UnexpectedEof`: Unexpected end of file

## Best Practices

1. **Always handle errors** - File operations can fail
2. **Use buffered I/O** - More efficient for multiple operations
3. **Close files explicitly** (or rely on RAII)
4. **Use `Path`/`PathBuf`** - Cross-platform path handling
5. **Check existence** before operations
6. **Set appropriate permissions** - Especially on Unix
7. **Avoid hardcoded paths** - Use path joining
8. **Clean up temporary files**

## Platform-Specific Features

### Unix-specific
```rust
#[cfg(unix)]
use std::os::unix::fs::{symlink, PermissionsExt};

// Create symbolic link
symlink("target", "link")?;

// Set Unix permissions
let mut perms = fs::metadata("file")?.permissions();
perms.set_mode(0o644);
fs::set_permissions("file", perms)?;
```

### Windows-specific
```rust
#[cfg(windows)]
use std::os::windows::fs::{symlink_file, symlink_dir};

symlink_file("target.txt", "link.txt")?;
symlink_dir("target_dir", "link_dir")?;
```

## Performance Tips

1. **Use buffered I/O** - Reduces system calls
2. **Read in chunks** - Don't read entire large files at once
3. **Memory-map large files** - Use `memmap` crate
4. **Async I/O** - Use `tokio::fs` for async operations
5. **Batch operations** - Combine multiple writes

## Security Considerations

1. **Validate paths** - Prevent directory traversal
2. **Check permissions** - Verify file access rights
3. **Avoid race conditions** - TOCTOU (Time Of Check, Time Of Use)
4. **Secure temp files** - Use secure random names
5. **Clean up sensitive data** - Securely delete files

## Resources

- [std::fs documentation](https://doc.rust-lang.org/std/fs/)
- [std::io documentation](https://doc.rust-lang.org/std/io/)
- [std::path documentation](https://doc.rust-lang.org/std/path/)
- [File I/O chapter in the Rust Book](https://doc.rust-lang.org/book/ch12-02-reading-a-file.html)
