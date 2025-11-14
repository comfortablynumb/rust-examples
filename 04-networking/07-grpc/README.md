# gRPC

High-performance RPC framework using Protocol Buffers.

## Prerequisites

**IMPORTANT**: This example requires `protoc` (Protocol Buffers compiler) to be installed.

### Installing protoc

```bash
# macOS
brew install protobuf

# Ubuntu/Debian
sudo apt-get install protobuf-compiler

# Or download from: https://github.com/protocolbuffers/protobuf/releases
```

Verify installation:
```bash
protoc --version
```

## Concepts Covered

- Protocol Buffers (protobuf) definition
- gRPC server and client
- Unary RPC calls
- Server streaming
- Code generation from .proto files

## gRPC Benefits

- **Fast**: Binary protocol, efficient serialization
- **Type-safe**: Generated code with strong typing
- **Multi-language**: Interoperable across languages
- **Streaming**: Built-in streaming support
- **HTTP/2**: Multiplexing, flow control, header compression

## RPC Types

1. **Unary**: Single request, single response
2. **Server Streaming**: Single request, stream of responses
3. **Client Streaming**: Stream of requests, single response
4. **Bidirectional Streaming**: Stream both ways

## Protocol Buffers

```protobuf
syntax = "proto3";

service Greeter {
  rpc SayHello (HelloRequest) returns (HelloReply);
}

message HelloRequest {
  string name = 1;
}

message HelloReply {
  string message = 1;
}
```

## Usage

### Server

```rust
#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let reply = HelloReply {
            message: format!("Hello, {}", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

Server::builder()
    .add_service(GreeterServer::new(greeter))
    .serve(addr)
    .await?;
```

### Client

```rust
let mut client = GreeterClient::connect("http://[::1]:50051").await?;

let request = Request::new(HelloRequest {
    name: "Alice".into(),
});

let response = client.say_hello(request).await?;
```

## Running

```bash
cargo run
```

## Use Cases

- Microservices communication
- Internal APIs
- Mobile backends
- Real-time applications
- Cloud-native services

## gRPC vs REST

| Feature | gRPC | REST |
|---------|------|------|
| Protocol | HTTP/2 | HTTP/1.1 |
| Format | Protobuf | JSON |
| Performance | Faster | Slower |
| Streaming | Native | Not native |
| Browser | Limited | Full support |
| Tooling | Code generation | Manual |

## Best Practices

1. **Use streaming** for large datasets
2. **Set deadlines** for requests
3. **Implement retries** with backoff
4. **Use interceptors** for auth/logging
5. **Version your APIs** carefully

## References

- [gRPC Documentation](https://grpc.io/docs/)
- [Tonic](https://docs.rs/tonic/)
- [Protocol Buffers](https://developers.google.com/protocol-buffers)
