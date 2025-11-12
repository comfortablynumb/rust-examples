#![allow(clippy::useless_vec)]

// TCP Client Example
//
// Demonstrates connecting to a TCP server, sending messages, and receiving responses.

use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    println!("=== TCP Client ===\n");

    // Connect to the server
    let server_addr = "127.0.0.1:8080";
    println!("Connecting to {}...", server_addr);

    let stream = TcpStream::connect(server_addr)?;
    let local_addr = stream.local_addr()?;
    let peer_addr = stream.peer_addr()?;

    println!("Connected!");
    println!("  Local address: {}", local_addr);
    println!("  Server address: {}\n", peer_addr);

    // Clone stream for reading
    let read_stream = stream.try_clone()?;
    let mut writer = stream;
    let mut reader = BufReader::new(read_stream);

    println!("Enter messages to send (or 'quit' to exit):");

    let stdin = io::stdin();
    let mut input = String::new();
    let mut response = String::new();

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
        writer.write_all(message.as_bytes())?;
        writer.write_all(b"\n")?;
        writer.flush()?;
        println!("Sent: {}", message);

        // Read response from server
        response.clear();
        match reader.read_line(&mut response) {
            Ok(0) => {
                println!("\nServer closed the connection");
                break;
            }
            Ok(_) => {
                println!("Response: {}", response.trim());
            }
            Err(e) => {
                eprintln!("Error reading response: {}", e);
                break;
            }
        }

        // Exit if user typed quit
        if message == "quit" {
            println!("\nGoodbye!");
            break;
        }

        println!();
    }

    Ok(())
}
