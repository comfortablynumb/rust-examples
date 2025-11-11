#![allow(dead_code)]
#![allow(clippy::useless_vec)]

// Environment and Process Management in Rust
//
// Demonstrates working with environment variables, command-line arguments,
// current directory, and process management.

use std::env;
use std::io;
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    println!("=== Environment and Process Examples ===\n");

    // Example 1: Command-line arguments
    println!("1. Command-Line Arguments:");
    command_line_arguments();
    println!();

    // Example 2: Environment variables
    println!("2. Environment Variables:");
    environment_variables();
    println!();

    // Example 3: Current directory
    println!("3. Current Directory:");
    current_directory()?;
    println!();

    // Example 4: Process information
    println!("4. Process Information:");
    process_information();
    println!();

    // Example 5: Executing commands
    println!("5. Executing Commands:");
    executing_commands()?;
    println!();

    // Example 6: Process pipes and redirection
    println!("6. Process Pipes:");
    process_pipes()?;
    println!();

    // Example 7: Temporary variables
    println!("7. Temporary Environment:");
    temporary_environment()?;
    println!();

    Ok(())
}

// Example 1: Command-line arguments
fn command_line_arguments() {
    // Get all arguments (including program name)
    let args: Vec<String> = env::args().collect();
    println!("  Program name: {}", args[0]);
    println!("  Number of arguments: {}", args.len() - 1);

    if args.len() > 1 {
        println!("  Arguments:");
        for (i, arg) in args.iter().skip(1).enumerate() {
            println!("    {}: {}", i + 1, arg);
        }
    } else {
        println!("  No arguments provided");
        println!("  Try running: cargo run -- arg1 arg2 arg3");
    }

    // Parse arguments (simple example)
    let mut verbose = false;
    let mut output_file = None;
    let mut remaining = Vec::new();

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--verbose" | "-v" => verbose = true,
            arg if arg.starts_with("--output=") => {
                output_file = Some(arg[9..].to_string());
            }
            _ => remaining.push(arg.clone()),
        }
    }

    println!("\n  Parsed flags:");
    println!("    Verbose: {}", verbose);
    println!("    Output file: {:?}", output_file);
    println!("    Remaining args: {:?}", remaining);
}

// Example 2: Environment variables
fn environment_variables() {
    // Read specific environment variable
    match env::var("HOME") {
        Ok(val) => println!("  HOME = {}", val),
        Err(_) => println!("  HOME is not set"),
    }

    match env::var("PATH") {
        Ok(val) => {
            println!("  PATH entries:");
            for (i, path) in val.split(':').take(3).enumerate() {
                println!("    {}: {}", i + 1, path);
            }
            println!("    ... (and more)");
        }
        Err(_) => println!("  PATH is not set"),
    }

    // Check if variable exists
    println!("\n  Environment checks:");
    println!("    USER exists: {}", env::var("USER").is_ok());
    println!("    RUST_LOG exists: {}", env::var("RUST_LOG").is_ok());

    // Get with default value
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    println!("    LOG_LEVEL (with default): {}", log_level);

    // List all environment variables (first few)
    println!("\n  All environment variables (sample):");
    for (i, (key, value)) in env::vars().take(5).enumerate() {
        println!("    {}: {} = {}", i + 1, key, value);
    }
    println!("    ... ({} total)", env::vars().count());

    // Set environment variable (for child processes)
    env::set_var("MY_CUSTOM_VAR", "hello");
    println!("\n  Set MY_CUSTOM_VAR = hello");
    println!("    Verify: {}", env::var("MY_CUSTOM_VAR").unwrap());

    // Remove environment variable
    env::remove_var("MY_CUSTOM_VAR");
    println!("  Removed MY_CUSTOM_VAR");
    println!("    Exists: {}", env::var("MY_CUSTOM_VAR").is_ok());
}

// Example 3: Current directory operations
fn current_directory() -> io::Result<()> {
    // Get current directory
    let current_dir = env::current_dir()?;
    println!("  Current directory: {:?}", current_dir);

    // Get as string
    if let Some(dir_str) = current_dir.to_str() {
        println!("  As string: {}", dir_str);
    }

    // Parent directory
    if let Some(parent) = current_dir.parent() {
        println!("  Parent directory: {:?}", parent);
    }

    // Components
    println!("  Path components:");
    for (i, component) in current_dir.components().enumerate() {
        println!("    {}: {:?}", i, component);
    }

    // Change directory (commented out to avoid side effects)
    // env::set_current_dir("/tmp")?;

    Ok(())
}

