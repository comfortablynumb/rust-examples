# TCP Server and Client

This example demonstrates TCP (Transmission Control Protocol) networking in Rust using the standard library.

## Concepts Covered

### TCP Characteristics
- **Connection-oriented**: Three-way handshake establishes connection
- **Reliable**: Guaranteed delivery with acknowledgments
- **Ordered**: Packets arrive in the order sent
- **Stream-based**: Continuous flow of data, not discrete packets
- **Error checking**: Checksum and retransmission

### Implementation
- `TcpListener::bind()` - Create server socket and listen
- `listener.incoming()` - Accept incoming connections
- `TcpStream::connect()` - Connect to server
- `BufReader` - Buffered reading for line-based protocols
- Multi-threaded server with `thread::spawn()`
- Broadcasting to multiple clients with `Arc<Mutex<Vec<TcpStream>>>`

## Running the Example

### Echo Server/Client

**Terminal 1 - Server:**
```bash
cargo run --bin server
```

**Terminal 2 - Client:**
```bash
cargo run --bin client
```

The client enters interactive mode where you can type messages. The server echoes each message back.

### Chat Server

**Terminal 1 - Server:**
```bash
cargo run --bin chat_server
```

**Terminal 2+ - Clients:**
```bash
telnet 127.0.0.1 8080
# OR
cargo run --bin client
```

Messages sent by any client are broadcast to all other connected clients.

## Key Takeaways

1. **Connection required** - Must establish connection before sending data
2. **Reliable delivery** - TCP handles packet loss and reordering
3. **Stream-based** - Data is a continuous stream, not discrete packets
4. **Thread per client** - Simple pattern for handling multiple connections
5. **Resource management** - Close connections properly to avoid leaks

## Use Cases

**TCP is good for:**
- HTTP/HTTPS
- File transfers (FTP, SFTP)
- Email (SMTP, IMAP, POP3)
- SSH
- Database connections
- Any application requiring reliable delivery

**TCP is NOT ideal for:**
- Real-time gaming (too slow)
- Video streaming (retransmission causes lag)
- Simple request/response where UDP suffices

## Common Patterns

### Basic Server
```rust
let listener = TcpListener::bind("127.0.0.1:8080")?;

for stream in listener.incoming() {
    let stream = stream?;
    thread::spawn(|| handle_client(stream));
}
```

### Basic Client
```rust
let mut stream = TcpStream::connect("127.0.0.1:8080")?;
stream.write_all(b"Hello\n")?;

let mut response = String::new();
let mut reader = BufReader::new(stream);
reader.read_line(&mut response)?;
```

### Buffered Reading
```rust
let mut reader = BufReader::new(stream);
let mut line = String::new();

loop {
    line.clear();
    match reader.read_line(&mut line) {
        Ok(0) => break, // EOF
        Ok(_) => process(&line),
        Err(e) => handle_error(e),
    }
}
```

### Broadcasting
```rust
let clients = Arc::new(Mutex::new(Vec::new()));

// Add client
clients.lock().unwrap().push(stream.try_clone()?);

// Broadcast
let message = "Hello all!\n";
let mut clients = clients.lock().unwrap();
clients.retain(|client| {
    client.write_all(message.as_bytes()).is_ok()
});
```

## Error Handling

Common errors:
- `ConnectionRefused`: No server listening
- `ConnectionReset`: Peer closed connection unexpectedly
- `BrokenPipe`: Write to closed connection
- `AddrInUse`: Port already bound
- `WouldBlock`: Non-blocking operation would block

## TCP vs UDP

| Feature | TCP | UDP |
|---------|-----|-----|
| Connection | Yes | No |
| Reliability | Guaranteed | Best effort |
| Ordering | Ordered | Unordered |
| Speed | Slower | Faster |
| Overhead | Higher | Lower |
| Use case | Reliability critical | Speed critical |

## Best Practices

1. **Set timeouts** - Prevent hanging on slow/dead connections
2. **Handle errors gracefully** - Clients can disconnect anytime
3. **Close connections** - Use `drop()` or ensure cleanup
4. **Use threads/async** - Don't block on single client
5. **Buffer I/O** - Use `BufReader`/`BufWriter` for efficiency
6. **Limit resources** - Connection pools, max clients, etc.

## Notes

- TCP guarantees delivery but not timing
- Nagle's algorithm batches small writes (can disable with `set_nodelay`)
- `shutdown()` closes one direction of communication
- `try_clone()` creates independent handle to same connection
- Line-based protocols need explicit newlines (`\n` or `\r\n`)
