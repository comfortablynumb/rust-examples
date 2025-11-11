#![allow(dead_code)]
#![allow(unused_variables)]

//! Comprehensive Actix Web Example
//!
//! This example demonstrates:
//! - RESTful API design with CRUD operations
//! - Path parameters and query strings
//! - JSON request/response handling
//! - Custom middleware
//! - Application state management
//! - Custom error handling
//! - Static file serving
//! - WebSocket connections
//! - Request guards and extractors
//! - Multiple services and scopes

use actix_web::http::header::HeaderValue;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error, get, middleware, post, put, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Result,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::{ready, Ready};
use std::pin::Pin;
use std::sync::{Arc, Mutex};

// ============================================================================
// Data Models
// ============================================================================

/// Todo item representation
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    id: u32,
    title: String,
    description: String,
    completed: bool,
}

/// Request body for creating a new todo
#[derive(Debug, Serialize, Deserialize)]
struct CreateTodo {
    title: String,
    description: String,
}

/// Request body for updating a todo
#[derive(Debug, Deserialize)]
struct UpdateTodo {
    title: Option<String>,
    description: Option<String>,
    completed: Option<bool>,
}

/// Query parameters for filtering todos
#[derive(Debug, Deserialize)]
struct TodoQuery {
    completed: Option<bool>,
    search: Option<String>,
}

/// User information for demonstrating extractors
#[derive(Debug, Deserialize)]
struct UserInfo {
    username: String,
    email: String,
}

// ============================================================================
// Application State
// ============================================================================

/// Shared application state
/// In a real application, this would use a database
#[derive(Clone)]
struct AppState {
    todos: Arc<Mutex<HashMap<u32, Todo>>>,
    counter: Arc<Mutex<u32>>,
}

impl AppState {
    fn new() -> Self {
        let mut todos = HashMap::new();

        // Add some sample data
        todos.insert(
            1,
            Todo {
                id: 1,
                title: "Learn Rust".to_string(),
                description: "Master the Rust programming language".to_string(),
                completed: false,
            },
        );
        todos.insert(
            2,
            Todo {
                id: 2,
                title: "Build web app".to_string(),
                description: "Create an Actix Web application".to_string(),
                completed: false,
            },
        );

        AppState {
            todos: Arc::new(Mutex::new(todos)),
            counter: Arc::new(Mutex::new(2)), // Start from 2 since we have 2 todos
        }
    }

    fn get_next_id(&self) -> u32 {
        let mut counter = self.counter.lock().unwrap();
        *counter += 1;
        *counter
    }
}

// ============================================================================
// Custom Error Handling
// ============================================================================

/// Custom error types for the application
#[derive(Debug)]
enum ApiError {
    NotFound(String),
    BadRequest(String),
    InternalError(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            ApiError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            ApiError::InternalError(msg) => write!(f, "Internal Error: {}", msg),
        }
    }
}

impl error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::NotFound(msg) => HttpResponse::NotFound().json(serde_json::json!({
                "error": "not_found",
                "message": msg
            })),
            ApiError::BadRequest(msg) => HttpResponse::BadRequest().json(serde_json::json!({
                "error": "bad_request",
                "message": msg
            })),
            ApiError::InternalError(msg) => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "internal_error",
                    "message": msg
                }))
            }
        }
    }
}

// ============================================================================
// Custom Middleware
// ============================================================================

/// Middleware for logging requests
pub struct RequestLogger;

impl<S, B> Transform<S, ServiceRequest> for RequestLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequestLoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestLoggerMiddleware { service }))
    }
}

pub struct RequestLoggerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestLoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("🔍 {} {}", req.method(), req.path());

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            println!("✅ Response status: {}", res.status());
            Ok(res)
        })
    }
}

/// Middleware for adding custom headers
pub struct CustomHeader;

impl<S, B> Transform<S, ServiceRequest> for CustomHeader
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CustomHeaderMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CustomHeaderMiddleware { service }))
    }
}

pub struct CustomHeaderMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CustomHeaderMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;
            res.headers_mut().insert(
                actix_web::http::header::HeaderName::from_static("x-custom-header"),
                HeaderValue::from_static("actix-web-example"),
            );
            Ok(res)
        })
    }
}

// ============================================================================
// Request Guards and Extractors
// ============================================================================

/// Custom extractor that validates API key
struct ApiKey(String);

impl actix_web::FromRequest for ApiKey {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let api_key = req
            .headers()
            .get("X-API-KEY")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("");

        if api_key.is_empty() {
            ready(Err(error::ErrorUnauthorized("Missing API key")))
        } else if api_key != "secret-key" {
            ready(Err(error::ErrorUnauthorized("Invalid API key")))
        } else {
            ready(Ok(ApiKey(api_key.to_string())))
        }
    }
}

// ============================================================================
// Route Handlers
// ============================================================================

