#![allow(clippy::useless_vec)]

// HTTP Server Example
//
// Demonstrates building a basic HTTP/1.1 server using TCP sockets.
// This shows the fundamentals of HTTP without external dependencies.

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::{fs, io};

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let peer_addr = stream.peer_addr()?;
    let mut reader = BufReader::new(stream.try_clone()?);

    // Read request line
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;

    println!("[Request] {} - {}", peer_addr, request_line.trim());

    // Parse request (GET /path HTTP/1.1)
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        send_response(&mut stream, 400, "Bad Request", "Invalid request")?;
        return Ok(());
    }

    let method = parts[0];
    let path = parts[1];

    // Read headers (we'll skip them for this simple example)
    let mut headers = Vec::new();
    let mut line = String::new();
    loop {
        line.clear();
        reader.read_line(&mut line)?;
        if line == "\r\n" || line == "\n" {
            break;
        }
        headers.push(line.trim().to_string());
    }

    // Route the request
    match (method, path) {
        ("GET", "/") => send_response(&mut stream, 200, "OK", "<h1>Welcome to Rust HTTP Server!</h1><p><a href='/about'>About</a> | <a href='/hello'>Hello</a></p>")?,
        ("GET", "/hello") => send_response(&mut stream, 200, "OK", "<h1>Hello, World!</h1><p><a href='/'>Home</a></p>")?,
        ("GET", "/about") => send_response(&mut stream, 200, "OK", "<h1>About</h1><p>Simple HTTP server built with Rust</p><p><a href='/'>Home</a></p>")?,
        ("GET", "/json") => send_json_response(&mut stream, r#"{"message": "Hello from Rust!", "status": "success"}"#)?,
        ("GET", _) if path.starts_with("/file/") => {
            // Serve a file (be careful with path traversal in production!)
            let filename = &path[6..]; // Remove "/file/" prefix
            serve_file(&mut stream, filename)?;
        }
        _ => send_response(&mut stream, 404, "Not Found", "<h1>404 Not Found</h1><p>The requested page does not exist.</p><p><a href='/'>Home</a></p>")?,
    }

    Ok(())
}

fn send_response(
    stream: &mut TcpStream,
    status_code: u16,
    status_text: &str,
    body: &str,
) -> io::Result<()> {
    let response = format!(
        "HTTP/1.1 {} {}\r\n\
         Content-Type: text/html; charset=utf-8\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {}",
        status_code,
        status_text,
        body.len(),
        body
    );

    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn send_json_response(stream: &mut TcpStream, json: &str) -> io::Result<()> {
    let response = format!(
        "HTTP/1.1 200 OK\r\n\
         Content-Type: application/json\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {}",
        json.len(),
        json
    );

    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn serve_file(stream: &mut TcpStream, filename: &str) -> io::Result<()> {
    match fs::read_to_string(filename) {
        Ok(contents) => {
            send_response(stream, 200, "OK", &contents)?;
        }
        Err(_) => {
            send_response(stream, 404, "Not Found", "<h1>File Not Found</h1>")?;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    println!("=== HTTP Server ===\n");

    let listener = TcpListener::bind("127.0.0.1:8000")?;
    println!("HTTP Server listening on http://127.0.0.1:8000");
    println!("Try these URLs:");
    println!("  http://127.0.0.1:8000/");
    println!("  http://127.0.0.1:8000/hello");
    println!("  http://127.0.0.1:8000/about");
    println!("  http://127.0.0.1:8000/json\n");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("[Error] {}", e);
                    }
                });
            }
            Err(e) => eprintln!("[Connection Error] {}", e),
        }
    }

    Ok(())
}
