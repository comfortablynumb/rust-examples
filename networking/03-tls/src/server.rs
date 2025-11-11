#![allow(clippy::useless_vec)]
#![allow(dead_code)]

// TLS Server Example
//
// Demonstrates creating a secure TLS server using rustls.
// TLS (Transport Layer Security) provides encrypted communication.

use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::ServerConfig;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

fn load_certs(path: &str) -> io::Result<Vec<CertificateDer<'static>>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    rustls_pemfile::certs(&mut reader)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid cert"))
}

fn load_private_key(path: &str) -> io::Result<PrivateKeyDer<'static>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    // Try reading as PKCS8 first, then RSA
    if let Some(Ok(key)) = rustls_pemfile::pkcs8_private_keys(&mut reader).next() {
        return Ok(PrivateKeyDer::Pkcs8(key));
    }

    // Reset reader
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    if let Some(Ok(key)) = rustls_pemfile::rsa_private_keys(&mut reader).next() {
        return Ok(PrivateKeyDer::Pkcs1(key));
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "No private key found",
    ))
}

fn handle_client(stream: TcpStream, config: Arc<ServerConfig>) -> io::Result<()> {
    let peer_addr = stream.peer_addr()?;
    println!("[TLS Connection] Client: {}", peer_addr);

    // Create TLS session
    let conn = rustls::ServerConnection::new(config)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("TLS error: {}", e)))?;

    let mut tls_stream = rustls::StreamOwned::new(conn, stream);

    // Read and echo messages
    let mut reader = BufReader::new(tls_stream.get_ref().try_clone()?);
    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => {
                println!("[Disconnected] {}", peer_addr);
                break;
            }
            Ok(_) => {
                let message = line.trim();
                println!("[Secure Message] From {}: {}", peer_addr, message);

                if message == "quit" {
                    tls_stream.write_all(b"Goodbye!\n")?;
                    break;
                }

                let response = format!("Echo: {}\n", message);
                tls_stream.write_all(response.as_bytes())?;
            }
            Err(e) => {
                eprintln!("[Error] {}", e);
                break;
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    println!("=== TLS Server ===\n");

    // Load certificate and private key
    println!("Loading certificates...");
    let certs = load_certs("cert.pem").or_else(|_| {
        eprintln!("Note: cert.pem not found. Using demo certificates.");
        eprintln!("In production, generate proper certificates with:");
        eprintln!(
            "  openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365\n"
        );
        // For demo, we'll just note this - in real usage you'd need actual certs
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Certificate files not found",
        ))
    })?;

    let key = load_private_key("key.pem")?;

    // Configure TLS
    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let config = Arc::new(config);

    // Start listening
    let listener = TcpListener::bind("127.0.0.1:8443")?;
    println!("TLS Server listening on 127.0.0.1:8443\n");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let config = Arc::clone(&config);
                std::thread::spawn(move || {
                    if let Err(e) = handle_client(stream, config) {
                        eprintln!("[Handler Error] {}", e);
                    }
                });
            }
            Err(e) => eprintln!("[Connection Error] {}", e),
        }
    }

    Ok(())
}
