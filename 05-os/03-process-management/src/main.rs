use std::process::{Command, Stdio};
use std::io::Write;
use tokio::process::Command as TokioCommand;

fn main() {
    println!("Process Management Examples\n");

    // Basic process spawning
    basic_spawn();

    // Process with output capture
    capture_output();

    // Process with input/output pipes
    pipe_communication();

    // Process with environment variables
    with_environment();

    // Parallel process execution
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        parallel_processes().await;
    });

    // Process chaining (pipe between processes)
    process_pipeline();
}

fn basic_spawn() {
    println!("=== Basic Process Spawning ===");

    let output = Command::new("echo")
        .arg("Hello from spawned process!")
        .output()
        .expect("Failed to execute process");

    println!("Status: {}", output.status);
    println!("Output: {}", String::from_utf8_lossy(&output.stdout));

    // Check exit code
    if output.status.success() {
        println!("✓ Process completed successfully\n");
    }
}

fn capture_output() {
    println!("=== Capturing Process Output ===");

    // Run a command and capture both stdout and stderr
    let output = Command::new("ls")
        .arg("-la")
        .output()
        .expect("Failed to execute ls");

    println!("stdout:");
    println!("{}", String::from_utf8_lossy(&output.stdout));

    if !output.stderr.is_empty() {
        println!("stderr:");
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}

fn pipe_communication() {
    println!("=== Pipe Communication ===");

    // Spawn process with stdin/stdout pipes
    let mut child = Command::new("cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn cat");

    // Write to child's stdin
    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(b"Hello from parent process!\n").expect("Failed to write to stdin");
        stdin.write_all(b"This is line 2\n").expect("Failed to write");
    } // stdin is closed when it goes out of scope

    // Read from child's stdout
    let output = child.wait_with_output().expect("Failed to wait on child");
    println!("Cat echoed:");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn with_environment() {
    println!("=== Process with Environment Variables ===");

    let output = Command::new("sh")
        .arg("-c")
        .arg("echo $CUSTOM_VAR")
        .env("CUSTOM_VAR", "Hello from environment!")
        .output()
        .expect("Failed to execute");

    println!("Output: {}", String::from_utf8_lossy(&output.stdout));
}

async fn parallel_processes() {
    println!("=== Parallel Process Execution ===");

    let commands = ["echo 'Process 1'", "echo 'Process 2'", "echo 'Process 3'"];

    let mut handles = vec![];

    for (i, cmd) in commands.iter().enumerate() {
        let cmd = cmd.to_string();
        let handle = tokio::spawn(async move {
            let output = TokioCommand::new("sh")
                .arg("-c")
                .arg(&cmd)
                .output()
                .await
                .expect("Failed to execute");

            println!("Task {}: {}", i + 1, String::from_utf8_lossy(&output.stdout).trim());
        });
        handles.push(handle);
    }

    // Wait for all to complete
    for handle in handles {
        handle.await.expect("Task failed");
    }

    println!();
}

fn process_pipeline() {
    println!("=== Process Pipeline (Chaining) ===");

    // Equivalent to: echo "hello world" | tr '[:lower:]' '[:upper:]'

    // First process: echo
    let mut echo = Command::new("echo")
        .arg("hello world")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start echo");

    // Second process: tr (translate), with input from first process
    let mut tr = Command::new("tr")
        .arg("[:lower:]")
        .arg("[:upper:]")
        .stdin(echo.stdout.take().unwrap()) // Pipe echo's stdout to tr's stdin
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start tr");

    // Wait on both processes to avoid zombies
    let _ = echo.wait().expect("Failed to wait on echo");
    let output = tr.wait_with_output().expect("Failed to wait on tr");
    println!("Pipeline result: {}", String::from_utf8_lossy(&output.stdout));
}
