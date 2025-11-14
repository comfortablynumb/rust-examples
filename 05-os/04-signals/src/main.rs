use signal_hook::consts::signal::*;
use signal_hook::iterator::Signals;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::signal;

#[tokio::main]
async fn main() {
    println!("Unix Signal Handling Examples\n");
    println!("Press Ctrl+C to trigger SIGINT\n");

    // Example 1: Basic Ctrl+C handling with tokio
    println!("=== Tokio Signal Handling ===");
    tokio_signal_example().await;

    // Example 2: Multiple signals with signal-hook
    println!("\n=== Multiple Signal Handling ===");
    multiple_signals_example();
}

async fn tokio_signal_example() {
    let mut sigint_count = 0;

    println!("Waiting for signals (will exit after 2 Ctrl+C)...");

    loop {
        tokio::select! {
            _ = signal::ctrl_c() => {
                sigint_count += 1;
                println!("\nReceived Ctrl+C (count: {})", sigint_count);

                if sigint_count >= 2 {
                    println!("Exiting gracefully...");
                    break;
                } else {
                    println!("Press Ctrl+C again to exit");
                }
            }
            _ = tokio::time::sleep(Duration::from_secs(1)) => {
                // Continue running
            }
        }

        // Simulate some work
        if sigint_count == 0 {
            tokio::time::sleep(Duration::from_secs(2)).await;
            println!("Working... (send SIGINT with Ctrl+C)");
        }
    }
}

fn multiple_signals_example() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Register signal handlers
    let mut signals = Signals::new(&[SIGINT, SIGTERM, SIGHUP])
        .expect("Failed to register signals");

    thread::spawn(move || {
        for sig in signals.forever() {
            println!("\nReceived signal: {}", signal_name(sig));

            match sig {
                SIGINT => {
                    println!("SIGINT received (Ctrl+C)");
                    r.store(false, Ordering::SeqCst);
                    break;
                }
                SIGTERM => {
                    println!("SIGTERM received (termination request)");
                    r.store(false, Ordering::SeqCst);
                    break;
                }
                SIGHUP => {
                    println!("SIGHUP received (hangup/reload config)");
                    // In real app, reload configuration here
                }
                _ => println!("Received signal: {}", sig),
            }
        }
    });

    println!("Signal handler thread started");
    println!("Main thread continuing...");

    // Simulate work
    let mut i = 0;
    while running.load(Ordering::SeqCst) && i < 3 {
        thread::sleep(Duration::from_secs(1));
        println!("Working... (iteration {})", i + 1);
        i += 1;
    }

    println!("\nShutting down gracefully");
}

fn signal_name(sig: i32) -> &'static str {
    match sig {
        SIGINT => "SIGINT",
        SIGTERM => "SIGTERM",
        SIGHUP => "SIGHUP",
        SIGQUIT => "SIGQUIT",
        SIGKILL => "SIGKILL",
        _ => "Unknown",
    }
}
