#![allow(dead_code)]
#![allow(clippy::useless_vec)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::manual_map)]
#![allow(clippy::unnecessary_literal_unwrap)]
#![allow(clippy::bind_instead_of_map)]
#![allow(clippy::unnecessary_fold)]
#![allow(clippy::unnecessary_sort_by)]
#![allow(clippy::let_and_return)]
#![allow(unused_variables)]
#![allow(clippy::iter_count)]

// Concurrency in Rust
//
// Rust's ownership and type system prevent data races at compile time.
// This makes concurrent programming safer and easier. Key concepts: threads,
// message passing with channels, and shared state with Mutex/Arc.

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Rust Concurrency Examples ===\n");

    // Example 1: Creating threads
    println!("1. Creating Threads:");
    creating_threads();
    println!();

    // Example 2: Moving data into threads
    println!("2. Moving Data into Threads:");
    moving_data();
    println!();

    // Example 3: Message passing with channels
    println!("3. Message Passing (Channels):");
    message_passing();
    println!();

    // Example 4: Multiple producers
    println!("4. Multiple Producers:");
    multiple_producers();
    println!();

    // Example 5: Shared state with Mutex
    println!("5. Shared State (Mutex):");
    shared_state_mutex();
    println!();

    // Example 6: Shared state across threads with Arc
    println!("6. Arc<Mutex<T>> Pattern:");
    arc_mutex_pattern();
    println!();

    // Example 7: Thread pool pattern
    println!("7. Thread Pool Pattern:");
    thread_pool_pattern();
    println!();

    // Example 8: Parallel iteration
    println!("8. Parallel Processing:");
    parallel_processing();
    println!();
}

// Example 1: Creating and joining threads
fn creating_threads() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("    Thread: count {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..3 {
        println!("  Main: count {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Wait for thread to finish
    handle.join().unwrap();
    println!("  Thread completed");

    // Multiple threads
    let mut handles = vec![];

    for i in 0..3 {
        let handle = thread::spawn(move || {
            println!("    Thread {} starting", i);
            thread::sleep(Duration::from_millis(10));
            println!("    Thread {} done", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("  All threads completed");
}

// Example 2: Moving data into threads
fn moving_data() {
    let v = vec![1, 2, 3];

    // move keyword transfers ownership to thread
    let handle = thread::spawn(move || {
        println!("    Thread vector: {:?}", v);
    });

    // v is no longer valid here
    // println!("{:?}", v); // Error!

    handle.join().unwrap();

    // Returning data from threads
    let handle = thread::spawn(|| {
        let result = (1..=100).sum::<i32>();
        result
    });

    let sum = handle.join().unwrap();
    println!("  Sum from thread: {}", sum);

    // Multiple threads with results
    let mut handles = vec![];

    for i in 1..=5 {
        let handle = thread::spawn(move || {
            let sum = (1..=i).sum::<i32>();
            (i, sum)
        });
        handles.push(handle);
    }

    for handle in handles {
        let (n, sum) = handle.join().unwrap();
        println!("  Sum of 1..={} is {}", n, sum);
    }
}

// Example 3: Message passing with channels
fn message_passing() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec!["Hello", "from", "the", "thread"];

        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Receive messages
    for received in rx {
        println!("  Received: {}", received);
    }

    // Single message
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let result = vec![1, 2, 3, 4, 5];
        tx.send(result).unwrap();
    });

    let data = rx.recv().unwrap();
    println!("  Received data: {:?}", data);
}

// Example 4: Multiple producers
fn multiple_producers() {
    let (tx, rx) = mpsc::channel();

    for i in 0..3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let messages = vec![
                format!("Thread {} - Message 1", i),
                format!("Thread {} - Message 2", i),
            ];

            for msg in messages {
                tx_clone.send(msg).unwrap();
                thread::sleep(Duration::from_millis(50));
            }
        });
    }

    // Drop original tx so rx knows when all senders are done
    drop(tx);

    for received in rx {
        println!("  Received: {}", received);
    }

    // Producer-consumer pattern
    let (tx, rx) = mpsc::channel();

    // Producer thread
    thread::spawn(move || {
        for i in 1..=10 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    // Consumer thread
    let handle = thread::spawn(move || {
        let mut sum = 0;
        for received in rx {
            sum += received;
        }
        sum
    });

    let total = handle.join().unwrap();
    println!("  Total from producer-consumer: {}", total);
}

// Example 5: Shared state with Mutex
fn shared_state_mutex() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
        println!("  Locked value: {}", num);
    } // lock is released here

    println!("  After unlock: {:?}", m);

    // Mutex with multiple scopes
    let counter = Mutex::new(0);

    {
        let mut count = counter.lock().unwrap();
        *count += 1;
    }

    {
        let mut count = counter.lock().unwrap();
        *count += 1;
    }

    println!("  Counter: {:?}", counter);
}

