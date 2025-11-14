# Unix Permissions

Working with file and directory permissions on Unix-like systems.

## Permission Bits

```
Owner  Group  Others
rwx    rwx    rwx
421    421    421
```

- **r (4)**: Read
- **w (2)**: Write
- **x (1)**: Execute

## Common Modes

- **0644**: `-rw-r--r--` (readable file)
- **0755**: `-rwxr-xr-x` (executable file)
- **0700**: `-rwx------` (private file)
- **0777**: `-rwxrwxrwx` (all permissions)

## Usage

### Read Permissions

```rust
let metadata = fs::metadata("file.txt")?;
let mode = metadata.permissions().mode();
println!("Mode: {:o}", mode);
```

### Set Permissions

```rust
let perms = Permissions::from_mode(0o644);
fs::set_permissions("file.txt", perms)?;
```

### Check Specific Permission

```rust
let mode = metadata.permissions().mode();
let owner_read = (mode & 0o400) != 0;
let owner_write = (mode & 0o200) != 0;
```

## Running

```bash
cargo run
```

## Use Cases

- Securing sensitive files
- Making scripts executable
- Setting directory access control
- Implementing file security policies

## Note

This example uses Unix-specific APIs. On Windows, use different permission models.

## References

- [std::fs::Permissions](https://doc.rust-lang.org/std/fs/struct.Permissions.html)
- [chmod(2) man page](https://man7.org/linux/man-pages/man2/chmod.2.html)
