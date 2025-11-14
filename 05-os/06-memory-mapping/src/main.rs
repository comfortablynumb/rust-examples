use memmap2::{Mmap, MmapMut, MmapOptions};
use std::fs::{File, OpenOptions};
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Memory Mapping (mmap) Examples\n");

    // Example 1: Read-only memory map
    readonly_mmap()?;

    // Example 2: Writable memory map
    writable_mmap()?;

    // Example 3: Anonymous memory map (shared memory)
    anonymous_mmap()?;

    // Cleanup
    std::fs::remove_file("test_readonly.txt").ok();
    std::fs::remove_file("test_writable.txt").ok();

    Ok(())
}

fn readonly_mmap() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Read-Only Memory Map ===");

    // Create test file
    let mut file = File::create("test_readonly.txt")?;
    file.write_all(b"Hello, memory-mapped file!")?;
    drop(file);

    // Open for reading
    let file = File::open("test_readonly.txt")?;

    // Create memory map
    let mmap = unsafe { Mmap::map(&file)? };

    // Access data like a slice
    println!("File size: {} bytes", mmap.len());
    println!("Content: {}", String::from_utf8_lossy(&mmap[..]));

    // Memory-mapped data is automatically unmapped when dropped
    println!("✓ Read-only mmap completed\n");

    Ok(())
}

fn writable_mmap() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Writable Memory Map ===");

    // Create test file with some content
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("test_writable.txt")?;

    // Set file size
    file.set_len(100)?;

    // Create mutable memory map
    let mut mmap = unsafe { MmapMut::map_mut(&file)? };

    println!("Initial size: {} bytes", mmap.len());

    // Write data
    let message = b"Modified via memory map!";
    mmap[0..message.len()].copy_from_slice(message);

    // Flush changes to disk
    mmap.flush()?;

    println!("✓ Written: {}", String::from_utf8_lossy(message));

    // Read back
    let file = File::open("test_writable.txt")?;
    let mmap = unsafe { Mmap::map(&file)? };
    let content = std::str::from_utf8(&mmap[0..message.len()])?;
    println!("✓ Read back: {}", content);
    println!();

    Ok(())
}

fn anonymous_mmap() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Anonymous Memory Map ===");

    // Create anonymous memory map (not backed by file)
    let mut mmap = MmapOptions::new()
        .len(4096)
        .map_anon()?;

    println!("Anonymous map size: {} bytes", mmap.len());

    // Write data
    let data = b"In-memory data, no file!";
    mmap[0..data.len()].copy_from_slice(data);

    // Read data
    let content = std::str::from_utf8(&mmap[0..data.len()])?;
    println!("Content: {}", content);

    println!("✓ Anonymous mmap completed\n");

    Ok(())
}
