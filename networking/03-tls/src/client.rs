#![allow(clippy::useless_vec)]
#![allow(dead_code)]

// TLS Client Example
//
// Demonstrates connecting to a TLS server with certificate validation.

use rustls::pki_types::ServerName;
use rustls::ClientConfig;
use rustls::RootCertStore;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::Arc;

fn main() -> io::Result<()> {
    println!("=== TLS Client ===\n");

    // Configure TLS client with root certificates
    let mut root_store = RootCertStore::empty();

    // Add system root certificates
    for cert in webpki_roots::TLS_SERVER_ROOTS.iter() {
        root_store.roots.push(cert.clone());
    }

    let config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let config = Arc::new(config);

    // Connect to server
    println!("Connecting to 127.0.0.1:8443...");
    let stream = TcpStream::connect("127.0.0.1:8443")?;
    println!("TCP connection established");

    // Create TLS connection
    let server_name = ServerName::try_from("localhost")
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid server name"))?;

    let conn = rustls::ClientConnection::new(config, server_name.to_owned())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("TLS error: {}", e)))?;

    let mut tls_stream = rustls::StreamOwned::new(conn, stream);

    println!("TLS handshake complete\n");
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

        // Send encrypted message
        tls_stream.write_all(message.as_bytes())?;
        tls_stream.write_all(b"\n")?;
        tls_stream.flush()?;
        println!("Sent (encrypted): {}", message);

        // Read encrypted response
        response.clear();
        let mut reader = BufReader::new(&mut tls_stream);
        reader.read_line(&mut response)?;
        println!("Response (decrypted): {}\n", response.trim());

        if message == "quit" {
            println!("Goodbye!");
            break;
        }
    }

    Ok(())
}
