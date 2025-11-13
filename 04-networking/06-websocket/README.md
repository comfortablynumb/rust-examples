# WebSocket

Real-time bidirectional communication over a single TCP connection.

## Concepts Covered

- WebSocket protocol
- Full-duplex communication
- Text and binary messages
- Ping/pong heartbeats
- Connection lifecycle

## WebSocket vs HTTP

| Feature | HTTP | WebSocket |
|---------|------|-----------|
| Direction | Request-Response | Bidirectional |
| Connection | Short-lived | Long-lived |
| Overhead | High (headers) | Low (after handshake) |
| Real-time | Polling required | Native |

## Message Types

- **Text**: UTF-8 encoded strings
- **Binary**: Raw byte data
- **Ping/Pong**: Keepalive mechanism
- **Close**: Graceful connection termination

## Usage

```rust
// Server
let listener = TcpListener::bind("127.0.0.1:9001").await?;
let (stream, _) = listener.accept().await?;
let ws_stream = accept_async(stream).await?;

// Client
let (ws_stream, _) = connect_async("ws://127.0.0.1:9001").await?;

// Send/Receive
let (mut write, mut read) = ws_stream.split();
write.send(Message::Text("Hello".into())).await?;
let msg = read.next().await;
```

## Use Cases

- Chat applications
- Live notifications
- Real-time dashboards
- Multiplayer games
- Collaborative editing
- Stock tickers
- IoT device communication

## Running

```bash
cargo run
```

## Best Practices

1. **Heartbeats**: Use ping/pong to detect dead connections
2. **Reconnection**: Implement automatic reconnection logic
3. **Message Queue**: Buffer messages during disconnections
4. **Rate Limiting**: Prevent message flooding
5. **Authentication**: Verify clients during handshake

## References

- [WebSocket RFC](https://datatracker.ietf.org/doc/html/rfc6455)
- [tokio-tungstenite](https://docs.rs/tokio-tungstenite/)
