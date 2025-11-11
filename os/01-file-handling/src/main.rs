#![allow(dead_code)]
#![allow(clippy::useless_vec)]
#![allow(clippy::redundant_pattern_matching)]

// File Handling in Rust
//
// Demonstrates file system operations including reading, writing,
// metadata, directories, and path handling.

use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    println!("=== File Handling Examples ===\n");

    // Example 1: Basic file writing and reading
    println!("1. Basic File Operations:");
    basic_file_operations()?;
    println!();

    // Example 2: Buffered I/O
    println!("2. Buffered I/O:");
    buffered_io()?;
    println!();

    // Example 3: File metadata
    println!("3. File Metadata:");
    file_metadata()?;
    println!();

    // Example 4: Directory operations
    println!("4. Directory Operations:");
    directory_operations()?;
    println!();

    // Example 5: Path handling
    println!("5. Path Handling:");
    path_handling();
    println!();

    // Example 6: File seeking
    println!("6. File Seeking:");
    file_seeking()?;
    println!();

    // Example 7: Temporary files
    println!("7. Working with Files:");
    working_with_files()?;
    println!();

    // Cleanup
    println!("8. Cleanup:");
    cleanup()?;

    Ok(())
}

// Example 1: Basic file reading and writing
fn basic_file_operations() -> io::Result<()> {
    // Write to a file (creates or truncates)
    let content = "Hello, Rust!\nThis is a test file.\n";
    fs::write("test.txt", content)?;
    println!("  ✓ Wrote to test.txt");

    // Read entire file to string
    let read_content = fs::read_to_string("test.txt")?;
    println!("  ✓ Read from test.txt:");
    println!("    {}", read_content.trim());

    // Read file as bytes
    let bytes = fs::read("test.txt")?;
    println!("  ✓ Read {} bytes", bytes.len());

    // Append to file
    let mut file = OpenOptions::new().append(true).open("test.txt")?;
    file.write_all(b"Appended line\n")?;
    println!("  ✓ Appended to test.txt");

    Ok(())
}

// Example 2: Buffered I/O for efficient reading/writing
fn buffered_io() -> io::Result<()> {
    // Buffered writing
    let file = File::create("buffered.txt")?;
    let mut writer = BufWriter::new(file);

    for i in 1..=5 {
        writeln!(writer, "Line {}", i)?;
    }
    writer.flush()?;
    println!("  ✓ Wrote buffered data");

    // Buffered reading (line by line)
    let file = File::open("buffered.txt")?;
    let reader = BufReader::new(file);

    println!("  ✓ Reading lines:");
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        println!("    {}. {}", i + 1, line);
    }

    Ok(())
}

// Example 3: File metadata and attributes
fn file_metadata() -> io::Result<()> {
    let metadata = fs::metadata("test.txt")?;

    println!("  File: test.txt");
    println!("    Size: {} bytes", metadata.len());
    println!("    Is file: {}", metadata.is_file());
    println!("    Is directory: {}", metadata.is_dir());
    println!("    Is symlink: {}", metadata.is_symlink());
    println!("    Read-only: {}", metadata.permissions().readonly());

    // Modified time
    if let Ok(modified) = metadata.modified() {
        println!("    Modified: {:?}", modified);
    }

    // Created time (not available on all platforms)
    if let Ok(created) = metadata.created() {
        println!("    Created: {:?}", created);
    }

    Ok(())
}

// Example 4: Directory operations
fn directory_operations() -> io::Result<()> {
    // Create directory
    fs::create_dir_all("test_dir/sub_dir")?;
    println!("  ✓ Created directory: test_dir/sub_dir");

    // Create files in directory
    fs::write("test_dir/file1.txt", "File 1")?;
    fs::write("test_dir/file2.txt", "File 2")?;
    fs::write("test_dir/sub_dir/file3.txt", "File 3")?;
    println!("  ✓ Created files in directory");

    // List directory contents
    println!("  ✓ Contents of test_dir:");
    for entry in fs::read_dir("test_dir")? {
        let entry = entry?;
        let file_type = if entry.file_type()?.is_dir() {
            "DIR"
        } else {
            "FILE"
        };
        println!("    [{}] {:?}", file_type, entry.file_name());
    }

    // Recursive directory traversal
    println!("  ✓ Recursive traversal:");
    visit_dirs(Path::new("test_dir"), 0)?;

    Ok(())
}

