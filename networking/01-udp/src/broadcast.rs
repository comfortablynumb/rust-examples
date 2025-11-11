#![allow(clippy::useless_vec)]

// UDP Broadcast Example
//
// Demonstrates UDP broadcasting to send messages to all devices on the network.

use std::io;
use std::net::UdpSocket;
use std::time::Duration;

fn main() -> io::Result<()> {
    println!("=== UDP Broadcast Example ===\n");

    // Create socket for broadcasting
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;
    socket.set_read_timeout(Some(Duration::from_secs(2)))?;

    println!("Broadcasting on port 9999...");

    // Broadcast message
    let broadcast_addr = "255.255.255.255:9999";
    let messages = vec!["Hello from Rust!", "Broadcast message #2", "Final message"];

    for (i, message) in messages.iter().enumerate() {
        println!("\n[Broadcast #{}]", i + 1);
        println!("  Sending: {}", message);

        match socket.send_to(message.as_bytes(), broadcast_addr) {
            Ok(size) => println!("  Sent {} bytes", size),
            Err(e) => eprintln!("  Error sending: {}", e),
        }

        // Try to receive any responses
        let mut buf = [0u8; 1024];
        match socket.recv_from(&mut buf) {
            Ok((size, addr)) => {
                let response = String::from_utf8_lossy(&buf[..size]);
                println!("  Response from {}: {}", addr, response);
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                println!("  No response received (timeout)");
            }
            Err(e) => {
                eprintln!("  Error receiving: {}", e);
            }
        }

        std::thread::sleep(Duration::from_secs(1));
    }

    println!("\nBroadcast complete!");
    println!("Note: Broadcast may be blocked by firewalls or network configuration");

    Ok(())
}
