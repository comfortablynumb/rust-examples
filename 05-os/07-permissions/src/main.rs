use std::fs::{self, File};
use std::io::Write;

#[cfg(unix)]
use std::fs::Permissions;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(unix)]
    {
        println!("Unix Permissions Examples\n");
        unix_main()
    }

    #[cfg(windows)]
    {
        println!("Windows File Attributes Examples\n");
        windows_main()
    }
}

#[cfg(unix)]
fn unix_main() -> Result<(), Box<dyn std::error::Error>> {
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

#[cfg(windows)]
fn windows_main() -> Result<(), Box<dyn std::error::Error>> {
    // Create test file
    let test_file = "test_permissions.txt";
    let mut file = File::create(test_file)?;
    file.write_all(b"Test content")?;
    drop(file);

    // Read current permissions (read-only status)
    println!("=== Reading File Attributes ===");
    let metadata = fs::metadata(test_file)?;
    let permissions = metadata.permissions();
    println!("File: {}", test_file);
    println!("Read-only: {}", permissions.readonly());
    println!();

    // Set to read-only
    println!("=== Setting Read-Only ===");
    let mut permissions = fs::metadata(test_file)?.permissions();
    permissions.set_readonly(true);
    fs::set_permissions(test_file, permissions)?;
    println!("Set file to read-only");

    let metadata = fs::metadata(test_file)?;
    println!("Read-only: {}", metadata.permissions().readonly());
    println!();

    // Set to writable
    println!("=== Setting Writable ===");
    let mut permissions = fs::metadata(test_file)?.permissions();
    #[allow(clippy::permissions_set_readonly_false)]
    permissions.set_readonly(false);
    fs::set_permissions(test_file, permissions)?;
    println!("Set file to writable");

    let metadata = fs::metadata(test_file)?;
    println!("Read-only: {}", metadata.permissions().readonly());
    println!();

    // Cleanup
    fs::remove_file(test_file)?;

    println!(
        "\nNote: On Windows, file permissions are managed through ACLs (Access Control Lists)."
    );
    println!("For advanced permissions, use the winapi crate or std::os::windows extensions.");
    println!("\nAll tests completed!");

    Ok(())
}

#[cfg(unix)]
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

#[cfg(unix)]
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

#[cfg(unix)]
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

    println!(
        "Owner:  read={} write={} execute={}",
        owner_read, owner_write, owner_exec
    );
    println!(
        "Group:  read={} write={} execute={}",
        group_read, group_write, group_exec
    );
    println!(
        "Others: read={} write={} execute={}",
        other_read, other_write, other_exec
    );
    println!();

    Ok(())
}

#[cfg(unix)]
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

#[cfg(unix)]
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
