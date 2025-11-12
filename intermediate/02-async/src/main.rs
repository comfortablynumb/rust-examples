#![allow(dead_code)]
#![allow(unused_variables)]

//! Comprehensive Async/Await Examples in Rust
//!
//! This file demonstrates asynchronous programming patterns using Rust's async/await
//! syntax with the Tokio runtime. Topics covered:
//! 1. Basic async/await syntax
//! 2. Async functions and blocks
//! 3. Futures and .await
//! 4. Tokio runtime usage
//! 5. Spawning async tasks
//! 6. join! and select! macros
//! 7. Async channels (mpsc)
//! 8. Timeouts and delays
//! 9. Async file I/O
//! 10. Error handling in async context

use std::time::Duration;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use tokio::time::{sleep, timeout};

// ============================================================================
// 1. BASIC ASYNC/AWAIT SYNTAX
// ============================================================================

/// Basic async function that simulates a delayed computation
///
/// Async functions return a Future that must be .await-ed to execute.
/// The async keyword transforms the function into a state machine.
async fn simple_async_function() -> String {
    // Simulate some work with a delay
    sleep(Duration::from_millis(100)).await;
    "Hello from async!".to_string()
}

/// Async function with parameters
///
/// Just like regular functions, async functions can take parameters
/// and use them in async operations.
async fn async_with_params(name: &str, delay_ms: u64) -> String {
    sleep(Duration::from_millis(delay_ms)).await;
    format!("Hello, {}!", name)
}

/// Async function that returns a Result
///
/// Error handling in async functions works the same as sync functions.
/// You can use ? operator to propagate errors.
async fn async_with_result(succeed: bool) -> Result<String, String> {
    sleep(Duration::from_millis(50)).await;
    if succeed {
        Ok("Success!".to_string())
    } else {
        Err("Failed!".to_string())
    }
}

// ============================================================================
// 2. ASYNC FUNCTIONS AND BLOCKS
// ============================================================================

/// Demonstrates async blocks within a regular function
///
/// Async blocks create anonymous async functions inline.
/// They capture variables from their environment.
fn function_with_async_blocks() {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    runtime.block_on(async {
        // Async block can contain multiple await expressions
        let result1 = simple_async_function().await;
        println!("Result 1: {}", result1);

        // Another async block can be nested
        let result2 = async {
            sleep(Duration::from_millis(50)).await;
            "Nested async block"
        }
        .await;
        println!("Result 2: {}", result2);
    });
}

/// Async function that uses async blocks for parallel operations
async fn async_blocks_parallel() {
    // Create async blocks that will run in parallel
    let task1 = async {
        sleep(Duration::from_millis(100)).await;
        println!("Task 1 complete");
        1
    };

    let task2 = async {
        sleep(Duration::from_millis(50)).await;
        println!("Task 2 complete");
        2
    };

    // Use tokio::join! to run them concurrently
    let (r1, r2) = tokio::join!(task1, task2);
    println!("Results: {} and {}", r1, r2);
}

// ============================================================================
// 3. FUTURES AND .AWAIT
// ============================================================================

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Custom Future implementation
///
/// This shows the low-level Future trait that async/await builds upon.
/// Most of the time you'll use async/await instead of implementing this directly.
struct DelayedValue {
    value: i32,
    ready: bool,
}

impl Future for DelayedValue {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.ready {
            Poll::Ready(self.value)
        } else {
            self.ready = true;
            // In a real implementation, you'd register a waker here
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

/// Function that returns a custom future
fn get_delayed_value(value: i32) -> impl Future<Output = i32> {
    DelayedValue {
        value,
        ready: false,
    }
}

/// Demonstrates working with futures
async fn working_with_futures() {
    // Using our custom future
    let value = get_delayed_value(42).await;
    println!("Got delayed value: {}", value);

    // Async functions return futures that can be stored
    let future = simple_async_function();
    // Future doesn't execute until awaited
    let result = future.await;
    println!("Future result: {}", result);
}

// ============================================================================
// 4. TOKIO RUNTIME
// ============================================================================

/// Demonstrates different ways to use the Tokio runtime
///
/// Tokio is an async runtime that provides:
/// - Task scheduler
/// - I/O event loop
/// - Timers
/// - Async I/O primitives
fn tokio_runtime_examples() {
    // Method 1: Create runtime manually
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        println!("Running in Tokio runtime");
        sleep(Duration::from_millis(100)).await;
    });

