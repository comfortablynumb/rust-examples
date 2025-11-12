#![allow(clippy::useless_vec)]
#![allow(dead_code)]

// REST API Server Example
//
// Demonstrates building a simple RESTful API with JSON responses.

use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

type Database = Arc<Mutex<HashMap<u32, String>>>;

fn handle_request(mut stream: TcpStream, db: Database) -> io::Result<()> {
    let peer_addr = stream.peer_addr()?;
    let mut reader = BufReader::new(stream.try_clone()?);

    // Read request line
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;

    println!("[API Request] {} - {}", peer_addr, request_line.trim());

    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return send_json_response(&mut stream, 400, r#"{"error": "Bad request"}"#);
    }

    let method = parts[0];
    let path = parts[1];

    // Skip headers
    let mut line = String::new();
    loop {
        line.clear();
        reader.read_line(&mut line)?;
        if line == "\r\n" || line == "\n" {
            break;
        }
    }

    // Route API requests
    match (method, path) {
        ("GET", "/api/items") => {
            // List all items
            let db = db.lock().unwrap();
            let items: Vec<String> = db
                .iter()
                .map(|(id, name)| format!(r#"{{"id": {}, "name": "{}"}}"#, id, name))
                .collect();
            let json = format!(r#"{{"items": [{}]}}"#, items.join(","));
            send_json_response(&mut stream, 200, &json)?;
        }
        ("GET", path) if path.starts_with("/api/items/") => {
            // Get specific item
            if let Ok(id) = path[11..].parse::<u32>() {
                let db = db.lock().unwrap();
                if let Some(name) = db.get(&id) {
                    let json = format!(r#"{{"id": {}, "name": "{}"}}"#, id, name);
                    send_json_response(&mut stream, 200, &json)?;
                } else {
                    send_json_response(&mut stream, 404, r#"{"error": "Item not found"}"#)?;
                }
            } else {
                send_json_response(&mut stream, 400, r#"{"error": "Invalid ID"}"#)?;
            }
        }
        ("POST", "/api/items") => {
            // Create new item (simplified - in real code, parse JSON body)
            let mut db = db.lock().unwrap();
            let id = (db.len() + 1) as u32;
            let name = format!("Item {}", id);
            db.insert(id, name.clone());
            let json = format!(r#"{{"id": {}, "name": "{}", "created": true}}"#, id, name);
            send_json_response(&mut stream, 201, &json)?;
        }
        ("DELETE", path) if path.starts_with("/api/items/") => {
            // Delete item
            if let Ok(id) = path[11..].parse::<u32>() {
                let mut db = db.lock().unwrap();
                if db.remove(&id).is_some() {
                    send_json_response(&mut stream, 200, r#"{"deleted": true}"#)?;
                } else {
                    send_json_response(&mut stream, 404, r#"{"error": "Item not found"}"#)?;
                }
            } else {
                send_json_response(&mut stream, 400, r#"{"error": "Invalid ID"}"#)?;
            }
        }
        ("GET", "/") => {
            send_html_response(
                &mut stream,
                200,
                "<h1>REST API Server</h1><p>Try these endpoints:</p>\
                 <ul>\
                 <li>GET /api/items - List all items</li>\
                 <li>GET /api/items/1 - Get item by ID</li>\
                 <li>POST /api/items - Create new item</li>\
                 <li>DELETE /api/items/1 - Delete item</li>\
                 </ul>",
            )?;
        }
        _ => send_json_response(&mut stream, 404, r#"{"error": "Endpoint not found"}"#)?,
    }

    Ok(())
}

fn send_json_response(stream: &mut TcpStream, status_code: u16, json: &str) -> io::Result<()> {
    let status_text = match status_code {
        200 => "OK",
        201 => "Created",
        400 => "Bad Request",
        404 => "Not Found",
        _ => "Error",
    };

    let response = format!(
        "HTTP/1.1 {} {}\r\n\
         Content-Type: application/json\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {}",
        status_code,
        status_text,
        json.len(),
        json
    );

    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn send_html_response(stream: &mut TcpStream, status_code: u16, html: &str) -> io::Result<()> {
    let response = format!(
        "HTTP/1.1 {} OK\r\n\
         Content-Type: text/html\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {}",
        status_code,
        html.len(),
        html
    );

    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn main() -> io::Result<()> {
    println!("=== REST API Server ===\n");

    // In-memory database
    let db: Database = Arc::new(Mutex::new(HashMap::new()));

    // Add some initial data
    {
        let mut db = db.lock().unwrap();
        db.insert(1, "Apple".to_string());
        db.insert(2, "Banana".to_string());
        db.insert(3, "Cherry".to_string());
    }

    let listener = TcpListener::bind("127.0.0.1:8000")?;
    println!("REST API Server listening on http://127.0.0.1:8000");
    println!("\nEndpoints:");
    println!("  GET    http://127.0.0.1:8000/api/items");
    println!("  GET    http://127.0.0.1:8000/api/items/1");
    println!("  POST   http://127.0.0.1:8000/api/items");
    println!("  DELETE http://127.0.0.1:8000/api/items/1\n");

    println!("Try with curl:");
    println!("  curl http://127.0.0.1:8000/api/items");
    println!("  curl -X POST http://127.0.0.1:8000/api/items");
    println!("  curl -X DELETE http://127.0.0.1:8000/api/items/1\n");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let db = Arc::clone(&db);
                std::thread::spawn(move || {
                    if let Err(e) = handle_request(stream, db) {
                        eprintln!("[Error] {}", e);
                    }
                });
            }
            Err(e) => eprintln!("[Connection Error] {}", e),
        }
    }

    Ok(())
}
