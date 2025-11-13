use std::fs::{self, File, Permissions};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Unix Permissions Examples\n");

    // Create test file
    let test_file = "test_permissions.txt";
    let mut file = File::create(test_file)?;
    file.write_all(b"Test content")?;
    drop(file);

    // Read current permissions
    read_permissions(test_file)?;

    // Set permissions
    set_permissions(test_file)?;

    // Check if readable/writable/executable
    check_permissions(test_file)?;

    // Create directory with permissions
    directory_permissions()?;

    // Cleanup
    fs::remove_file(test_file)?;
    fs::remove_dir("test_dir")?;

    println!("\nAll tests completed!");

    Ok(())
}

fn read_permissions(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Reading Permissions ===");

    let metadata = fs::metadata(path)?;
    let permissions = metadata.permissions();

    let mode = permissions.mode();
    println!("File: {}", path);
    println!("Octal mode: {:o}", mode);
    println!("Symbolic: {}", format_permissions(mode));
    println!();

    Ok(())
}

fn set_permissions(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Setting Permissions ===");

    // Set to rwxr-xr-x (0755)
    let perms = Permissions::from_mode(0o755);
    fs::set_permissions(path, perms)?;

    println!("Set permissions to 0755");
    read_permissions(path)?;

    // Set to rw-r--r-- (0644)
    let perms = Permissions::from_mode(0o644);
    fs::set_permissions(path, perms)?;

    println!("Set permissions to 0644");
    read_permissions(path)?;

    Ok(())
}

fn check_permissions(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Checking Permissions ===");

    let metadata = fs::metadata(path)?;
    let mode = metadata.permissions().mode();

    // Owner permissions
    let owner_read = (mode & 0o400) != 0;
    let owner_write = (mode & 0o200) != 0;
    let owner_exec = (mode & 0o100) != 0;

    // Group permissions
    let group_read = (mode & 0o040) != 0;
    let group_write = (mode & 0o020) != 0;
    let group_exec = (mode & 0o010) != 0;

    // Other permissions
    let other_read = (mode & 0o004) != 0;
    let other_write = (mode & 0o002) != 0;
    let other_exec = (mode & 0o001) != 0;

    println!("Owner:  read={} write={} execute={}", owner_read, owner_write, owner_exec);
    println!("Group:  read={} write={} execute={}", group_read, group_write, group_exec);
    println!("Others: read={} write={} execute={}", other_read, other_write, other_exec);
    println!();

    Ok(())
}

fn directory_permissions() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Directory Permissions ===");

    // Create directory with specific permissions
    fs::create_dir("test_dir")?;

    // Set directory permissions to rwxr-xr-x (0755)
    let perms = Permissions::from_mode(0o755);
    fs::set_permissions("test_dir", perms)?;

    println!("Created directory with 0755 permissions");
    read_permissions("test_dir")?;

    Ok(())
}

fn format_permissions(mode: u32) -> String {
    let user = format!(
        "{}{}{}",
        if mode & 0o400 != 0 { "r" } else { "-" },
        if mode & 0o200 != 0 { "w" } else { "-" },
        if mode & 0o100 != 0 { "x" } else { "-" }
    );

    let group = format!(
        "{}{}{}",
        if mode & 0o040 != 0 { "r" } else { "-" },
        if mode & 0o020 != 0 { "w" } else { "-" },
        if mode & 0o010 != 0 { "x" } else { "-" }
    );

    let other = format!(
        "{}{}{}",
        if mode & 0o004 != 0 { "r" } else { "-" },
        if mode & 0o002 != 0 { "w" } else { "-" },
        if mode & 0o001 != 0 { "x" } else { "-" }
    );

    format!("{}{}{}", user, group, other)
}
