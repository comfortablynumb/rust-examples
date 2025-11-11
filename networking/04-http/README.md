# HTTP Server and Client

This example demonstrates building HTTP servers and clients from scratch using TCP sockets, showing the fundamentals of the HTTP protocol.

## Concepts Covered

### HTTP Protocol
- **Request/Response**: Client sends request, server sends response
- **Methods**: GET, POST, PUT, DELETE, etc.
- **Status Codes**: 200 OK, 404 Not Found, etc.
- **Headers**: Metadata about request/response
- **Body**: Actual content

### Implementation
- Parsing HTTP requests manually
- Building HTTP responses
- Routing requests to handlers
- Serving static and dynamic content
- JSON APIs
- RESTful design

## Running the Examples

### Basic HTTP Server

**Terminal 1 - Server:**
```bash
cargo run --bin server
```

**Terminal 2 - Test with Browser:**
Open http://127.0.0.1:8000 in your browser

**OR Test with Client:**
```bash
cargo run --bin client
```

**OR Test with curl:**
```bash
curl http://127.0.0.1:8000/
curl http://127.0.0.1:8000/hello
curl http://127.0.0.1:8000/json
```

### REST API Server

**Terminal 1 - API Server:**
```bash
cargo run --bin rest_server
```

**Terminal 2 - Test with curl:**
```bash
# List items
curl http://127.0.0.1:8000/api/items

# Get specific item
curl http://127.0.0.1:8000/api/items/1

# Create item
curl -X POST http://127.0.0.1:8000/api/items

# Delete item
curl -X DELETE http://127.0.0.1:8000/api/items/1
```

## Key Takeaways

1. **HTTP is text-based** - Requests and responses are human-readable
2. **Stateless protocol** - Each request is independent
3. **Request format** - Method + Path + Version + Headers + Body
4. **Response format** - Version + Status + Headers + Body
5. **Content-Type matters** - Tells client how to interpret body

## HTTP Request Format

```
GET /path HTTP/1.1
Host: example.com
User-Agent: Client/1.0
Accept: */*

[optional body]
```

## HTTP Response Format

```
HTTP/1.1 200 OK
Content-Type: text/html
Content-Length: 1234

<html>...</html>
```

## Common Status Codes

| Code | Meaning | Use Case |
|------|---------|----------|
| 200 | OK | Successful request |
| 201 | Created | Resource created |
| 400 | Bad Request | Invalid request |
| 401 | Unauthorized | Authentication required |
| 403 | Forbidden | Not allowed |
| 404 | Not Found | Resource doesn't exist |
| 500 | Internal Server Error | Server error |

## HTTP Methods

| Method | Purpose | Idempotent | Safe |
|--------|---------|------------|------|
| GET | Retrieve resource | Yes | Yes |
| POST | Create resource | No | No |
| PUT | Update/replace | Yes | No |
| PATCH | Partial update | No | No |
| DELETE | Delete resource | Yes | No |
| HEAD | Get headers only | Yes | Yes |
| OPTIONS | Get allowed methods | Yes | Yes |

## RESTful Design

### Resource-Based URLs
- `/api/users` - Collection
- `/api/users/123` - Specific resource
- `/api/users/123/posts` - Sub-resource

### Use HTTP Methods Correctly
- `GET /api/users` - List users
- `POST /api/users` - Create user
- `GET /api/users/123` - Get user
- `PUT /api/users/123` - Update user
- `DELETE /api/users/123` - Delete user

### JSON Response Format
```json
{
  "data": {...},
  "error": null,
  "meta": {
    "timestamp": "2024-01-01T00:00:00Z"
  }
}
```

## Production Considerations

### For Production Use

This example is educational. For production, use:
- **hyper** - Fast, safe HTTP implementation
- **axum** - Web framework built on hyper
- **actix-web** - Actor-based web framework
- **rocket** - Full-featured web framework
- **warp** - Filter-based web framework

### Security Considerations
- **Validate all input** - Prevent injection attacks
- **Use HTTPS** - Encrypt communication
- **Sanitize paths** - Prevent directory traversal
- **Rate limiting** - Prevent abuse
- **CORS headers** - Control cross-origin requests
- **Authentication** - Verify identity
- **Authorization** - Control access

### Performance Optimizations
- **Connection pooling** - Reuse connections
- **Caching** - Store frequently accessed data
- **Compression** - Gzip/Brotli responses
- **HTTP/2** - Multiplexing, server push
- **CDN** - Distribute static content
- **Load balancing** - Distribute requests

## Common Patterns

### Simple Server
```rust
let listener = TcpListener::bind("127.0.0.1:8000")?;

for stream in listener.incoming() {
    let stream = stream?;
    thread::spawn(|| handle_http_request(stream));
}
```

### Request Parsing
```rust
let mut reader = BufReader::new(stream);
let mut request_line = String::new();
reader.read_line(&mut request_line)?;

let parts: Vec<&str> = request_line.split_whitespace().collect();
let method = parts[0];
let path = parts[1];
```

### Response Building
```rust
let response = format!(
    "HTTP/1.1 200 OK\r\n\
     Content-Type: text/html\r\n\
     Content-Length: {}\r\n\
     \r\n\
     {}",
    body.len(),
    body
);
stream.write_all(response.as_bytes())?;
```

### JSON Response
```rust
let json = r#"{"status": "ok", "data": [1, 2, 3]}"#;
let response = format!(
    "HTTP/1.1 200 OK\r\n\
     Content-Type: application/json\r\n\
     Content-Length: {}\r\n\
     \r\n\
     {}",
    json.len(),
    json
);
```

## Testing

### Using curl
```bash
# GET request
curl http://localhost:8000/api/items

# POST with data
curl -X POST -H "Content-Type: application/json" \
  -d '{"name":"New Item"}' \
  http://localhost:8000/api/items

# Custom headers
curl -H "Authorization: Bearer token123" \
  http://localhost:8000/api/protected

# Verbose output
curl -v http://localhost:8000/
```

### Using browsers
- Chrome DevTools (F12) - Network tab
- Firefox Developer Tools - Network tab
- Safari Web Inspector

## Resources

- [HTTP/1.1 RFC](https://tools.ietf.org/html/rfc7230)
- [HTTP/2 RFC](https://tools.ietf.org/html/rfc7540)
- [HTTP Status Codes](https://httpstatuses.com/)
- [REST API Best Practices](https://www.restapitutorial.com/)
- [MDN HTTP Guide](https://developer.mozilla.org/en-US/docs/Web/HTTP)
