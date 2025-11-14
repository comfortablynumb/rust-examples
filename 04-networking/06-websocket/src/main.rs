use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, connect_async, tungstenite::Message};

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    username: String,
    message: String,
    timestamp: u64,
}

#[tokio::main]
async fn main() {
    println!("WebSocket Example\n");

    // Spawn server in background
    let server_handle = tokio::spawn(async {
        run_server().await.expect("Server failed");
    });

    // Give server time to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Run client
    run_client().await.expect("Client failed");

    // Clean shutdown (in real app, would handle this properly)
    server_handle.abort();
}

async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:9001";
    let listener = TcpListener::bind(addr).await?;
    println!("WebSocket server listening on: {}", addr);

    // Accept one connection for demo
    let (stream, peer) = listener.accept().await?;
    println!("New connection from: {}", peer);

    handle_connection(stream, peer).await?;

    Ok(())
}

async fn handle_connection(
    stream: TcpStream,
    peer: SocketAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    let ws_stream = accept_async(stream).await?;
    println!("WebSocket handshake completed with {}", peer);

    let (mut write, mut read) = ws_stream.split();

    // Send welcome message
    let welcome = ChatMessage {
        username: "Server".to_string(),
        message: "Welcome to the chat!".to_string(),
        timestamp: current_timestamp(),
    };
    let welcome_json = serde_json::to_string(&welcome)?;
    write.send(Message::Text(welcome_json)).await?;

    // Echo messages back
    while let Some(msg) = read.next().await {
        let msg = msg?;

        match msg {
            Message::Text(text) => {
                println!("Received text: {}", text);

                // Parse and echo back
                if let Ok(chat_msg) = serde_json::from_str::<ChatMessage>(&text) {
                    println!("  From: {}", chat_msg.username);
                    println!("  Message: {}", chat_msg.message);

                    // Echo back
                    let response = ChatMessage {
                        username: "Server".to_string(),
                        message: format!("Echo: {}", chat_msg.message),
                        timestamp: current_timestamp(),
                    };
                    let response_json = serde_json::to_string(&response)?;
                    write.send(Message::Text(response_json)).await?;
                }
            }
            Message::Binary(data) => {
                println!("Received binary data: {} bytes", data.len());
                write.send(Message::Binary(data)).await?;
            }
            Message::Ping(data) => {
                println!("Received ping");
                write.send(Message::Pong(data)).await?;
            }
            Message::Pong(_) => {
                println!("Received pong");
            }
            Message::Close(_) => {
                println!("Client closed connection");
                break;
            }
            _ => {}
        }
    }

    Ok(())
}

async fn run_client() -> Result<(), Box<dyn std::error::Error>> {
    let url = "ws://127.0.0.1:9001";
    println!("\nConnecting to {}", url);

    let (ws_stream, _) = connect_async(url).await?;
    println!("WebSocket connected");

    let (mut write, mut read) = ws_stream.split();

    // Receive welcome message
    if let Some(msg) = read.next().await {
        let msg = msg?;
        if let Message::Text(text) = msg {
            let chat_msg: ChatMessage = serde_json::from_str(&text)?;
            println!("Server: {}", chat_msg.message);
        }
    }

    // Send messages
    let messages = vec!["Hello, WebSocket!", "How are you?", "Goodbye!"];

    for msg_text in messages {
        let chat_msg = ChatMessage {
            username: "Client".to_string(),
            message: msg_text.to_string(),
            timestamp: current_timestamp(),
        };

        let json = serde_json::to_string(&chat_msg)?;
        write.send(Message::Text(json)).await?;
        println!("\nSent: {}", msg_text);

        // Wait for response
        if let Some(msg) = read.next().await {
            let msg = msg?;
            if let Message::Text(text) = msg {
                let response: ChatMessage = serde_json::from_str(&text)?;
                println!("Received: {}", response.message);
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    // Send ping
    println!("\nSending ping");
    write.send(Message::Ping(vec![])).await?;

    // Wait for pong
    if let Some(msg) = read.next().await {
        let msg = msg?;
        if let Message::Pong(_) = msg {
            println!("Received pong");
        }
    }

    // Close connection
    write.send(Message::Close(None)).await?;
    println!("\nConnection closed");

    Ok(())
}

fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
