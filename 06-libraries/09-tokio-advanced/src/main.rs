use std::time::Duration;
use tokio::select;
use tokio::sync::{mpsc, oneshot, Semaphore};
use tokio::time::{interval, sleep, timeout};

#[tokio::main]
async fn main() {
    println!("Tokio Advanced Features\n");

    // Timeouts
    timeout_example().await;

    // Select macro
    select_example().await;

    // Channels
    channel_example().await;

    // Semaphores for rate limiting
    semaphore_example().await;

    // Spawning tasks
    spawn_example().await;

    // Intervals
    interval_example().await;
}

async fn timeout_example() {
    println!("=== Timeout Example ===");

    // Operation that completes in time
    let result = timeout(Duration::from_secs(2), async {
        sleep(Duration::from_millis(100)).await;
        "Completed!"
    })
    .await;

    match result {
        Ok(value) => println!("✓ {}", value),
        Err(_) => println!("✗ Timed out"),
    }

    // Operation that times out
    let result = timeout(Duration::from_millis(100), async {
        sleep(Duration::from_secs(2)).await;
        "This won't complete"
    })
    .await;

    match result {
        Ok(value) => println!("✓ {}", value),
        Err(_) => println!("✗ Timed out (expected)"),
    }

    println!();
}

async fn select_example() {
    println!("=== Select Macro Example ===");

    let mut interval = interval(Duration::from_millis(100));
    let mut count = 0;

    loop {
        select! {
            _ = interval.tick() => {
                count += 1;
                println!("Tick {}", count);

                if count >= 3 {
                    println!("Stopping after 3 ticks");
                    break;
                }
            }
            _ = sleep(Duration::from_millis(250)) => {
                println!("Sleep completed first");
                break;
            }
        }
    }

    println!();
}

async fn channel_example() {
    println!("=== Channel Example ===");

    // mpsc (multiple producer, single consumer)
    let (tx, mut rx) = mpsc::channel(32);

    // Spawn producers
    for i in 1..=3 {
        let tx = tx.clone();
        tokio::spawn(async move {
            sleep(Duration::from_millis(i * 100)).await;
            tx.send(format!("Message from producer {}", i))
                .await
                .unwrap();
        });
    }

    drop(tx); // Close channel

    // Receive all messages
    while let Some(msg) = rx.recv().await {
        println!("Received: {}", msg);
    }

    println!();

    // oneshot channel
    oneshot_example().await;
}

async fn oneshot_example() {
    println!("=== Oneshot Channel ===");

    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        sleep(Duration::from_millis(100)).await;
        tx.send("Hello from oneshot!").unwrap();
    });

    match rx.await {
        Ok(msg) => println!("Received: {}", msg),
        Err(_) => println!("Sender dropped"),
    }

    println!();
}

async fn semaphore_example() {
    println!("=== Semaphore (Rate Limiting) ===");

    let semaphore = std::sync::Arc::new(Semaphore::new(2)); // Allow 2 concurrent tasks

    let mut handles = vec![];

    for i in 1..=5 {
        let permit = semaphore.clone();
        let handle = tokio::spawn(async move {
            let _permit = permit.acquire().await.unwrap();
            println!("Task {} acquired permit", i);

            sleep(Duration::from_millis(500)).await;

            println!("Task {} releasing permit", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    println!();
}

async fn spawn_example() {
    println!("=== Spawning Tasks ===");

    let handle1 = tokio::spawn(async {
        sleep(Duration::from_millis(100)).await;
        "Task 1 result"
    });

    let handle2 = tokio::spawn(async {
        sleep(Duration::from_millis(150)).await;
        "Task 2 result"
    });

    // Wait for both
    let (result1, result2) = tokio::join!(handle1, handle2);

    println!("Result 1: {}", result1.unwrap());
    println!("Result 2: {}", result2.unwrap());

    println!();
}

async fn interval_example() {
    println!("=== Interval Example ===");

    let mut interval = interval(Duration::from_millis(200));

    for i in 1..=3 {
        interval.tick().await;
        println!("Interval tick {}", i);
    }

    println!();
}
