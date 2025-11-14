# QUIC Protocol

Modern transport protocol combining features of TCP and TLS with improved performance.

## Concepts Covered

- QUIC protocol basics
- Connection establishment
- Bidirectional streams
- Multiplexing without head-of-line blocking
- Built-in encryption

## QUIC Advantages

- **Fast Handshake**: 0-RTT connection establishment
- **No Head-of-Line Blocking**: Independent streams
- **Connection Migration**: Survive network changes
- **Built-in Encryption**: TLS 1.3 by default
- **Better Congestion Control**: Modern algorithms

## QUIC vs TCP

| Feature | QUIC | TCP |
|---------|------|-----|
| Handshake | 0-RTT or 1-RTT | 3-way (1-RTT) |
| Streams | Multiple, independent | Single, ordered |
| Encryption | Built-in (TLS 1.3) | Optional (TLS) |
| Head-of-line blocking | No | Yes |
| Connection Migration | Yes | No |

## Key Features

### Multiple Streams

```rust
// Open multiple streams on one connection
let (send1, recv1) = connection.open_bi().await?;
let (send2, recv2) = connection.open_bi().await?;
// Each stream is independent
```

### 0-RTT Resumption

```rust
// Subsequent connections can send data immediately
// without waiting for handshake completion
```

### Connection Migration

```rust
// QUIC connections survive network changes
// (e.g., switching from WiFi to cellular)
```

## Usage

### Server

```rust
let server_config = configure_server()?;
let endpoint = Endpoint::server(server_config, addr)?;

let connection = endpoint.accept().await?.await?;
let (mut send, mut recv) = connection.accept_bi().await?;

// Read and write data
let data = recv.read_to_end(1024).await?;
send.write_all(b"response").await?;
```

### Client

```rust
let mut endpoint = Endpoint::client(bind_addr)?;
let connection = endpoint.connect(server_addr, "hostname")?.await?;

let (mut send, mut recv) = connection.open_bi().await?;
send.write_all(b"request").await?;
let response = recv.read_to_end(1024).await?;
```

## Running

```bash
cargo run
```

## Use Cases

- HTTP/3 (the next version of HTTP)
- Video streaming
- Gaming
- VoIP
- IoT communications
- CDN content delivery

## Performance Characteristics

- **Latency**: Lower connection establishment time
- **Throughput**: Similar to TCP
- **Packet Loss**: Better recovery than TCP
- **Mobile**: Seamless handover between networks

## Security

- **Always Encrypted**: TLS 1.3 mandatory
- **Forward Secrecy**: Key rotation per connection
- **Replay Protection**: Built-in mechanisms

## Best Practices

1. **Use Connection Pooling**: Reuse connections
2. **Set Appropriate Timeouts**: Handle slow networks
3. **Monitor Streams**: Track active streams
4. **Handle Migration**: Implement reconnection logic
5. **Certificate Management**: Proper cert validation in production

## HTTP/3

QUIC is the transport layer for HTTP/3:
- Multiplexed streams
- No head-of-line blocking
- Faster page loads
- Better mobile performance

## References

- [QUIC RFC 9000](https://datatracker.ietf.org/doc/html/rfc9000)
- [Quinn Documentation](https://docs.rs/quinn/)
- [HTTP/3 Explained](https://http3-explained.haxx.se/)
