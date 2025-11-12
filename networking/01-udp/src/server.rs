#![allow(clippy::useless_vec)]

// UDP Server Example
//
// UDP (User Datagram Protocol) is a connectionless protocol that sends
// datagrams without establishing a connection. It's fast but unreliable -
// packets may be lost, duplicated, or arrive out of order.

use std::io;
use std::net::UdpSocket;

fn main() -> io::Result<()> {
    println!("=== UDP Server ===\n");

    // Bind to a local address and port
    let socket = UdpSocket::bind("127.0.0.1:8080")?;
    println!("Server listening on 127.0.0.1:8080");
    println!("Waiting for messages...\n");

    // Set timeout to prevent blocking forever
    socket.set_read_timeout(Some(std::time::Duration::from_secs(60)))?;

    let mut buf = [0u8; 1024];
    let mut message_count = 0;

    loop {
        // Receive data from any client
        match socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                message_count += 1;
                let received = String::from_utf8_lossy(&buf[..size]);
                println!("[Message #{}] From: {}", message_count, src);
                println!("  Received: {}", received);

                // Echo back to client
                let response = format!("Echo: {}", received);
                socket.send_to(response.as_bytes(), src)?;
                println!("  Sent: {}\n", response);

                // Exit command
                if received.trim() == "quit" {
                    println!("Received quit command. Shutting down...");
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                if e.kind() == io::ErrorKind::WouldBlock || e.kind() == io::ErrorKind::TimedOut {
                    println!("Timeout - no messages received");
                    break;
                }
            }
        }
    }

    println!("\nServer statistics:");
    println!("  Total messages received: {}", message_count);
    println!("  Server stopped");

    Ok(())
}
