#![allow(clippy::useless_vec)]

// UDP Client Example
//
// Demonstrates sending and receiving UDP datagrams to/from a server.

use std::io::{self, Write};
use std::net::UdpSocket;
use std::time::Duration;

fn main() -> io::Result<()> {
    println!("=== UDP Client ===\n");

    // Bind to any available port
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    let local_addr = socket.local_addr()?;
    println!("Client bound to: {}", local_addr);

    // Set timeout for receiving responses
    socket.set_read_timeout(Some(Duration::from_secs(5)))?;

    // Server address
    let server_addr = "127.0.0.1:8080";
    println!("Connecting to server: {}\n", server_addr);

    // Interactive mode
    println!("Enter messages to send (or 'quit' to exit):");

    let stdin = io::stdin();
    let mut input = String::new();
    let mut buf = [0u8; 1024];

    loop {
        print!("> ");
        io::stdout().flush()?;

        input.clear();
        stdin.read_line(&mut input)?;
        let message = input.trim();

        if message.is_empty() {
            continue;
        }

        // Send message to server
        socket.send_to(message.as_bytes(), server_addr)?;
        println!("Sent: {}", message);

        // Wait for response
        match socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                let response = String::from_utf8_lossy(&buf[..size]);
                println!("Response from {}: {}\n", src, response);
            }
            Err(e) => {
                eprintln!("Error receiving response: {}\n", e);
            }
        }

        // Exit if user typed quit
        if message == "quit" {
            println!("Goodbye!");
            break;
        }
    }

    Ok(())
}
