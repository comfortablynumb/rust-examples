# Rust Popular Libraries Examples

This directory contains comprehensive examples of the most popular and widely-used Rust crates. These libraries are battle-tested, production-ready, and essential for building modern Rust applications.

## Prerequisites

Before working through these examples, you should be comfortable with:
- Basic Rust syntax and ownership
- Async/await programming
- Traits and generics
- Error handling with Result and Option
- JSON and serialization concepts

## Examples

### [01. Actix Web](01-actix-web/)
A powerful, pragmatic, and extremely fast web framework:
- **RESTful API design** with CRUD operations
- **JSON request/response** handling
- **Custom middleware** for logging and headers
- **Application state** management with Arc<Mutex<T>>
- **Request guards** and extractors
- **WebSocket** routing patterns
- **Error handling** with custom error types
- **Multiple services** and route scopes

### [02. Axum](02-axum/)
A modern, ergonomic web framework built on tokio and tower:
- **Ergonomic routing** with nested routers
- **Type-safe extractors** (Path, Query, State, Json)
- **Middleware** using tower layers
- **Shared state** with Arc<RwLock<T>>
- **Custom error types** implementing IntoResponse
- **Request validation** with custom traits
- **CORS** and logging middleware
- **Static file serving** with ServeDir

### [03. Rocket](03-rocket/)
A web framework with a focus on ease of use and expressiveness:
- **Macro-based routing** (#[get], #[post], etc.)
- **Request guards** for authentication
- **Managed state** for shared data
- **Custom fairings** (middleware)
- **Form handling** with FromForm
- **Path segments** and query parameters
- **Custom error catchers** for all status codes
- **Multiple mount points** for API organization

### [04. Serde](04-serde/)
The de-facto serialization/deserialization framework:
- **JSON, YAML, TOML, CSV** format support
- **Custom serializers/deserializers** with serialize_with/deserialize_with
- **Field attributes** (rename, skip, default, flatten)
- **Enum representations** (externally/internally/adjacently tagged, untagged)
- **Working with Options** and skip_serializing_if
- **Complex nested structures** with validation
- **Error handling** during deserialization
- **Multi-format** data exchange

### [05. Clap](05-clap/)
A powerful command-line argument parser:
- **Derive API** with #[derive(Parser)]
- **Subcommands** and nested subcommands
- **Value enums** with ValueEnum trait
- **Custom validators** for input validation
- **Environment variable** fallback
- **Argument groups** and conflicts
- **Custom help** messages and templates
- **Shell completions** generation

### [06. Reqwest](06-reqwest/)
An ergonomic, async HTTP client:
- **GET, POST, PUT, PATCH, DELETE** requests
- **Query parameters** (multiple methods)
- **Custom headers** and User-Agent
- **Bearer and Basic** authentication
- **JSON serialization** automatic handling
- **Multipart forms** for file uploads
- **Error handling** with status codes
- **Concurrent requests** with tokio::join!
- **Client configuration** and connection pooling
- **Streaming responses** for large data

### [07. SQLx](07-sqlx/)
An async, pure Rust SQL toolkit with compile-time checked queries:
- **Async database operations** with tokio
- **Connection pooling** for efficiency
- **CRUD operations** (Create, Read, Update, Delete)
- **Transactions** for atomic operations
- **Prepared statements** (automatic)
- **Query macros** for compile-time verification
- **Type-safe results** with FromRow
- **Multiple databases** (PostgreSQL, MySQL, SQLite)

## Learning Path

We recommend working through these examples in the following order:

1. **Serde** - Master serialization first, as it's used by most other libraries
2. **Clap** - Build simple CLI tools to practice
3. **Reqwest** - Learn HTTP clients and API consumption
4. **Actix Web or Axum or Rocket** - Choose one web framework to start
   - **Actix Web**: Most mature, highest performance, steeper learning curve
   - **Axum**: Modern, excellent type safety, great tower ecosystem integration
   - **Rocket**: Most beginner-friendly, expressive macro syntax
5. **SQLx** - Add database persistence to your applications

## When to Use Each Library

### Web Frameworks

**Actix Web:**
- Maximum performance required (benchmarks leader)
- Need WebSocket support with actors
- Building microservices at scale
- Comfortable with more boilerplate

**Axum:**
- Want excellent type safety and ergonomics
- Building on tokio/tower ecosystem
- Prefer extractors over macros
- Need middleware composability

**Rocket:**
- Prioritize developer experience
- Want expressive, macro-based routing
- Building smaller web services
- Need quick prototyping

### Data & I/O

**Serde:**
- Any data serialization/deserialization
- Working with JSON APIs
- Configuration file parsing
- Data transformation between formats

**Reqwest:**
- HTTP API consumption
- REST client implementation
- File downloads/uploads
- External service integration

**SQLx:**
- Database access with async support
- Type-safe SQL queries
- Connection pooling needed
- Supporting multiple databases

**Clap:**
- Building CLI applications
- Command-line tool development
- Argument parsing and validation
- Git-like subcommand interfaces

## Running Examples

Each example can be run independently:

```bash
cd libraries/01-actix-web
cargo run
```

Some examples require specific setup:

### Web Framework Examples
Web servers will start on specified ports:
- **Actix Web**: http://localhost:8080
- **Axum**: http://localhost:3000
- **Rocket**: http://localhost:8000

Test them with curl:
```bash
# Actix Web
curl http://localhost:8080/api/todos

# Axum
curl http://localhost:3000/api/products

# Rocket
curl http://localhost:8000/api/books
```

### Reqwest Example
Requires internet connection to access JSONPlaceholder API:
```bash
cd 06-reqwest
cargo run
```

### SQLx Example
Uses in-memory SQLite database (no setup required):
```bash
cd 07-sqlx
cargo run
```

## Common Patterns and Best Practices

### Web Development

**Error Handling:**
```rust
// Custom error type implementing ResponseError/IntoResponse
#[derive(Debug)]
enum ApiError {
    NotFound(String),
    BadRequest(String),
    InternalError(String),
}
```

**State Management:**
```rust
// Thread-safe shared state
#[derive(Clone)]
struct AppState {
    data: Arc<Mutex<HashMap<K, V>>>,
}
```

**Middleware:**
- Keep middleware focused and composable
- Use for cross-cutting concerns (logging, auth, CORS)
- Order matters: logging → auth → routes

### API Design

**RESTful Conventions:**
- `GET /resources` - List all
- `GET /resources/:id` - Get one
- `POST /resources` - Create
- `PUT /resources/:id` - Full update
- `PATCH /resources/:id` - Partial update
- `DELETE /resources/:id` - Delete

**JSON Responses:**
```rust
#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: Option<String>,
}
```

### Database Operations

**Use Transactions:**
```rust
let mut tx = pool.begin().await?;
// Multiple operations...
tx.commit().await?;
```

**Connection Pooling:**
```rust
// Create once, reuse everywhere
let pool = SqlitePool::connect("sqlite:db.db").await?;
```

**Prepared Statements:**
- SQLx automatically prepares and caches statements
- Reuse queries with different parameters

### HTTP Clients

**Client Reuse:**
```rust
// Create once, clone cheaply
let client = Client::builder()
    .timeout(Duration::from_secs(30))
    .build()?;
```

**Error Handling:**
```rust
let response = client.get(url)
    .send()
    .await?
    .error_for_status()?; // Convert 4xx/5xx to errors
```

### Serialization

**Field Attributes:**
```rust
#[derive(Serialize, Deserialize)]
struct Config {
    #[serde(rename = "apiKey")]
    api_key: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    optional: Option<String>,

    #[serde(default)]
    with_default: bool,
}
```

**Custom Serialization:**
```rust
#[serde(serialize_with = "custom_serializer")]
#[serde(deserialize_with = "custom_deserializer")]
field: CustomType,
```

## Testing

All examples include comprehensive tests:

```bash
# Test a specific library example
cd libraries/04-serde
cargo test

# Test all library examples
for dir in libraries/*/; do
    cd "$dir"
    cargo test
    cd -
done
```

## Performance Considerations

### Web Frameworks
- **Actix Web**: Highest throughput, actor-based concurrency
- **Axum**: Excellent performance, lower resource usage than Actix
- **Rocket**: Good performance, slightly higher overhead due to macros

### Database Access
- Use connection pooling (SQLx does this automatically)
- Batch operations in transactions when possible
- Index frequently queried columns

### HTTP Clients
- Reuse Client instances (cheap to clone)
- Use connection pooling (reqwest does this by default)
- Consider timeout values for production

### Serialization
- Serde is zero-cost at runtime (work done at compile time)
- Use `skip_serializing` for fields that don't need serialization
- Consider binary formats (bincode, msgpack) for performance

## Common Errors and Solutions

### "can't find crate for tokio"
- Add `tokio = { version = "1", features = ["full"] }` to Cargo.toml
- Web frameworks and async libraries require an async runtime

### "multiple tokio runtimes detected"
- Use only one `#[tokio::main]` in your application
- Don't mix `#[tokio::main]` with `#[actix_web::main]`

### "connection pool exhausted"
- Increase max_connections in pool configuration
- Ensure connections are properly released
- Check for connection leaks in error paths

### "database is locked (SQLite)"
- SQLite has limited concurrent write support
- Consider PostgreSQL/MySQL for write-heavy workloads
- Use WAL mode: `PRAGMA journal_mode=WAL;`

### "SSL/TLS errors with reqwest"
- Enable TLS features in Cargo.toml
- `reqwest = { version = "0.11", features = ["rustls-tls"] }`

### "serde deserialization failed"
- Check field name matches (case-sensitive)
- Use `#[serde(rename = "...")]` for field mapping
- Verify JSON structure matches struct

## Real-World Applications

These libraries are used together in production for:

### Web Applications
```
Actix/Axum/Rocket + Serde + SQLx
↓
Full-stack web services with database persistence
```

### CLI Tools
```
Clap + Serde + Reqwest
↓
Command-line tools that interact with APIs and files
```

### API Clients
```
Reqwest + Serde
↓
Type-safe API client libraries
```

### Microservices
```
Axum + SQLx + Reqwest + Serde
↓
Service-to-service communication with databases
```

## Further Reading

### Official Documentation

- **Actix Web**: https://actix.rs/
- **Axum**: https://docs.rs/axum/
- **Rocket**: https://rocket.rs/
- **Serde**: https://serde.rs/
- **Clap**: https://docs.rs/clap/
- **Reqwest**: https://docs.rs/reqwest/
- **SQLx**: https://github.com/launchbadge/sqlx

### Guides and Tutorials

- [Actix Web Book](https://actix.rs/docs/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial) - Foundation for async web frameworks
- [Serde Data Model](https://serde.rs/data-model.html)
- [Clap Derive Tutorial](https://docs.rs/clap/latest/clap/_derive/index.html)
- [SQLx Examples](https://github.com/launchbadge/sqlx/tree/main/examples)

### Books

- **Zero To Production In Rust** - Covers Actix Web, SQLx, and more
- **Rust Web Development** - Comprehensive web framework coverage
- **Command-Line Rust** - Deep dive into CLI applications with Clap

## Ecosystem Integration

These libraries work well together:

```rust
// Example: Full-stack API with all libraries
#[tokio::main]
async fn main() {
    // Database (SQLx)
    let pool = SqlitePool::connect("sqlite:db.db").await?;

    // Web server (Axum)
    let app = Router::new()
        .route("/api/data", get(fetch_data))
        .with_state(pool);

    // Listen
    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;
}

// Handler using all libraries
async fn fetch_data(
    State(pool): State<SqlitePool>,
) -> Result<Json<Response>, AppError> {
    // Database query (SQLx)
    let data = sqlx::query_as::<_, Record>("SELECT * FROM table")
        .fetch_all(&pool)
        .await?;

    // External API call (Reqwest)
    let client = reqwest::Client::new();
    let external = client.get("https://api.example.com/data")
        .send()
        .await?
        .json::<ExternalData>()
        .await?;

    // Combine and serialize (Serde)
    Ok(Json(Response {
        internal: data,
        external,
    }))
}
```

## Best Practices Summary

1. **Choose the right tool**
   - Actix for max performance
   - Axum for type safety and ergonomics
   - Rocket for rapid development

2. **Embrace async/await**
   - Use tokio runtime consistently
   - Don't block in async code
   - Use `spawn_blocking` for CPU-intensive work

3. **Handle errors properly**
   - Use custom error types
   - Implement IntoResponse/ResponseError
   - Provide meaningful error messages

4. **Optimize database access**
   - Use connection pooling
   - Batch operations in transactions
   - Index appropriately

5. **Reuse HTTP clients**
   - Create Client once
   - Clone cheaply for concurrent use
   - Configure timeouts

6. **Leverage serde attributes**
   - Use rename for API compatibility
   - Skip unnecessary fields
   - Provide defaults for optional fields

7. **Test thoroughly**
   - Unit tests for business logic
   - Integration tests for APIs
   - Mock external dependencies

## Summary

The libraries category demonstrates essential crates for building production-grade Rust applications:

- **Web Frameworks** - Build REST APIs and web services
- **Serialization** - Handle data in multiple formats
- **HTTP Client** - Consume external APIs
- **Database** - Persist data with type safety
- **CLI** - Create powerful command-line tools

Master these libraries to build:
- High-performance web services
- Robust CLI applications
- Type-safe API clients
- Database-backed applications
- Microservices architectures

These are the building blocks of modern Rust applications used by thousands of production systems worldwide.
