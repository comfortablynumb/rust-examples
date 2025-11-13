# Process Management

Spawning and managing child processes, pipes, and inter-process communication.

## Concepts Covered

- Spawning processes with `Command`
- Capturing stdout and stderr
- Piping between processes
- Environment variables
- Exit codes
- Async process execution

## Process Creation

```rust
let output = Command::new("echo")
    .arg("hello")
    .output()?;

println!("Status: {}", output.status);
println!("Output: {}", String::from_utf8_lossy(&output.stdout));
```

## Pipes and Redirection

```rust
let mut child = Command::new("cat")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()?;

// Write to stdin
let stdin = child.stdin.as_mut().unwrap();
stdin.write_all(b"data")?;

// Read from stdout
let output = child.wait_with_output()?;
```

## Process Chaining

```rust
// echo "hello" | tr '[:lower:]' '[:upper:]'
let echo = Command::new("echo")
    .arg("hello")
    .stdout(Stdio::piped())
    .spawn()?;

let tr = Command::new("tr")
    .arg("[:lower:]")
    .arg("[:upper:]")
    .stdin(echo.stdout.unwrap())
    .output()?;
```

## Running

```bash
cargo run
```

## References

- [std::process](https://doc.rust-lang.org/std/process/)
- [tokio::process](https://docs.rs/tokio/latest/tokio/process/)
