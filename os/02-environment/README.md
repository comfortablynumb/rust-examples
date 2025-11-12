# Environment and Process Management

This example demonstrates working with environment variables, command-line arguments, and process management in Rust using the standard library.

## Concepts Covered

### Command-Line Arguments
- Reading program arguments with `env::args()`
- Parsing flags and options
- Simple argument processing
- Best practices for CLI argument parsing

### Environment Variables
- Reading environment variables with `env::var()`
- Setting environment variables with `env::set_var()`
- Removing environment variables
- Listing all environment variables
- Providing default values

### Current Directory
- Getting current working directory
- Path manipulation
- Directory components
- Parent directory access

### Process Information
- Current executable path
- Temporary directory location
- OS and architecture constants
- System information

### Executing Commands
- Running external commands with `Command`
- Capturing output (stdout/stderr)
- Setting environment for child processes
- Changing working directory for commands
- Exit status checking

### Process Pipes and I/O
- Piping between processes
- Redirecting stdout/stderr
- Spawning background processes
- Process synchronization
- Capturing child process output

### Temporary Environment
- Creating isolated environments for commands
- Clearing parent environment
- Setting specific variables
- Environment inheritance

## Running the Example

```bash
cargo run
```

Run with command-line arguments:
```bash
cargo run -- --verbose --output=results.txt arg1 arg2
```

## Key Takeaways

1. **Safe defaults**: Use `unwrap_or_else()` for environment variables
2. **Child processes**: `env::set_var()` only affects child processes
3. **Cross-platform**: Use `env::consts` for platform detection
4. **Error handling**: All file/process operations return `Result`
5. **Isolation**: Child processes can have custom environments

## Common Patterns

### Read command-line arguments
```rust
use std::env;

let args: Vec<String> = env::args().collect();
println!("Program: {}", args[0]);

for arg in args.iter().skip(1) {
    println!("Argument: {}", arg);
}
```

### Read environment variable with default
```rust
use std::env;

let log_level = env::var("LOG_LEVEL")
    .unwrap_or_else(|_| "info".to_string());
```

### Check if environment variable exists
```rust
use std::env;

if env::var("DEBUG").is_ok() {
    println!("Debug mode enabled");
}
```

### Set environment variable
```rust
use std::env;

// Only affects child processes
env::set_var("MY_VAR", "value");
```

### Execute command and capture output
```rust
use std::process::Command;

let output = Command::new("ls")
    .arg("-la")
    .output()?;

println!("Output: {}", String::from_utf8_lossy(&output.stdout));
println!("Exit code: {}", output.status);
```

### Execute command with custom environment
```rust
use std::process::Command;

let output = Command::new("printenv")
    .env("CUSTOM_VAR", "custom_value")
    .output()?;
```

### Execute command in different directory
```rust
use std::process::Command;

let output = Command::new("pwd")
    .current_dir("/tmp")
    .output()?;
```

### Pipe between processes
```rust
use std::process::{Command, Stdio};

let echo_child = Command::new("echo")
    .arg("Hello World")
    .stdout(Stdio::piped())
    .spawn()?;

let echo_out = echo_child.stdout.unwrap();

let grep_output = Command::new("grep")
    .arg("World")
    .stdin(Stdio::from(echo_out))
    .output()?;
```

### Spawn background process
```rust
use std::process::Command;

let mut child = Command::new("long-running-task")
    .spawn()?;

println!("Process ID: {}", child.id());

// Do other work...

// Wait for completion
let status = child.wait()?;
```

### Clear environment for child process
```rust
use std::process::Command;

let output = Command::new("env")
    .env_clear()
    .env("ONLY_VAR", "value")
    .output()?;
```

## Best Practices

1. **Always handle errors** - Process operations can fail
2. **Use `Path`/`PathBuf`** - For cross-platform path handling
3. **Validate input** - Don't trust command-line arguments
4. **Use `env::consts`** - For platform-specific code
5. **Capture stderr** - Don't ignore error output
6. **Check exit status** - Use `status.success()`
7. **Use libraries** - Consider `clap` for argument parsing
8. **Avoid shell injection** - Don't pass untrusted input to shell

## Argument Parsing Libraries

For production code, consider these libraries:

### clap
```rust
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    verbose: bool,

    #[arg(short, long)]
    output: Option<String>,
}

let args = Args::parse();
```

### structopt (older, still used)
```rust
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long)]
    verbose: bool,
}
```

## Platform-Specific Features

### Unix-specific
```rust
#[cfg(unix)]
use std::os::unix::process::CommandExt;

#[cfg(unix)]
let mut cmd = Command::new("program");
cmd.uid(1000);  // Set user ID
cmd.gid(1000);  // Set group ID
```

### Windows-specific
```rust
#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
let mut cmd = Command::new("program");
cmd.creation_flags(0x08000000);  // CREATE_NO_WINDOW
```

## Security Considerations

1. **Validate paths** - Prevent directory traversal
2. **Sanitize input** - Never pass untrusted input to shell
3. **Avoid `sh -c`** - Use direct command execution
4. **Limit environment** - Don't expose sensitive variables
5. **Check permissions** - Verify executable permissions
6. **Use absolute paths** - Avoid PATH injection
7. **Timeout long operations** - Prevent DoS

## Performance Tips

1. **Reuse `Command`** - Create once, modify as needed
2. **Use `spawn()` vs `output()`** - For background processes
3. **Buffer I/O** - Use `BufReader`/`BufWriter`
4. **Async I/O** - Use `tokio::process` for async
5. **Limit environment** - Smaller environment = faster spawn

## Common Errors

- `NotFound`: Command not found in PATH
- `PermissionDenied`: Insufficient permissions to execute
- `InvalidInput`: Invalid command or arguments
- `BrokenPipe`: Process terminated unexpectedly

## Resources

- [std::env documentation](https://doc.rust-lang.org/std/env/)
- [std::process documentation](https://doc.rust-lang.org/std/process/)
- [clap documentation](https://docs.rs/clap/)
- [Command Line Apps in Rust](https://rust-cli.github.io/book/)