// Example 6: Arc<Mutex<T>> - shared mutable state across threads
fn arc_mutex_pattern() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("    Thread {} incremented counter", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("  Final counter: {}", *counter.lock().unwrap());

    // Shared data structure
    let data = Arc::new(Mutex::new(vec![]));
    let mut handles = vec![];

    for i in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut vec = data.lock().unwrap();
            vec.push(i);
            println!("    Thread {} added {}", i, i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("  Shared vector: {:?}", *data.lock().unwrap());
}

// Example 7: Thread pool pattern
fn thread_pool_pattern() {
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));
    let mut handles = vec![];

    // Create worker threads
    for id in 0..4 {
        let rx = Arc::clone(&rx);
        let handle = thread::spawn(move || loop {
            let job = rx.lock().unwrap().recv();
            match job {
                Ok(num) => {
                    println!("    Worker {} processing: {}", id, num);
                    thread::sleep(Duration::from_millis(50));
                }
                Err(_) => {
                    println!("    Worker {} shutting down", id);
                    break;
                }
            }
        });
        handles.push(handle);
    }

    // Send jobs
    for i in 1..=10 {
        tx.send(i).unwrap();
    }

    drop(tx); // Signal workers to shut down

    for handle in handles {
        handle.join().unwrap();
    }

    println!("  All workers completed");
}

// Example 8: Parallel processing
fn parallel_processing() {
    // Map-reduce pattern
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let chunk_size = 3;
    let chunks: Vec<Vec<i32>> = data.chunks(chunk_size).map(|c| c.to_vec()).collect();

    let mut handles = vec![];

    for chunk in chunks {
        let handle = thread::spawn(move || chunk.iter().map(|x| x * x).sum::<i32>());
        handles.push(handle);
    }

    let results: Vec<i32> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    let total: i32 = results.iter().sum();

    println!("  Parallel sum of squares: {}", total);

    // Concurrent computation
    let numbers = Arc::new(vec![1, 2, 3, 4, 5, 6, 7, 8]);
    let results = Arc::new(Mutex::new(vec![]));
    let mut handles = vec![];

    for i in 0..4 {
        let numbers = Arc::clone(&numbers);
        let results = Arc::clone(&results);

        let handle = thread::spawn(move || {
            let start = i * 2;
            let end = (i + 1) * 2;

            for idx in start..end.min(numbers.len()) {
                let squared = numbers[idx] * numbers[idx];
                results.lock().unwrap().push(squared);
                println!("    Thread {} computed {}^2 = {}", i, numbers[idx], squared);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_results = results.lock().unwrap();
    println!("  Results: {:?}", *final_results);

    // Barrier synchronization
    use std::sync::Barrier;

    let barrier = Arc::new(Barrier::new(3));
    let mut handles = vec![];

    for i in 0..3 {
        let barrier = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            println!("    Thread {} before barrier", i);
            barrier.wait();
            println!("    Thread {} after barrier", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("  All threads synchronized");
}