    // Method 2: Multi-threaded runtime (default)
    let multi_thread_rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();

    multi_thread_rt.block_on(async {
        println!("Running in multi-threaded runtime");
    });

    // Method 3: Current thread runtime (single-threaded)
    let current_thread_rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    current_thread_rt.block_on(async {
        println!("Running in current-thread runtime");
    });
}

// ============================================================================
// 5. SPAWNING ASYNC TASKS
// ============================================================================

/// Demonstrates task spawning with tokio::spawn
///
/// tokio::spawn creates a new task that runs concurrently.
/// It returns a JoinHandle that can be awaited for the result.
async fn spawning_tasks() {
    println!("\n=== Spawning Tasks ===");

    // Spawn a single task
    let handle = tokio::spawn(async {
        sleep(Duration::from_millis(100)).await;
        println!("Spawned task completed");
        42
    });

    // Continue doing other work
    println!("Main task continues...");
    sleep(Duration::from_millis(50)).await;

    // Wait for spawned task to complete
    let result = handle.await.unwrap();
    println!("Spawned task returned: {}", result);

    // Spawn multiple tasks
    let mut handles = vec![];
    for i in 0..5 {
        let handle = tokio::spawn(async move {
            sleep(Duration::from_millis(100 - i * 10)).await;
            println!("Task {} completed", i);
            i
        });
        handles.push(handle);
    }

    // Wait for all tasks
    for handle in handles {
        let result = handle.await.unwrap();
        println!("Task result: {}", result);
    }
}

/// Demonstrates task cancellation
async fn task_cancellation() {
    println!("\n=== Task Cancellation ===");

    let handle = tokio::spawn(async {
        loop {
            println!("Long running task...");
            sleep(Duration::from_millis(100)).await;
        }
    });

    // Let it run for a bit
    sleep(Duration::from_millis(350)).await;

    // Cancel the task by dropping the handle or calling abort
    handle.abort();
    println!("Task aborted");
}

// ============================================================================
// 6. JOIN! AND SELECT! MACROS
// ============================================================================

/// Demonstrates tokio::join! for concurrent execution
///
/// join! waits for ALL futures to complete
async fn join_macro_example() {
    println!("\n=== Join Macro ===");

    async fn task_a() -> i32 {
        sleep(Duration::from_millis(200)).await;
        println!("Task A completed");
        1
    }

    async fn task_b() -> i32 {
        sleep(Duration::from_millis(100)).await;
        println!("Task B completed");
        2
    }

    async fn task_c() -> i32 {
        sleep(Duration::from_millis(150)).await;
        println!("Task C completed");
        3
    }

    // All tasks run concurrently, join! waits for all
    let (a, b, c) = tokio::join!(task_a(), task_b(), task_c());
    println!("All tasks completed: {}, {}, {}", a, b, c);
}

/// Demonstrates tokio::select! for racing futures
///
/// select! waits for the FIRST future to complete
async fn select_macro_example() {
    println!("\n=== Select Macro ===");

    let mut count = 0;

    loop {
        tokio::select! {
            // First branch: simulates receiving a message
            _ = sleep(Duration::from_millis(100)) => {
                count += 1;
                println!("Timer fired, count: {}", count);
                if count >= 3 {
                    break;
                }
            }
            // Second branch: simulates a cancel signal
            _ = sleep(Duration::from_millis(500)) => {
                println!("Timeout occurred");
                break;
            }
        }
    }
}

/// Advanced select! with biased selection
async fn select_biased_example() {
    println!("\n=== Biased Select ===");

    let mut interval = tokio::time::interval(Duration::from_millis(100));
    let mut counter = 0;

    loop {
        tokio::select! {
            biased;
            // This branch is checked first (biased)
            _ = interval.tick() => {
                counter += 1;
                println!("Tick: {}", counter);
                if counter >= 5 {
                    break;
                }
            }
        }
    }
}

// ============================================================================
// 7. ASYNC CHANNELS (MPSC)
// ============================================================================

