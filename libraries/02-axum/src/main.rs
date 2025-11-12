#![allow(dead_code)]
#![allow(unused_variables)]

//! # Axum Web Framework Example
//!
//! This example demonstrates a comprehensive Axum web application with:
//! - RESTful API routing
//! - JSON request/response handling
//! - Path and query parameter extraction
//! - Shared application state
//! - Custom error handling
//! - Middleware (logging, CORS)
//! - Nested routers
//! - Static file serving
//! - Request validation

use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};

// ============================================================================
// Data Models
// ============================================================================

/// Product model representing an item in our store
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Product {
    id: u32,
    name: String,
    description: String,
    price: f64,
    quantity: u32,
    category: String,
}

/// User model for authentication/authorization demo
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: u32,
    username: String,
    email: String,
    role: UserRole,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum UserRole {
    Admin,
    User,
    Guest,
}

/// Request body for creating a new product
#[derive(Debug, Deserialize)]
struct CreateProductRequest {
    name: String,
    description: String,
    price: f64,
    quantity: u32,
    category: String,
}

/// Request body for updating an existing product
#[derive(Debug, Deserialize)]
struct UpdateProductRequest {
    name: Option<String>,
    description: Option<String>,
    price: Option<f64>,
    quantity: Option<u32>,
    category: Option<String>,
}

/// Query parameters for listing products
#[derive(Debug, Deserialize)]
struct ListProductsQuery {
    category: Option<String>,
    min_price: Option<f64>,
    max_price: Option<f64>,
    limit: Option<usize>,
    offset: Option<usize>,
}

/// Response for product listing with pagination
#[derive(Debug, Serialize)]
struct ProductListResponse {
    products: Vec<Product>,
    total: usize,
    limit: usize,
    offset: usize,
}

/// Generic API response wrapper
#[derive(Debug, Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: Option<String>,
}

impl<T> ApiResponse<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    fn error(message: String) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            message: Some(message),
        }
    }
}

// ============================================================================
// Application State
// ============================================================================

/// Shared application state
/// Using Arc<RwLock<T>> for thread-safe shared mutable state
#[derive(Clone)]
struct AppState {
    products: Arc<RwLock<HashMap<u32, Product>>>,
    users: Arc<RwLock<HashMap<u32, User>>>,
    next_product_id: Arc<RwLock<u32>>,
    next_user_id: Arc<RwLock<u32>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            products: Arc::new(RwLock::new(HashMap::new())),
            users: Arc::new(RwLock::new(HashMap::new())),
            next_product_id: Arc::new(RwLock::new(1)),
            next_user_id: Arc::new(RwLock::new(1)),
        }
    }

    /// Initialize state with sample data
    async fn init_sample_data(&self) {
        let mut products = self.products.write().await;
        let mut next_id = self.next_product_id.write().await;

        let sample_products = vec![
            Product {
                id: 1,
                name: "Laptop".to_string(),
                description: "High-performance laptop".to_string(),
                price: 999.99,
                quantity: 10,
                category: "Electronics".to_string(),
            },
            Product {
                id: 2,
                name: "Mouse".to_string(),
                description: "Wireless optical mouse".to_string(),
                price: 29.99,
                quantity: 50,
                category: "Electronics".to_string(),
            },
            Product {
                id: 3,
                name: "Desk Chair".to_string(),
                description: "Ergonomic office chair".to_string(),
                price: 199.99,
                quantity: 15,
                category: "Furniture".to_string(),
            },
        ];

        for product in sample_products {
            products.insert(product.id, product);
        }

        *next_id = 4;
    }
}

// ============================================================================
// Custom Error Handling
// ============================================================================

/// Custom error type for our application
#[derive(Debug)]
enum AppError {
    NotFound(String),
    BadRequest(String),
    InternalServerError(String),
    Unauthorized(String),
    ValidationError(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "Internal Server Error: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

/// Convert our custom error type into an HTTP response
/// This implementation of IntoResponse is what makes Axum able to
/// return our custom error from handler functions
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let body = Json(ApiResponse::<()>::error(message));
        (status, body).into_response()
    }
}

// ============================================================================
// Request Validation
// ============================================================================

/// Simple validation trait for request validation
trait Validate {
    fn validate(&self) -> Result<(), AppError>;
}

impl Validate for CreateProductRequest {
    fn validate(&self) -> Result<(), AppError> {
        if self.name.is_empty() || self.name.len() > 100 {
            return Err(AppError::ValidationError(
                "Name must be between 1 and 100 characters".to_string(),
            ));
        }
        if self.description.is_empty() || self.description.len() > 500 {
            return Err(AppError::ValidationError(
                "Description must be between 1 and 500 characters".to_string(),
            ));
        }
        if self.price <= 0.0 {
            return Err(AppError::ValidationError(
                "Price must be greater than 0".to_string(),
            ));
        }
        if self.category.is_empty() || self.category.len() > 50 {
            return Err(AppError::ValidationError(
                "Category must be between 1 and 50 characters".to_string(),
            ));
        }
        Ok(())
    }
}