fn visit_dirs(dir: &Path, depth: usize) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            let indent = "  ".repeat(depth);
            println!(
                "    {}└─ {}",
                indent,
                path.file_name().unwrap().to_string_lossy()
            );

            if path.is_dir() {
                visit_dirs(&path, depth + 1)?;
            }
        }
    }
    Ok(())
}

// Example 5: Path handling
fn path_handling() {
    // Creating paths
    let path = Path::new("test_dir/sub_dir/file.txt");
    println!("  Full path: {:?}", path);

    // Path components
    if let Some(parent) = path.parent() {
        println!("  Parent: {:?}", parent);
    }

    if let Some(file_name) = path.file_name() {
        println!("  File name: {:?}", file_name);
    }

    if let Some(extension) = path.extension() {
        println!("  Extension: {:?}", extension);
    }

    if let Some(stem) = path.file_stem() {
        println!("  Stem: {:?}", stem);
    }

    // Building paths
    let mut path_buf = PathBuf::from("test_dir");
    path_buf.push("sub_dir");
    path_buf.push("new_file.txt");
    println!("  Built path: {:?}", path_buf);

    // Joining paths
    let joined = Path::new("test_dir").join("file.txt");
    println!("  Joined path: {:?}", joined);

    // Checking path properties
    println!("  Path exists: {}", Path::new("test.txt").exists());
    println!("  Is absolute: {}", path.is_absolute());
    println!("  Is relative: {}", path.is_relative());
}

// Example 6: File seeking (random access)
fn file_seeking() -> io::Result<()> {
    // Create a file with known content
    fs::write("seek_test.txt", "0123456789ABCDEFGHIJ")?;

    let mut file = File::open("seek_test.txt")?;
    let mut buffer = [0u8; 5];

    // Read from beginning
    file.read_exact(&mut buffer)?;
    println!("  From start: {}", String::from_utf8_lossy(&buffer));

    // Seek to position 10
    file.seek(SeekFrom::Start(10))?;
    file.read_exact(&mut buffer)?;
    println!("  From position 10: {}", String::from_utf8_lossy(&buffer));

    // Seek from end
    file.seek(SeekFrom::End(-5))?;
    file.read_exact(&mut buffer)?;
    println!("  Last 5 chars: {}", String::from_utf8_lossy(&buffer));

    // Seek relative to current position
    file.seek(SeekFrom::Start(0))?;
    file.seek(SeekFrom::Current(5))?;
    file.read_exact(&mut buffer)?;
    println!("  After seeking +5: {}", String::from_utf8_lossy(&buffer));

    Ok(())
}

// Example 7: Various file operations
fn working_with_files() -> io::Result<()> {
    // Copy file
    fs::copy("test.txt", "test_copy.txt")?;
    println!("  ✓ Copied test.txt to test_copy.txt");

    // Rename/move file
    fs::rename("test_copy.txt", "test_moved.txt")?;
    println!("  ✓ Renamed test_copy.txt to test_moved.txt");

    // Check if file exists
    if Path::new("test_moved.txt").exists() {
        println!("  ✓ test_moved.txt exists");
    }

    // Get file size without reading
    let size = fs::metadata("test_moved.txt")?.len();
    println!("  ✓ File size: {} bytes", size);

    // Create hard link (Unix)
    #[cfg(unix)]
    {
        if let Ok(_) = fs::hard_link("test.txt", "test_hardlink.txt") {
            println!("  ✓ Created hard link");
        }
    }

    // Create symbolic link (Unix)
    #[cfg(unix)]
    {
        if let Ok(_) = std::os::unix::fs::symlink("test.txt", "test_symlink.txt") {
            println!("  ✓ Created symbolic link");
        }
    }

    Ok(())
}

// Example 8: Cleanup
fn cleanup() -> io::Result<()> {
    // Remove files
    let files = vec![
        "test.txt",
        "buffered.txt",
        "seek_test.txt",
        "test_moved.txt",
    ];

    for file in files {
        if Path::new(file).exists() {
            fs::remove_file(file)?;
            println!("  ✓ Removed {}", file);
        }
    }

    // Remove directory recursively
    if Path::new("test_dir").exists() {
        fs::remove_dir_all("test_dir")?;
        println!("  ✓ Removed test_dir");
    }

    // Remove links if they exist
    #[cfg(unix)]
    {
        for link in &["test_hardlink.txt", "test_symlink.txt"] {
            if Path::new(link).exists() {
                fs::remove_file(link)?;
                println!("  ✓ Removed {}", link);
            }
        }
    }

    Ok(())
}
