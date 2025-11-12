#![allow(clippy::useless_vec)]

// TCP Server Example
//
// TCP (Transmission Control Protocol) provides reliable, ordered, connection-oriented
// communication. Unlike UDP, TCP establishes a connection before data transfer and
// guarantees delivery and ordering of packets.

use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let peer_addr = stream.peer_addr()?;
    println!("[New Connection] Client: {}", peer_addr);

    // Set timeout to prevent hanging
    stream.set_read_timeout(Some(Duration::from_secs(60)))?;

    let mut reader = BufReader::new(stream.try_clone()?);
    let mut line = String::new();
    let mut message_count = 0;

    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => {
                // Connection closed by client
                println!("[Disconnected] {}", peer_addr);
                break;
            }
            Ok(_) => {
                message_count += 1;
                let message = line.trim();
                println!(
                    "[Message #{}] From {}: {}",
                    message_count, peer_addr, message
                );

                // Check for quit command
                if message == "quit" {
                    println!("[Quit Request] {} is disconnecting", peer_addr);
                    let response = "Goodbye!\n";
                    stream.write_all(response.as_bytes())?;
                    break;
                }

                // Echo back to client
                let response = format!("Echo: {}\n", message);
                stream.write_all(response.as_bytes())?;
                println!("[Response] Sent to {}", peer_addr);
            }
            Err(e) => {
                eprintln!("[Error] Reading from {}: {}", peer_addr, e);
                break;
            }
        }
    }

    println!(
        "[Session End] {} - {} messages processed",
        peer_addr, message_count
    );
    Ok(())
}

fn main() -> io::Result<()> {
    println!("=== TCP Server ===\n");

    // Bind to address and start listening
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on 127.0.0.1:8080");
    println!("Waiting for connections...\n");

    // Accept connections and process them in separate threads
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread for each client
                thread::spawn(move || {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("[Thread Error] {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("[Connection Error] {}", e);
            }
        }
    }

    Ok(())
}