impl Validate for UpdateProductRequest {
    fn validate(&self) -> Result<(), AppError> {
        if let Some(ref name) = self.name {
            if name.is_empty() || name.len() > 100 {
                return Err(AppError::ValidationError(
                    "Name must be between 1 and 100 characters".to_string(),
                ));
            }
        }
        if let Some(ref description) = self.description {
            if description.is_empty() || description.len() > 500 {
                return Err(AppError::ValidationError(
                    "Description must be between 1 and 500 characters".to_string(),
                ));
            }
        }
        if let Some(price) = self.price {
            if price <= 0.0 {
                return Err(AppError::ValidationError(
                    "Price must be greater than 0".to_string(),
                ));
            }
        }
        if let Some(ref category) = self.category {
            if category.is_empty() || category.len() > 50 {
                return Err(AppError::ValidationError(
                    "Category must be between 1 and 50 characters".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// ============================================================================
// Middleware
// ============================================================================

/// Custom logging middleware that logs request method and path
/// Middleware functions in Axum take a request and Next, and return a Response
async fn logging_middleware(req: Request<axum::body::Body>, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();

    // Log the request
    println!("[{}] {} {}", get_current_timestamp(), method, uri);

    // Call the next middleware/handler
    next.run(req).await
}

/// CORS middleware (simplified version)
/// In production, use tower-http's CorsLayer for more features
async fn cors_middleware(req: Request<axum::body::Body>, next: Next) -> Response {
    // Process the request
    let mut response = next.run(req).await;

    // Add CORS headers to the response
    let headers = response.headers_mut();
    headers.insert(
        axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
        "*".parse().unwrap(),
    );
    headers.insert(
        axum::http::header::ACCESS_CONTROL_ALLOW_METHODS,
        "GET, POST, PUT, DELETE, OPTIONS".parse().unwrap(),
    );
    headers.insert(
        axum::http::header::ACCESS_CONTROL_ALLOW_HEADERS,
        "Content-Type, Authorization".parse().unwrap(),
    );

    response
}

/// Authentication middleware example
/// Checks for an "Authorization" header
/// Note: This is a simplified example. In production, you'd validate JWT tokens, etc.
async fn auth_middleware(
    headers: HeaderMap,
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, AppError> {
    // In a real app, you would validate the token here
    match headers.get("Authorization") {
        Some(_token) => {
            // Token exists, allow request to proceed
            Ok(next.run(req).await)
        }
        None => {
            // No token, return unauthorized
            Err(AppError::Unauthorized(
                "Missing authorization header".to_string(),
            ))
        }
    }
}

// ============================================================================
// Handler Functions
// ============================================================================

/// Root handler - returns API information
/// Simple handler that returns JSON without any extractors
async fn root_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "name": "Axum REST API Example",
        "version": "0.1.0",
        "endpoints": {
            "products": "/api/products",
            "users": "/api/users",
            "health": "/health",
        }
    }))
}

/// Health check endpoint
/// Returns server status and current timestamp
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": get_current_timestamp(),
    }))
}

// ============================================================================
// Product Handlers
// ============================================================================

/// List all products with optional filtering and pagination
/// Demonstrates: Query parameters, shared state, JSON response
///
/// Example requests:
/// - GET /api/products
/// - GET /api/products?category=Electronics
/// - GET /api/products?min_price=50&max_price=1000
/// - GET /api/products?limit=5&offset=10
async fn list_products(
    State(state): State<AppState>,
    Query(params): Query<ListProductsQuery>,
) -> Json<ApiResponse<ProductListResponse>> {
    let products = state.products.read().await;

    // Filter products based on query parameters
    let mut filtered: Vec<Product> = products
        .values()
        .filter(|p| {
            // Filter by category if provided
            if let Some(ref cat) = params.category {
                if &p.category != cat {
                    return false;
                }
            }

            // Filter by minimum price if provided
            if let Some(min) = params.min_price {
                if p.price < min {
                    return false;
                }
            }

            // Filter by maximum price if provided
            if let Some(max) = params.max_price {
                if p.price > max {
                    return false;
                }
            }

            true
        })
        .cloned()
        .collect();

    // Sort by ID for consistent ordering
    filtered.sort_by_key(|p| p.id);

    let total = filtered.len();
    let offset = params.offset.unwrap_or(0);
    let limit = params.limit.unwrap_or(10).min(100); // Cap at 100

    // Apply pagination
    let paginated: Vec<Product> = filtered.into_iter().skip(offset).take(limit).collect();

    Json(ApiResponse::success(ProductListResponse {
        products: paginated,
        total,
        limit,
        offset,
    }))
}