/// Demonstrates multi-producer, single-consumer channels
///
/// Channels allow async tasks to communicate by sending messages.
async fn mpsc_channel_example() {
    println!("\n=== MPSC Channel ===");

    // Create a channel with buffer size of 32
    let (tx, mut rx) = mpsc::channel::<String>(32);

    // Spawn producer tasks
    for i in 0..3 {
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            for j in 0..3 {
                let msg = format!("Message {}-{}", i, j);
                tx_clone.send(msg).await.unwrap();
                sleep(Duration::from_millis(50)).await;
            }
            println!("Producer {} finished", i);
        });
    }

    // Drop original sender so channel closes when all senders are done
    drop(tx);

    // Receive all messages
    while let Some(msg) = rx.recv().await {
        println!("Received: {}", msg);
    }

    println!("All messages received");
}

/// Demonstrates bounded vs unbounded channels
async fn bounded_unbounded_channels() {
    println!("\n=== Bounded vs Unbounded Channels ===");

    // Bounded channel (backpressure when full)
    let (tx_bounded, mut rx_bounded) = mpsc::channel::<i32>(2);

    tokio::spawn(async move {
        for i in 0..5 {
            println!("Sending bounded: {}", i);
            tx_bounded.send(i).await.unwrap();
            println!("Sent bounded: {}", i);
        }
    });

    sleep(Duration::from_millis(100)).await;

    while let Some(val) = rx_bounded.recv().await {
        println!("Received bounded: {}", val);
        sleep(Duration::from_millis(50)).await;
    }

    // Unbounded channel (no backpressure, uses more memory)
    let (tx_unbounded, mut rx_unbounded) = mpsc::unbounded_channel::<i32>();

    tokio::spawn(async move {
        for i in 0..5 {
            println!("Sending unbounded: {}", i);
            tx_unbounded.send(i).unwrap();
            println!("Sent unbounded: {}", i);
        }
    });

    sleep(Duration::from_millis(100)).await;

    while let Some(val) = rx_unbounded.recv().await {
        println!("Received unbounded: {}", val);
        sleep(Duration::from_millis(50)).await;
    }
}

// ============================================================================
// 8. TIMEOUTS AND DELAYS
// ============================================================================

/// Demonstrates various time-related operations
async fn timeout_examples() {
    println!("\n=== Timeouts and Delays ===");

    // Simple delay
    println!("Starting delay...");
    sleep(Duration::from_millis(100)).await;
    println!("Delay finished");

    // Timeout a slow operation
    let slow_operation = async {
        sleep(Duration::from_secs(2)).await;
        "Done"
    };

    match timeout(Duration::from_millis(100), slow_operation).await {
        Ok(result) => println!("Operation completed: {}", result),
        Err(_) => println!("Operation timed out!"),
    }

    // Timeout a fast operation
    let fast_operation = async {
        sleep(Duration::from_millis(50)).await;
        "Quick!"
    };

    match timeout(Duration::from_millis(200), fast_operation).await {
        Ok(result) => println!("Operation completed: {}", result),
        Err(_) => println!("Operation timed out!"),
    }
}

/// Demonstrates interval timers
async fn interval_example() {
    println!("\n=== Interval Timer ===");

    let mut interval = tokio::time::interval(Duration::from_millis(100));
    let mut count = 0;

    loop {
        interval.tick().await;
        count += 1;
        println!("Tick {}", count);

        if count >= 5 {
            break;
        }
    }
}

// ============================================================================
// 9. ASYNC FILE I/O
// ============================================================================