/// Basic GET handler - returns welcome message
#[get("/")]
async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Welcome to Actix Web API",
        "version": "1.0.0",
        "endpoints": {
            "todos": "/api/todos",
            "health": "/health",
            "websocket": "/ws",
            "protected": "/api/protected"
        }
    })))
}

/// Health check endpoint
#[get("/health")]
async fn health_check() -> Result<HttpResponse> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": timestamp
    })))
}

/// GET all todos with optional filtering via query parameters
#[get("/todos")]
async fn get_todos(
    state: web::Data<AppState>,
    query: web::Query<TodoQuery>,
) -> Result<HttpResponse> {
    let todos = state.todos.lock().unwrap();
    let mut result: Vec<Todo> = todos.values().cloned().collect();

    // Filter by completed status if specified
    if let Some(completed) = query.completed {
        result.retain(|todo| todo.completed == completed);
    }

    // Search in title or description if specified
    if let Some(search) = &query.search {
        let search_lower = search.to_lowercase();
        result.retain(|todo| {
            todo.title.to_lowercase().contains(&search_lower)
                || todo.description.to_lowercase().contains(&search_lower)
        });
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "todos": result,
        "count": result.len()
    })))
}

/// GET a specific todo by ID (path parameter example)
#[get("/todos/{id}")]
async fn get_todo(
    state: web::Data<AppState>,
    path: web::Path<u32>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let todos = state.todos.lock().unwrap();

    match todos.get(&id) {
        Some(todo) => Ok(HttpResponse::Ok().json(todo)),
        None => Err(ApiError::NotFound(format!("Todo with id {} not found", id))),
    }
}

/// POST - Create a new todo (JSON request body example)
#[post("/todos")]
async fn create_todo(
    state: web::Data<AppState>,
    todo_data: web::Json<CreateTodo>,
) -> Result<HttpResponse, ApiError> {
    if todo_data.title.is_empty() {
        return Err(ApiError::BadRequest("Title cannot be empty".to_string()));
    }

    let new_id = state.get_next_id();
    let new_todo = Todo {
        id: new_id,
        title: todo_data.title.clone(),
        description: todo_data.description.clone(),
        completed: false,
    };

    let mut todos = state.todos.lock().unwrap();
    todos.insert(new_id, new_todo.clone());

    Ok(HttpResponse::Created().json(new_todo))
}

/// PUT - Update an existing todo
#[put("/todos/{id}")]
async fn update_todo(
    state: web::Data<AppState>,
    path: web::Path<u32>,
    update_data: web::Json<UpdateTodo>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let mut todos = state.todos.lock().unwrap();

    match todos.get_mut(&id) {
        Some(todo) => {
            if let Some(title) = &update_data.title {
                todo.title = title.clone();
            }
            if let Some(description) = &update_data.description {
                todo.description = description.clone();
            }
            if let Some(completed) = update_data.completed {
                todo.completed = completed;
            }

            Ok(HttpResponse::Ok().json(todo.clone()))
        }
        None => Err(ApiError::NotFound(format!("Todo with id {} not found", id))),
    }
}

/// DELETE - Remove a todo
async fn delete_todo(
    state: web::Data<AppState>,
    path: web::Path<u32>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let mut todos = state.todos.lock().unwrap();

    match todos.remove(&id) {
        Some(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": format!("Todo {} deleted successfully", id)
        }))),
        None => Err(ApiError::NotFound(format!("Todo with id {} not found", id))),
    }
}

/// Mark todo as completed (custom action endpoint)
#[put("/todos/{id}/complete")]
async fn complete_todo(
    state: web::Data<AppState>,
    path: web::Path<u32>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let mut todos = state.todos.lock().unwrap();

    match todos.get_mut(&id) {
        Some(todo) => {
            todo.completed = true;
            Ok(HttpResponse::Ok().json(todo.clone()))
        }
        None => Err(ApiError::NotFound(format!("Todo with id {} not found", id))),
    }
}

/// Protected endpoint demonstrating custom extractor (API key validation)
#[get("/protected")]
async fn protected_route(_api_key: ApiKey) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "You have access to protected resources!",
        "data": "This is sensitive information"
    })))
}

/// Example of multiple extractors in one handler
#[post("/users")]
async fn create_user(user_info: web::Json<UserInfo>, req: HttpRequest) -> Result<HttpResponse> {
    // Access request headers
    let user_agent = req
        .headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown");

    Ok(HttpResponse::Created().json(serde_json::json!({
        "message": "User created successfully",
        "username": user_info.username,
        "email": user_info.email,
        "user_agent": user_agent
    })))
}

/// WebSocket handler
async fn websocket_route(req: HttpRequest, _stream: web::Payload) -> Result<HttpResponse, Error> {
    // In a real application, you would use actix-web-actors for WebSocket support
    // This is a simplified example showing the concept

    println!(
        "WebSocket connection request from: {:?}",
        req.connection_info()
    );

    // For demonstration, we'll return a message about WebSocket support
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "WebSocket endpoint",
        "note": "To implement full WebSocket support, add 'actix-web-actors' dependency",
        "example": "This demonstrates the routing structure for WebSocket connections"
    })))
}

