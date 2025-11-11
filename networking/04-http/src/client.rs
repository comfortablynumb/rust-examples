#![allow(clippy::useless_vec)]

// HTTP Client Example
//
// Demonstrates making HTTP requests using TCP sockets.
// Shows the fundamentals of HTTP protocol.

use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::TcpStream;

fn http_get(url: &str) -> io::Result<String> {
    // Parse URL (simplified - only handles http://host:port/path)
    let url = url.strip_prefix("http://").ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "URL must start with http://")
    })?;

    let (host_port, path) = url.split_once('/').unwrap_or((url, ""));
    let path = format!("/{}", path);

    let (host, port) = if let Some((h, p)) = host_port.split_once(':') {
        (h, p.parse().unwrap_or(80))
    } else {
        (host_port, 80)
    };

    println!("[Connecting] {}:{}", host, port);
    let mut stream = TcpStream::connect((host, port))?;
    println!("[Connected]");

    // Send HTTP request
    let request = format!(
        "GET {} HTTP/1.1\r\n\
         Host: {}\r\n\
         User-Agent: Rust-HTTP-Client/1.0\r\n\
         Accept: */*\r\n\
         Connection: close\r\n\
         \r\n",
        path, host
    );

    println!("[Request]\n{}", request);
    stream.write_all(request.as_bytes())?;
    stream.flush()?;

    // Read response
    let mut reader = BufReader::new(stream);
    let mut status_line = String::new();
    reader.read_line(&mut status_line)?;

    println!("[Response] {}", status_line.trim());

    // Read headers
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

    println!("[Headers] ({} headers)", headers.len());
    for header in &headers {
        println!("  {}", header);
    }

    // Read body
    let mut body = String::new();
    reader.read_to_string(&mut body)?;

    Ok(body)
}

fn main() -> io::Result<()> {
    println!("=== HTTP Client ===\n");

    let urls = vec![
        "http://127.0.0.1:8000/",
        "http://127.0.0.1:8000/hello",
        "http://127.0.0.1:8000/json",
    ];

    for url in urls {
        println!("\n{}", "=".repeat(60));
        println!("Fetching: {}", url);
        println!("{}\n", "=".repeat(60));

        match http_get(url) {
            Ok(body) => {
                println!("[Body] ({} bytes)", body.len());
                println!("{}\n", body);
            }
            Err(e) => {
                eprintln!("[Error] Failed to fetch {}: {}\n", url, e);
                println!("Note: Make sure the HTTP server is running:");
                println!("  cargo run --bin server\n");
            }
        }
    }

    Ok(())
}