// Example 4: Process information
fn process_information() {
    // Current executable
    match env::current_exe() {
        Ok(exe_path) => println!("  Executable: {:?}", exe_path),
        Err(e) => println!("  Failed to get executable path: {}", e),
    }

    // Temporary directory
    let temp_dir = env::temp_dir();
    println!("  Temp directory: {:?}", temp_dir);

    // OS type
    println!("  OS: {}", env::consts::OS);
    println!("  Family: {}", env::consts::FAMILY);
    println!("  Architecture: {}", env::consts::ARCH);
    println!("  DLL extension: {}", env::consts::DLL_EXTENSION);
    println!("  EXE extension: {}", env::consts::EXE_EXTENSION);

    // Number of CPUs (available threads)
    // Note: This is in std::thread, not std::env
    // Showing as related information
    println!("  Available parallelism: {:?}", std::thread::available_parallelism());
}

// Example 5: Executing commands
fn executing_commands() -> io::Result<()> {
    // Execute simple command
    println!("  Executing 'echo Hello, Rust!':");
    let output = Command::new("echo")
        .arg("Hello,")
        .arg("Rust!")
        .output()?;

    println!("    Exit code: {}", output.status);
    println!("    Output: {}", String::from_utf8_lossy(&output.stdout));

    // Execute command with status check
    println!("  Checking exit status:");
    let status = Command::new("true").status()?;

    if status.success() {
        println!("    Command succeeded!");
    } else {
        println!("    Command failed with: {}", status);
    }

    // Execute with environment variables
    println!("  With custom environment:");
    let output = Command::new("sh")
        .arg("-c")
        .arg("echo $MY_VAR")
        .env("MY_VAR", "custom_value")
        .output()?;

    println!("    Output: {}", String::from_utf8_lossy(&output.stdout));

    // Execute in different directory
    println!("  In different directory:");
    let output = Command::new("pwd")
        .current_dir("/tmp")
        .output()?;

    println!("    Working dir: {}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}

// Example 6: Process pipes and I/O redirection
fn process_pipes() -> io::Result<()> {
    // Pipe between commands (echo | grep)
    println!("  Piping commands:");

    let echo_child = Command::new("echo")
        .arg("Hello\nWorld\nRust")
        .stdout(Stdio::piped())
        .spawn()?;

    let echo_out = echo_child.stdout.expect("Failed to open echo stdout");

    let grep_output = Command::new("grep")
        .arg("Rust")
        .stdin(Stdio::from(echo_out))
        .output()?;

    println!("    Result: {}", String::from_utf8_lossy(&grep_output.stdout));

    // Capture stderr separately
    println!("  Capturing stderr:");
    let output = Command::new("ls")
        .arg("/nonexistent")
        .stderr(Stdio::piped())
        .output()?;

    if !output.status.success() {
        println!("    Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Spawn without waiting
    println!("  Spawning background process:");
    let mut child = Command::new("sleep")
        .arg("1")
        .spawn()?;

    println!("    Process spawned with PID: {}", child.id());
    println!("    Waiting for completion...");

    let status = child.wait()?;
    println!("    Process exited with: {}", status);

    Ok(())
}

// Example 7: Working with environment temporarily
fn temporary_environment() -> io::Result<()> {
    println!("  Creating temporary environment for command:");

    // Clear all environment and set specific vars
    let output = Command::new("env")
        .env_clear()
        .env("CUSTOM_VAR", "value1")
        .env("ANOTHER_VAR", "value2")
        .output()?;

    println!("    Environment variables in child process:");
    let env_output = String::from_utf8_lossy(&output.stdout);
    for line in env_output.lines().take(3) {
        println!("      {}", line);
    }

    // Inherit parent environment and add more
    env::set_var("PARENT_VAR", "from_parent");

    let output = Command::new("sh")
        .arg("-c")
        .arg("echo PARENT=$PARENT_VAR CHILD=$CHILD_VAR")
        .env("CHILD_VAR", "from_child")
        .output()?;

    println!("\n    With inheritance:");
    println!("      {}", String::from_utf8_lossy(&output.stdout));

    env::remove_var("PARENT_VAR");

    Ok(())
}