/// Static file serving example
async fn serve_static() -> Result<HttpResponse> {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Actix Web Example</title>
        <style>
            body {
                font-family: Arial, sans-serif;
                max-width: 800px;
                margin: 50px auto;
                padding: 20px;
                background-color: #f5f5f5;
            }
            h1 { color: #333; }
            .info { background: white; padding: 20px; border-radius: 5px; }
            code { background: #eee; padding: 2px 5px; border-radius: 3px; }
        </style>
    </head>
    <body>
        <h1>Actix Web Example Application</h1>
        <div class="info">
            <h2>Available Endpoints:</h2>
            <ul>
                <li><code>GET /</code> - API information</li>
                <li><code>GET /health</code> - Health check</li>
                <li><code>GET /api/todos</code> - List all todos</li>
                <li><code>GET /api/todos/{id}</code> - Get specific todo</li>
                <li><code>POST /api/todos</code> - Create new todo</li>
                <li><code>PUT /api/todos/{id}</code> - Update todo</li>
                <li><code>DELETE /api/todos/{id}</code> - Delete todo</li>
                <li><code>PUT /api/todos/{id}/complete</code> - Mark as complete</li>
                <li><code>GET /api/protected</code> - Protected endpoint (requires X-API-KEY header)</li>
            </ul>
        </div>
    </body>
    </html>
    "#;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

/// Advanced route with multiple path parameters
#[get("/categories/{category}/items/{item_id}")]
async fn nested_resource(path: web::Path<(String, u32)>) -> Result<HttpResponse> {
    let (category, item_id) = path.into_inner();

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "category": category,
        "item_id": item_id,
        "message": format!("Item {} in category {}", item_id, category)
    })))
}

/// Example of manual response building
async fn custom_response() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .insert_header(("X-Custom-Header", "custom-value"))
        .insert_header(("X-Request-Id", "12345"))
        .body(r#"{"message": "Custom response with headers"}"#))
}

// ============================================================================
// Application Configuration
// ============================================================================

/// Configure API routes
fn config_api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .wrap(RequestLogger)
            .wrap(CustomHeader)
            .service(get_todos)
            .service(get_todo)
            .service(create_todo)
            .service(update_todo)
            .service(complete_todo)
            .service(protected_route)
            .route("/todos/{id}", web::delete().to(delete_todo)),
    );
}

/// Configure additional routes (demonstrating multiple scopes)
fn config_extra(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/extra")
            .service(nested_resource)
            .route("/custom", web::get().to(custom_response))
            .service(create_user),
    );
}

// ============================================================================
// Main Application
// ============================================================================

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Starting Actix Web server...");
    println!("📍 Server running at: http://localhost:8080");
    println!("📚 API Documentation at: http://localhost:8080/docs");
    println!("🔍 Try: curl http://localhost:8080/");
    println!("🔍 Try: curl http://localhost:8080/api/todos");
    println!("🔍 Try: curl -X POST http://localhost:8080/api/todos \\");
    println!("         -H 'Content-Type: application/json' \\");
    println!("         -d '{{\"title\":\"New Task\",\"description\":\"Task description\"}}'");
    println!("🔐 Protected: curl -H 'X-API-KEY: secret-key' http://localhost:8080/api/protected");
    println!();

    // Initialize application state
    let app_state = web::Data::new(AppState::new());

    // Create and run HTTP server
    HttpServer::new(move || {
        App::new()
            // Register application state
            .app_data(app_state.clone())
            // Add built-in middleware
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            // Register routes and scopes
            .service(index)
            .service(health_check)
            .configure(config_api)
            .configure(config_extra)
            .route("/ws", web::get().to(websocket_route))
            .route("/docs", web::get().to(serve_static))
            // Default 404 handler
            .default_service(web::route().to(|| async {
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": "not_found",
                    "message": "The requested resource was not found"
                }))
            }))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_index() {
        let app = test::init_service(App::new().service(index)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_health_check() {
        let app = test::init_service(App::new().service(health_check)).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_todos() {
        let app_state = web::Data::new(AppState::new());
        let app =
            test::init_service(App::new().app_data(app_state.clone()).service(get_todos)).await;

        let req = test::TestRequest::get().uri("/todos").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_create_todo() {
        let app_state = web::Data::new(AppState::new());
        let app =
            test::init_service(App::new().app_data(app_state.clone()).service(create_todo)).await;

        let req = test::TestRequest::post()
            .uri("/todos")
            .set_json(&CreateTodo {
                title: "Test Todo".to_string(),
                description: "Test Description".to_string(),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::CREATED);
    }
}
