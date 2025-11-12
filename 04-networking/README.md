# Networking Examples

This directory contains comprehensive examples of network programming in Rust. Each example is an independent Cargo project demonstrating different networking protocols and patterns.

## Examples

1. **[01-udp](01-udp/)** - UDP server and client with datagram communication
2. **[02-tcp](02-tcp/)** - TCP server and client with stream-based communication
3. **[03-tls](03-tls/)** - TLS encrypted connections with rustls
4. **[04-http](04-http/)** - HTTP server and client using standard libraries
5. **[05-dns](05-dns/)** - DNS server and client with hickory-dns

## Key Concepts

### Transport Layer
- **UDP**: Connectionless, unreliable, fast datagram communication
- **TCP**: Connection-oriented, reliable, ordered stream communication

### Security
- **TLS**: Transport Layer Security for encrypted connections
- Certificate generation and validation
- Client and server authentication

### Application Layer
- **HTTP**: Request/response protocol for web applications
- **DNS**: Domain Name System for hostname resolution

## Running Examples

Each example can be run independently. Most examples include both server and client:

```bash
# Terminal 1 - Run server
cd 01-udp
cargo run --bin server

# Terminal 2 - Run client
cd 01-udp
cargo run --bin client
```

## Prerequisites

- Rust and Cargo installed
- Basic understanding of networking concepts
- For TLS examples: OpenSSL or similar (for certificate generation)

## Network Programming in Rust

Rust provides excellent support for network programming through:
- **std::net**: Standard library networking (TCP, UDP)
- **tokio**: Async runtime for scalable network applications
- **rustls**: Modern TLS implementation
- **hyper**: Fast HTTP implementation
- **hickory-dns**: DNS client and server

## Best Practices

1. **Error Handling**: Network operations can fail - always handle errors
2. **Timeouts**: Set timeouts to prevent hanging
3. **Resource Cleanup**: Ensure sockets are properly closed
4. **Security**: Use TLS for sensitive data
5. **Testing**: Test with various network conditions

## Common Patterns

### Server Pattern
```rust
let listener = TcpListener::bind("127.0.0.1:8080")?;
for stream in listener.incoming() {
    let stream = stream?;
    handle_client(stream);
}
```

### Client Pattern
```rust
let mut stream = TcpStream::connect("127.0.0.1:8080")?;
stream.write_all(b"Hello")?;
```

### Error Handling
```rust
match socket.recv_from(&mut buf) {
    Ok((size, addr)) => process(size, addr),
    Err(e) => eprintln!("Error: {}", e),
}
```
