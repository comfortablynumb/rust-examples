# Memory Mapping (mmap)

Memory-map files for efficient I/O and shared memory.

## Concepts

- **mmap**: Map files into process memory
- **Shared Memory**: Inter-process communication
- **Zero-copy**: Avoid buffer copying
- **Lazy Loading**: Pages loaded on demand

## Types

### Read-Only Map

```rust
let file = File::open("data.bin")?;
let mmap = unsafe { Mmap::map(&file)? };

// Access like a slice
let data: &[u8] = &mmap[..];
```

### Writable Map

```rust
let file = OpenOptions::new()
    .read(true)
    .write(true)
    .open("data.bin")?;

let mut mmap = unsafe { MmapMut::map_mut(&file)? };
mmap[0] = 42;
mmap.flush()?;
```

### Anonymous Map

```rust
let mut mmap = MmapOptions::new()
    .len(4096)
    .map_anon()?;

// In-memory, not backed by file
```

## Advantages

- **Performance**: No read/write system calls
- **Lazy Loading**: Load on access
- **Shared Memory**: Multiple processes can share
- **Large Files**: Work with files larger than RAM

## Use Cases

- Reading large files efficiently
- Shared memory between processes
- Database implementations
- Memory-efficient data processing
- Zero-copy I/O

## Safety

Memory mapping is `unsafe` because:
- File can change externally
- Multiple processes can access simultaneously
- Potential for data races

## Running

```bash
cargo run
```

## References

- [memmap2 Documentation](https://docs.rs/memmap2/)
- [mmap(2) man page](https://man7.org/linux/man-pages/man2/mmap.2.html)