/// Get a single product by ID
/// Demonstrates: Path parameter extraction, error handling
///
/// Example: GET /api/products/1
async fn get_product(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<ApiResponse<Product>>, AppError> {
    let products = state.products.read().await;

    products
        .get(&id)
        .cloned()
        .map(|product| Json(ApiResponse::success(product)))
        .ok_or_else(|| AppError::NotFound(format!("Product with id {} not found", id)))
}

/// Create a new product
/// Demonstrates: JSON request body, validation, state mutation
///
/// Example POST /api/products:
/// {
///   "name": "Keyboard",
///   "description": "Mechanical keyboard",
///   "price": 79.99,
///   "quantity": 20,
///   "category": "Electronics"
/// }
async fn create_product(
    State(state): State<AppState>,
    Json(payload): Json<CreateProductRequest>,
) -> Result<Json<ApiResponse<Product>>, AppError> {
    // Validate the request
    payload.validate()?;

    // Get next ID and increment
    let mut next_id = state.next_product_id.write().await;
    let id = *next_id;
    *next_id += 1;
    drop(next_id); // Release the lock

    // Create the product
    let product = Product {
        id,
        name: payload.name,
        description: payload.description,
        price: payload.price,
        quantity: payload.quantity,
        category: payload.category,
    };

    // Store the product
    let mut products = state.products.write().await;
    products.insert(id, product.clone());

    Ok(Json(ApiResponse::success(product)))
}

/// Update an existing product
/// Demonstrates: Path params, JSON body, partial updates
///
/// Example PUT /api/products/1:
/// {
///   "price": 89.99,
///   "quantity": 25
/// }
async fn update_product(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(payload): Json<UpdateProductRequest>,
) -> Result<Json<ApiResponse<Product>>, AppError> {
    // Validate the request
    payload.validate()?;

    let mut products = state.products.write().await;

    let product = products
        .get_mut(&id)
        .ok_or_else(|| AppError::NotFound(format!("Product with id {} not found", id)))?;

    // Apply updates (only update fields that are provided)
    if let Some(name) = payload.name {
        product.name = name;
    }
    if let Some(description) = payload.description {
        product.description = description;
    }
    if let Some(price) = payload.price {
        product.price = price;
    }
    if let Some(quantity) = payload.quantity {
        product.quantity = quantity;
    }
    if let Some(category) = payload.category {
        product.category = category;
    }

    Ok(Json(ApiResponse::success(product.clone())))
}

/// Delete a product
/// Demonstrates: DELETE method, state mutation
///
/// Example: DELETE /api/products/1
async fn delete_product(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let mut products = state.products.write().await;

    products
        .remove(&id)
        .ok_or_else(|| AppError::NotFound(format!("Product with id {} not found", id)))?;

    Ok(Json(ApiResponse {
        success: true,
        data: None,
        message: Some(format!("Product {} deleted successfully", id)),
    }))
}

// ============================================================================
// User Handlers (simplified for demonstration)
// ============================================================================

/// List all users
/// Demonstrates: Simple state read
async fn list_users(State(state): State<AppState>) -> Json<ApiResponse<Vec<User>>> {
    let users = state.users.read().await;
    let user_list: Vec<User> = users.values().cloned().collect();
    Json(ApiResponse::success(user_list))
}

/// Get user by ID
/// Demonstrates: Path extraction with different resource type
async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<ApiResponse<User>>, AppError> {
    let users = state.users.read().await;

    users
        .get(&id)
        .cloned()
        .map(|user| Json(ApiResponse::success(user)))
        .ok_or_else(|| AppError::NotFound(format!("User with id {} not found", id)))
}

// ============================================================================
// Router Configuration
// ============================================================================

/// Create the products router (nested router example)
/// This demonstrates how to organize related endpoints together
fn products_router() -> Router<AppState> {
    Router::new()
        // Route with multiple HTTP methods on root path
        .route("/", get(list_products).post(create_product))
        // Route with path parameter
        .route(
            "/:id",
            get(get_product).put(update_product).delete(delete_product),
        )
}

/// Create the users router (nested router example)
/// Similar structure to products but for user resources
fn users_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users))
        .route("/:id", get(get_user))
}

/// Create the main API router
/// This demonstrates nesting multiple sub-routers under a common prefix
fn api_router() -> Router<AppState> {
    Router::new()
        // Nest products and users routers under /api
        .nest("/products", products_router())
        .nest("/users", users_router())
        // Direct route on /api
        .route("/health", get(health_check))
}