/// Demonstrates async file operations
async fn async_file_io_example() -> std::io::Result<()> {
    println!("\n=== Async File I/O ===");

    let file_path = "/tmp/async_test.txt";

    // Write to file asynchronously
    let mut file = fs::File::create(file_path).await?;
    file.write_all(b"Hello, async world!\n").await?;
    file.write_all(b"This is async file I/O.\n").await?;
    println!("File written");

    // Read from file asynchronously
    let mut file = fs::File::open(file_path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    println!("File contents:\n{}", contents);

    // Read entire file at once
    let contents = fs::read_to_string(file_path).await?;
    println!("File read (whole): {}", contents.len());

    // Clean up
    fs::remove_file(file_path).await?;
    println!("File removed");

    Ok(())
}

/// Demonstrates concurrent file operations
async fn concurrent_file_operations() -> std::io::Result<()> {
    println!("\n=== Concurrent File Operations ===");

    // Create multiple files concurrently
    let mut handles = vec![];

    for i in 0..3 {
        let handle = tokio::spawn(async move {
            let path = format!("/tmp/async_file_{}.txt", i);
            let content = format!("Content of file {}\n", i);

            fs::write(&path, content).await?;
            println!("Created file {}", i);

            // Read it back
            let read_content = fs::read_to_string(&path).await?;
            println!("Read file {}: {}", i, read_content.trim());

            // Clean up
            fs::remove_file(&path).await?;

            Ok::<(), std::io::Error>(())
        });
        handles.push(handle);
    }

    // Wait for all operations
    for handle in handles {
        handle.await.unwrap()?;
    }

    println!("All file operations completed");

    Ok(())
}

// ============================================================================
// 10. ERROR HANDLING IN ASYNC
// ============================================================================

#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Timeout,
    Custom(String),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO Error: {}", e),
            AppError::Timeout => write!(f, "Timeout Error"),
            AppError::Custom(msg) => write!(f, "Custom Error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

/// Async function with error handling using Result
async fn async_with_error_handling(should_fail: bool) -> Result<String, AppError> {
    if should_fail {
        return Err(AppError::Custom("Intentional failure".to_string()));
    }

    // Simulate some async work
    sleep(Duration::from_millis(50)).await;

    // Use ? operator for error propagation
    let file_path = "/tmp/error_test.txt";
    fs::write(file_path, b"test").await?;
    let contents = fs::read_to_string(file_path).await?;
    fs::remove_file(file_path).await?;

    Ok(contents)
}

/// Demonstrates error handling with timeout
async fn async_with_timeout_error() -> Result<String, AppError> {
    let operation = async {
        sleep(Duration::from_millis(500)).await;
        Ok::<String, AppError>("Success".to_string())
    };

    match timeout(Duration::from_millis(100), operation).await {
        Ok(result) => result,
        Err(_) => Err(AppError::Timeout),
    }
}

/// Demonstrates using Result with try-join
async fn try_join_example() -> Result<(), AppError> {
    println!("\n=== Try Join ===");

    async fn task_success() -> Result<i32, AppError> {
        sleep(Duration::from_millis(100)).await;
        Ok(42)
    }

    async fn task_failure() -> Result<i32, AppError> {
        sleep(Duration::from_millis(50)).await;
        Err(AppError::Custom("Task failed".to_string()))
    }

    // try_join! short-circuits on first error
    match tokio::try_join!(task_success(), task_success()) {
        Ok((a, b)) => println!("Both succeeded: {}, {}", a, b),
        Err(e) => println!("Error: {}", e),
    }

    // This will fail
    match tokio::try_join!(task_success(), task_failure()) {
        Ok((a, b)) => println!("Both succeeded: {}, {}", a, b),
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}

// ============================================================================
// PRACTICAL EXAMPLE: WEB SCRAPER SIMULATION
// ============================================================================

/// Simulates fetching a URL
async fn fetch_url(url: &str) -> Result<String, AppError> {
    println!("Fetching: {}", url);
    sleep(Duration::from_millis(100)).await;
    Ok(format!("Content from {}", url))
}

/// Demonstrates a practical async pattern: concurrent web requests
async fn web_scraper_example() -> Result<(), AppError> {
    println!("\n=== Web Scraper Example ===");

    let urls = vec![
        "http://example.com/1",
        "http://example.com/2",
        "http://example.com/3",
    ];

    // Fetch all URLs concurrently
    let mut handles = vec![];

    for url in urls {
        let handle = tokio::spawn(async move { fetch_url(url).await });
        handles.push(handle);
    }

    // Collect results
    for handle in handles {
        match handle.await {
            Ok(Ok(content)) => println!("Got: {}", content),
            Ok(Err(e)) => println!("Error: {}", e),
            Err(e) => println!("Task error: {}", e),
        }
    }

    Ok(())
}

// ============================================================================
// MAIN FUNCTION
// ============================================================================

#[tokio::main]
async fn main() {
    println!("=== Rust Async/Await Comprehensive Examples ===\n");

    // 1. Basic async/await
    let result = simple_async_function().await;
    println!("Simple async: {}", result);

    let result = async_with_params("World", 50).await;
    println!("Async with params: {}", result);

    // 2. Async blocks
    async_blocks_parallel().await;

    // 3. Futures
    working_with_futures().await;

    // 4. Tokio runtime (demonstrated in separate function)
    // tokio_runtime_examples(); // Uncomment to see runtime examples

    // 5. Spawning tasks
    spawning_tasks().await;
    task_cancellation().await;

    // 6. join! and select!
    join_macro_example().await;
    select_macro_example().await;
    select_biased_example().await;

    // 7. Channels
    mpsc_channel_example().await;
    bounded_unbounded_channels().await;

    // 8. Timeouts and delays
    timeout_examples().await;
    interval_example().await;

    // 9. Async file I/O
    if let Err(e) = async_file_io_example().await {
        eprintln!("File I/O error: {}", e);
    }

    if let Err(e) = concurrent_file_operations().await {
        eprintln!("Concurrent file ops error: {}", e);
    }

    // 10. Error handling
    match async_with_error_handling(false).await {
        Ok(result) => println!("\nError handling success: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match async_with_error_handling(true).await {
        Ok(result) => println!("Error handling success: {}", result),
        Err(e) => println!("Error (expected): {}", e),
    }

    match async_with_timeout_error().await {
        Ok(result) => println!("Timeout success: {}", result),
        Err(e) => println!("Timeout error (expected): {}", e),
    }

    if let Err(e) = try_join_example().await {
        eprintln!("Try join error: {}", e);
    }

    // Practical example
    if let Err(e) = web_scraper_example().await {
        eprintln!("Web scraper error: {}", e);
    }

    println!("\n=== All examples completed ===");
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_simple_async_function() {
        let result = simple_async_function().await;
        assert_eq!(result, "Hello from async!");
    }

    #[tokio::test]
    async fn test_async_with_params() {
        let result = async_with_params("Test", 10).await;
        assert_eq!(result, "Hello, Test!");
    }

    #[tokio::test]
    async fn test_async_with_result_success() {
        let result = async_with_result(true).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Success!");
    }

    #[tokio::test]
    async fn test_async_with_result_failure() {
        let result = async_with_result(false).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_spawning_task() {
        let handle = tokio::spawn(async { 42 });
        let result = handle.await.unwrap();
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_join_macro() {
        let task1 = async { 1 };
        let task2 = async { 2 };
        let (r1, r2) = tokio::join!(task1, task2);
        assert_eq!(r1, 1);
        assert_eq!(r2, 2);
    }

    #[tokio::test]
    async fn test_mpsc_channel() {
        let (tx, mut rx) = mpsc::channel::<i32>(10);

        tokio::spawn(async move {
            for i in 0..5 {
                tx.send(i).await.unwrap();
            }
        });

        let mut sum = 0;
        while let Some(val) = rx.recv().await {
            sum += val;
        }

        assert_eq!(sum, 10); // 0+1+2+3+4
    }

    #[tokio::test]
    async fn test_timeout_success() {
        let operation = async {
            sleep(Duration::from_millis(50)).await;
            "Done"
        };

        let result = timeout(Duration::from_millis(100), operation).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_timeout_failure() {
        let operation = async {
            sleep(Duration::from_millis(200)).await;
            "Done"
        };

        let result = timeout(Duration::from_millis(100), operation).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_async_file_io() {
        let path = "/tmp/test_async_io.txt";
        let content = "Test content";

        fs::write(path, content).await.unwrap();
        let read_content = fs::read_to_string(path).await.unwrap();
        assert_eq!(read_content, content);

        fs::remove_file(path).await.unwrap();
    }

    #[tokio::test]
    async fn test_error_handling_success() {
        let result = async_with_error_handling(false).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_error_handling_failure() {
        let result = async_with_error_handling(true).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_custom_future() {
        let value = get_delayed_value(99).await;
        assert_eq!(value, 99);
    }

    #[tokio::test]
    async fn test_select_macro() {
        let mut counter = 0;

        tokio::select! {
            _ = sleep(Duration::from_millis(50)) => {
                counter = 1;
            }
            _ = sleep(Duration::from_millis(100)) => {
                counter = 2;
            }
        }

        // First branch should complete first
        assert_eq!(counter, 1);
    }

    #[tokio::test]
    async fn test_fetch_url() {
        let result = fetch_url("http://test.com").await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("test.com"));
    }
}
