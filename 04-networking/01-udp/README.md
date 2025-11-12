# UDP Server and Client

This example demonstrates UDP (User Datagram Protocol) networking in Rust using the standard library.

## Concepts Covered

### UDP Characteristics
- **Connectionless**: No handshake or connection establishment
- **Unreliable**: No guarantee of delivery, ordering, or duplicate protection
- **Fast**: Low overhead, minimal latency
- **Datagram-based**: Messages sent as discrete packets

### Implementation
- `UdpSocket::bind()` - Create and bind socket to address
- `recv_from()` - Receive datagram and sender address
- `send_to()` - Send datagram to specific address
- Timeout handling with `set_read_timeout()`
- Broadcasting with `set_broadcast()`

## Running the Example

### Basic Echo Server/Client

**Terminal 1 - Server:**
```bash
cargo run --bin server
```

**Terminal 2 - Client:**
```bash
cargo run --bin client
```

The client will enter interactive mode where you can type messages to send to the server. The server echoes back each message.

### Broadcast Example

```bash
cargo run --bin broadcast
```

This sends broadcast messages to all devices on the local network.

## Key Takeaways

1. **No connection needed** - Just bind and start sending/receiving
2. **Address with each message** - `recv_from()` returns sender address
3. **Timeouts are important** - Prevent blocking forever
4. **Packet loss is normal** - UDP doesn't guarantee delivery
5. **Size limits** - UDP packets are limited (typically ~65KB)

## Use Cases

**UDP is good for:**
- Real-time applications (VoIP, gaming, streaming)
- DNS queries
- Broadcasting/multicasting
- Applications that can tolerate packet loss

**UDP is NOT good for:**
- File transfers
- Reliable messaging
- Applications requiring ordered delivery

## Common Patterns

### Simple Server
```rust
let socket = UdpSocket::bind("127.0.0.1:8080")?;
let mut buf = [0u8; 1024];

loop {
    let (size, src) = socket.recv_from(&mut buf)?;
    let data = &buf[..size];
    // Process data
    socket.send_to(response, src)?;
}
```

### Simple Client
```rust
let socket = UdpSocket::bind("0.0.0.0:0")?;
socket.send_to(message, "127.0.0.1:8080")?;

let mut buf = [0u8; 1024];
let (size, _) = socket.recv_from(&mut buf)?;
let response = &buf[..size];
```

### With Timeout
```rust
socket.set_read_timeout(Some(Duration::from_secs(5)))?;
match socket.recv_from(&mut buf) {
    Ok((size, src)) => { /* handle data */ }
    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
        // Timeout
    }
    Err(e) => { /* other error */ }
}
```

## Error Handling

Common errors:
- `AddrInUse`: Port already in use
- `WouldBlock`/`TimedOut`: Timeout expired
- `ConnectionRefused`: No process listening (ICMP response)
- `InvalidInput`: Invalid address format

## Notes

- UDP packets can arrive out of order
- Packets may be duplicated
- Packets may be lost without notification
- Maximum safe UDP payload is ~508 bytes (to avoid fragmentation)
- Actual maximum is ~65,507 bytes (65,535 - 8 byte header - 20 byte IP header)