/// Build the complete application with all routes and middleware
/// Demonstrates: Router composition, state injection, middleware layers
fn app(state: AppState) -> Router {
    Router::new()
        // Root endpoint
        .route("/", get(root_handler))
        // Health check at root level
        .route("/health", get(health_check))
        // API routes (nested under /api prefix)
        .nest("/api", api_router())
        // Static file serving example
        // In a real app, create a "static" directory with files
        // This shows how to serve static files like images, CSS, JS
        .nest_service("/static", ServeDir::new("static"))
        // Inject shared state into the router
        // All handlers with State<AppState> will receive this state
        .with_state(state)
        // Add middleware layers
        // Middleware is executed in reverse order (bottom to top)
        // So requests flow: cors -> logging -> tracing -> handlers
        .layer(
            ServiceBuilder::new()
                // Tracing/logging layer from tower-http
                // Provides detailed request/response logging
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().include_headers(true)),
                )
                // Custom logging middleware
                .layer(middleware::from_fn(logging_middleware))
                // CORS middleware
                .layer(middleware::from_fn(cors_middleware)),
        )
}

// ============================================================================
// Utility Functions
// ============================================================================

/// Get current timestamp as a string
/// Simple helper to avoid external dependencies
fn get_current_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    format!("{}.{:03}", duration.as_secs(), duration.subsec_millis())
}

// ============================================================================
// Main Function
// ============================================================================

/// Main entry point for the Axum application
/// Demonstrates: tokio runtime, server setup, application initialization
#[tokio::main]
async fn main() {
    // Initialize tracing for better logging
    // This enables the TraceLayer middleware
    tracing_subscriber::fmt::init();

    println!("Initializing Axum REST API server...");

    // Create and initialize application state
    let state = AppState::new();
    state.init_sample_data().await;

    println!("Sample data initialized");

    // Build the application with routes and middleware
    let app = app(state);

    // Configure the server address
    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    println!("Server listening on http://{}", addr);
    println!("\nAvailable endpoints:");
    println!("  GET    /                    - API information");
    println!("  GET    /health              - Health check");
    println!("  GET    /api/health          - API health check");
    println!("  GET    /api/products        - List products (supports ?category=X&min_price=Y&max_price=Z&limit=N&offset=M)");
    println!("  POST   /api/products        - Create product");
    println!("  GET    /api/products/:id    - Get product by ID");
    println!("  PUT    /api/products/:id    - Update product");
    println!("  DELETE /api/products/:id    - Delete product");
    println!("  GET    /api/users           - List users");
    println!("  GET    /api/users/:id       - Get user by ID");
    println!("  GET    /static/*            - Serve static files");
    println!("\nExample curl commands:");
    println!("  curl http://localhost:3000/");
    println!("  curl http://localhost:3000/api/products");
    println!("  curl http://localhost:3000/api/products/1");
    println!("  curl -X POST http://localhost:3000/api/products \\");
    println!("    -H 'Content-Type: application/json' \\");
    println!("    -d '{{\"name\":\"Test\",\"description\":\"Test product\",\"price\":19.99,\"quantity\":10,\"category\":\"Test\"}}'");
    println!("\nPress Ctrl+C to stop the server\n");

    // Start the server
    // This runs the server and blocks until it's shut down
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

// ============================================================================
// Tests Module
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Test state initialization
    #[tokio::test]
    async fn test_state_initialization() {
        let state = AppState::new();
        state.init_sample_data().await;

        let products = state.products.read().await;
        assert_eq!(products.len(), 3);
        assert!(products.contains_key(&1));
        assert!(products.contains_key(&2));
        assert!(products.contains_key(&3));
    }

    /// Test product validation - valid product
    #[test]
    fn test_product_validation_valid() {
        let valid_request = CreateProductRequest {
            name: "Test Product".to_string(),
            description: "Test Description".to_string(),
            price: 10.0,
            quantity: 5,
            category: "Test".to_string(),
        };
        assert!(valid_request.validate().is_ok());
    }

    /// Test product validation - invalid name
    #[test]
    fn test_product_validation_invalid_name() {
        let invalid_request = CreateProductRequest {
            name: "".to_string(),
            description: "Test".to_string(),
            price: 10.0,
            quantity: 0,
            category: "Test".to_string(),
        };
        assert!(invalid_request.validate().is_err());
    }

    /// Test product validation - invalid price
    #[test]
    fn test_product_validation_invalid_price() {
        let invalid_request = CreateProductRequest {
            name: "Test".to_string(),
            description: "Test".to_string(),
            price: -5.0,
            quantity: 0,
            category: "Test".to_string(),
        };
        assert!(invalid_request.validate().is_err());
    }

    /// Test API response construction
    #[test]
    fn test_api_response() {
        let success_response = ApiResponse::success("test data");
        assert!(success_response.success);
        assert!(success_response.data.is_some());
        assert!(success_response.message.is_none());

        let error_response = ApiResponse::<()>::error("error message".to_string());
        assert!(!error_response.success);
        assert!(error_response.data.is_none());
        assert!(error_response.message.is_some());
    }
}
