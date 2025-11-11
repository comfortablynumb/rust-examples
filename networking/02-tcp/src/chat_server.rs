#![allow(clippy::useless_vec)]
#![allow(dead_code)]

// TCP Chat Server Example
//
// Demonstrates handling multiple concurrent TCP connections and
// broadcasting messages to all connected clients.

use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

type ClientList = Arc<Mutex<Vec<TcpStream>>>;

fn broadcast_message(clients: &ClientList, message: &str, sender_addr: &str) {
    let mut clients = clients.lock().unwrap();
    let broadcast = format!("[{}]: {}\n", sender_addr, message);

    // Remove disconnected clients and send to active ones
    clients.retain(|mut client| client.write_all(broadcast.as_bytes()).is_ok());
}

fn handle_client(stream: TcpStream, clients: ClientList) -> io::Result<()> {
    let peer_addr = stream.peer_addr()?.to_string();
    println!("[Join] {} connected", peer_addr);

    // Add client to list
    {
        let mut client_list = clients.lock().unwrap();
        client_list.push(stream.try_clone()?);
    }

    // Notify others
    broadcast_message(&clients, "joined the chat", &peer_addr);

    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => {
                println!("[Leave] {} disconnected", peer_addr);
                broadcast_message(&clients, "left the chat", &peer_addr);
                break;
            }
            Ok(_) => {
                let message = line.trim();
                if message == "quit" {
                    println!("[Quit] {} is leaving", peer_addr);
                    broadcast_message(&clients, "left the chat", &peer_addr);
                    break;
                }

                println!("[Message] {}: {}", peer_addr, message);
                broadcast_message(&clients, message, &peer_addr);
            }
            Err(e) => {
                eprintln!("[Error] {}: {}", peer_addr, e);
                break;
            }
        }
    }

    // Remove client from list
    let mut client_list = clients.lock().unwrap();
    client_list.retain(|c| c.peer_addr().unwrap().to_string() != peer_addr);

    Ok(())
}

fn main() -> io::Result<()> {
    println!("=== TCP Chat Server ===\n");

    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Chat server listening on 127.0.0.1:8080");
    println!("Clients can connect with: telnet 127.0.0.1 8080\n");

    let clients: ClientList = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let clients = Arc::clone(&clients);
                thread::spawn(move || {
                    if let Err(e) = handle_client(stream, clients) {
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
