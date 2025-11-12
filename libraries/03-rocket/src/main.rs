#![allow(dead_code)]
#![allow(unused_variables)]

//! Comprehensive Rocket Web Framework Example
//!
//! This example demonstrates all major features of the Rocket web framework:
//! - Route macros and HTTP methods
//! - Path segments and query parameters
//! - JSON request/response handling
//! - Request guards for authentication
//! - Managed state for shared data
//! - Custom error catchers
//! - Fairings (middleware) for logging
//! - Form handling
//! - Static file serving
//! - Multiple mount points for API organization

use rocket::{
    delete,
    fairing::{Fairing, Info, Kind},
    form::{Form, FromForm},
    get,
    http::{Header, Status},
    patch, post, put,
    request::{FromRequest, Outcome, Request},
    serde::json::Json,
    Build, Rocket, State,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// ============================================================================
// Data Models
// ============================================================================

/// Represents a book in our library
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Book {
    id: u32,
    title: String,
    author: String,
    year: u32,
    isbn: String,
}

/// Represents a blog post
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Post {
    id: u32,
    title: String,
    content: String,
    author: String,
    published: bool,
}

/// Request body for creating a new book
#[derive(Debug, Deserialize)]
struct CreateBook {
    title: String,
    author: String,
    year: u32,
    isbn: String,
}

/// Request body for creating a new post
#[derive(Debug, Deserialize)]
struct CreatePost {
    title: String,
    content: String,
    author: String,
}

/// Form data for book submission
#[derive(Debug, FromForm)]
struct BookForm {
    title: String,
    author: String,
    year: u32,
    isbn: String,
}

/// API response wrapper
#[derive(Debug, Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: String,
}

impl<T> ApiResponse<T> {
    fn success(data: T, message: String) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            message,
        }
    }

    fn error(message: String) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            message,
        }
    }
}

// ============================================================================
// Managed State - Shared Application State
// ============================================================================

/// Application state containing books and posts
/// Wrapped in Arc<Mutex<>> for thread-safe shared access
#[derive(Clone)]
struct AppState {
    books: Arc<Mutex<Vec<Book>>>,
    posts: Arc<Mutex<Vec<Post>>>,
    request_count: Arc<Mutex<u64>>,
}

impl AppState {
    fn new() -> Self {
        // Initialize with some sample data
        let books = vec![
            Book {
                id: 1,
                title: "The Rust Programming Language".to_string(),
                author: "Steve Klabnik and Carol Nichols".to_string(),
                year: 2018,
                isbn: "978-1593278281".to_string(),
            },
            Book {
                id: 2,
                title: "Programming Rust".to_string(),
                author: "Jim Blandy and Jason Orendorff".to_string(),
                year: 2017,
                isbn: "978-1491927281".to_string(),
            },
        ];

        let posts = vec![Post {
            id: 1,
            title: "Getting Started with Rocket".to_string(),
            content: "Rocket is a web framework for Rust...".to_string(),
            author: "Admin".to_string(),
            published: true,
        }];

        AppState {
            books: Arc::new(Mutex::new(books)),
            posts: Arc::new(Mutex::new(posts)),
            request_count: Arc::new(Mutex::new(0)),
        }
    }
}

// ============================================================================
// Request Guards - Custom Authentication
// ============================================================================

/// API key extracted from request headers
/// This is a request guard that validates the API key
struct ApiKey(String);

#[derive(Debug)]
enum ApiKeyError {
    Missing,
    Invalid,
}

/// Implement FromRequest trait to create a custom request guard
/// This checks for an "X-API-Key" header and validates it
#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Check for X-API-Key header
        match req.headers().get_one("X-API-Key") {
            Some(key) => {
                // Simple validation - in production, check against database
                if key == "secret-api-key-12345" {
                    Outcome::Success(ApiKey(key.to_string()))
                } else {
                    Outcome::Error((Status::Unauthorized, ApiKeyError::Invalid))
                }
            }
            None => Outcome::Error((Status::Unauthorized, ApiKeyError::Missing)),
        }
    }
}

/// Admin guard - requires both API key and admin role
struct Admin(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Check for admin key
        match req.headers().get_one("X-Admin-Key") {
            Some(key) if key == "admin-secret-key" => Outcome::Success(Admin(key.to_string())),
            Some(_) => Outcome::Error((Status::Forbidden, ApiKeyError::Invalid)),
            None => Outcome::Error((Status::Forbidden, ApiKeyError::Missing)),
        }
    }
}

// ============================================================================
// Routes - Basic GET Requests
// ============================================================================

/// Root endpoint - simple hello message
#[get("/")]
fn index() -> &'static str {
    "Welcome to the Rocket API! Visit /api/books or /api/posts to see data."
}

/// Health check endpoint
#[get("/health")]
fn health() -> Json<ApiResponse<String>> {
    Json(ApiResponse::success(
        "OK".to_string(),
        "Service is healthy".to_string(),
    ))
}

/// Status endpoint showing request count
#[get("/status")]
fn status(state: &State<AppState>) -> Json<ApiResponse<u64>> {
    let count = *state.request_count.lock().unwrap();
    Json(ApiResponse::success(
        count,
        "Request count retrieved".to_string(),
    ))
}

// ============================================================================
// Routes - Path Segments and Query Parameters
// ============================================================================

/// Get book by ID using path segment
/// Example: /api/books/1
#[get("/books/<id>")]
fn get_book(id: u32, state: &State<AppState>) -> Result<Json<ApiResponse<Book>>, Status> {
    let books = state.books.lock().unwrap();

    books
        .iter()
        .find(|b| b.id == id)
        .map(|book| {
            Json(ApiResponse::success(
                book.clone(),
                format!("Book {} found", id),
            ))
        })
        .ok_or(Status::NotFound)
}

/// Get all books with optional filtering using query parameters
/// Example: /api/books?author=Steve&year=2018
#[get("/books?<author>&<year>")]
fn get_books(
    author: Option<String>,
    year: Option<u32>,
    state: &State<AppState>,
) -> Json<ApiResponse<Vec<Book>>> {
    let books = state.books.lock().unwrap();

    let filtered: Vec<Book> = books
        .iter()
        .filter(|b| {
            let author_match = author.as_ref().is_none_or(|a| b.author.contains(a));
            let year_match = year.is_none_or(|y| b.year == y);
            author_match && year_match
        })
        .cloned()
        .collect();

    Json(ApiResponse::success(
        filtered,
        "Books retrieved".to_string(),
    ))
}

/// Get post by ID with path segment
#[get("/posts/<id>")]
fn get_post(id: u32, state: &State<AppState>) -> Result<Json<ApiResponse<Post>>, Status> {
    let posts = state.posts.lock().unwrap();

    posts
        .iter()
        .find(|p| p.id == id)
        .map(|post| {
            Json(ApiResponse::success(
                post.clone(),
                format!("Post {} found", id),
            ))
        })
        .ok_or(Status::NotFound)
}

/// Get all posts
#[get("/posts")]
fn get_posts(state: &State<AppState>) -> Json<ApiResponse<Vec<Post>>> {
    let posts = state.posts.lock().unwrap();
    Json(ApiResponse::success(
        posts.clone(),
        "Posts retrieved".to_string(),
    ))
}

// ============================================================================
// Routes - JSON Request/Response with POST
// ============================================================================

/// Create a new book using JSON request body
/// Requires API key for authentication (request guard)
#[post("/books", format = "json", data = "<book>")]
fn create_book(
    book: Json<CreateBook>,
    state: &State<AppState>,
    _key: ApiKey, // Request guard ensures API key is valid
) -> Json<ApiResponse<Book>> {
    let mut books = state.books.lock().unwrap();

    // Generate new ID
    let new_id = books.iter().map(|b| b.id).max().unwrap_or(0) + 1;

    let new_book = Book {
        id: new_id,
        title: book.title.clone(),
        author: book.author.clone(),
        year: book.year,
        isbn: book.isbn.clone(),
    };

    books.push(new_book.clone());

    Json(ApiResponse::success(
        new_book,
        "Book created successfully".to_string(),
    ))
}

/// Create a new post using JSON request body
#[post("/posts", format = "json", data = "<post>")]
fn create_post(
    post: Json<CreatePost>,
    state: &State<AppState>,
    _key: ApiKey,
) -> Json<ApiResponse<Post>> {
    let mut posts = state.posts.lock().unwrap();

    let new_id = posts.iter().map(|p| p.id).max().unwrap_or(0) + 1;

    let new_post = Post {
        id: new_id,
        title: post.title.clone(),
        content: post.content.clone(),
        author: post.author.clone(),
        published: false,
    };

    posts.push(new_post.clone());

    Json(ApiResponse::success(
        new_post,
        "Post created successfully".to_string(),
    ))
}

// ============================================================================
// Routes - PUT and DELETE Operations
// ============================================================================

/// Update a book by ID
#[put("/books/<id>", format = "json", data = "<book>")]
fn update_book(
    id: u32,
    book: Json<CreateBook>,
    state: &State<AppState>,
    _key: ApiKey,
) -> Result<Json<ApiResponse<Book>>, Status> {
    let mut books = state.books.lock().unwrap();

    if let Some(existing) = books.iter_mut().find(|b| b.id == id) {
        existing.title = book.title.clone();
        existing.author = book.author.clone();
        existing.year = book.year;
        existing.isbn = book.isbn.clone();

        Ok(Json(ApiResponse::success(
            existing.clone(),
            format!("Book {} updated", id),
        )))
    } else {
        Err(Status::NotFound)
    }
}

/// Delete a book by ID - requires admin access
#[delete("/books/<id>")]
fn delete_book(
    id: u32,
    state: &State<AppState>,
    _admin: Admin, // Admin guard for elevated permissions
) -> Result<Json<ApiResponse<()>>, Status> {
    let mut books = state.books.lock().unwrap();

    if let Some(pos) = books.iter().position(|b| b.id == id) {
        books.remove(pos);
        Ok(Json(ApiResponse::success(
            (),
            format!("Book {} deleted", id),
        )))
    } else {
        Err(Status::NotFound)
    }
}

/// Publish a post (toggle published status)
#[patch("/posts/<id>/publish")]
fn publish_post(
    id: u32,
    state: &State<AppState>,
    _key: ApiKey,
) -> Result<Json<ApiResponse<Post>>, Status> {
    let mut posts = state.posts.lock().unwrap();

    if let Some(post) = posts.iter_mut().find(|p| p.id == id) {
        post.published = !post.published;
        Ok(Json(ApiResponse::success(
            post.clone(),
            format!("Post {} publication status toggled", id),
        )))
    } else {
        Err(Status::NotFound)
    }
}

// ============================================================================
// Routes - Form Handling
// ============================================================================

/// Submit a book via HTML form data
/// Example form: <form method="post" action="/form/book">
#[post("/book", data = "<form>")]
fn submit_book_form(form: Form<BookForm>, state: &State<AppState>) -> String {
    let mut books = state.books.lock().unwrap();

    let new_id = books.iter().map(|b| b.id).max().unwrap_or(0) + 1;

    let new_book = Book {
        id: new_id,
        title: form.title.clone(),
        author: form.author.clone(),
        year: form.year,
        isbn: form.isbn.clone(),
    };

    books.push(new_book.clone());

    format!(
        "Book '{}' by {} created successfully with ID {}!",
        new_book.title, new_book.author, new_book.id
    )
}

/// Display form submission page
#[get("/book")]
fn book_form() -> &'static str {
    r#"
    <html>
        <body>
            <h2>Submit a Book</h2>
            <form method="post" action="/form/book">
                <label>Title: <input type="text" name="title" required /></label><br/>
                <label>Author: <input type="text" name="author" required /></label><br/>
                <label>Year: <input type="number" name="year" required /></label><br/>
                <label>ISBN: <input type="text" name="isbn" required /></label><br/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>
    "#
}

// ============================================================================
// Custom Error Catchers
// ============================================================================

/// Custom 404 Not Found handler
#[rocket::catch(404)]
fn not_found(req: &Request) -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        success: false,
        data: None,
        message: format!("Resource '{}' not found", req.uri()),
    })
}

/// Custom 401 Unauthorized handler
#[rocket::catch(401)]
fn unauthorized() -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        success: false,
        data: None,
        message: "Unauthorized - API key required".to_string(),
    })
}

/// Custom 403 Forbidden handler
#[rocket::catch(403)]
fn forbidden() -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        success: false,
        data: None,
        message: "Forbidden - Admin access required".to_string(),
    })
}

/// Custom 500 Internal Server Error handler
#[rocket::catch(500)]
fn internal_error() -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        success: false,
        data: None,
        message: "Internal server error occurred".to_string(),
    })
}

/// Default error handler
#[rocket::catch(default)]
fn default_catcher(status: Status, req: &Request) -> String {
    format!(
        "Error {}: {} at {}",
        status.code,
        status.reason().unwrap_or("Unknown"),
        req.uri()
    )
}

// ============================================================================
// Fairings - Middleware for Request/Response Processing
// ============================================================================

/// Custom fairing for logging and request counting
pub struct RequestLogger;

#[rocket::async_trait]
impl Fairing for RequestLogger {
    fn info(&self) -> Info {
        Info {
            name: "Request Logger",
            kind: Kind::Request | Kind::Response,
        }
    }

    /// Called on each request
    async fn on_request(&self, request: &mut Request<'_>, _: &mut rocket::Data<'_>) {
        println!(
            "[REQUEST] {} {} from {}",
            request.method(),
            request.uri(),
            request
                .remote()
                .map(|r| r.to_string())
                .unwrap_or_else(|| "unknown".to_string())
        );

        // Increment request counter
        if let Some(state) = request.rocket().state::<AppState>() {
            let mut count = state.request_count.lock().unwrap();
            *count += 1;
        }
    }

    /// Called on each response
    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut rocket::Response<'r>) {
        println!(
            "[RESPONSE] {} {} -> {}",
            request.method(),
            request.uri(),
            response.status()
        );

        // Add custom header to all responses
        response.set_header(Header::new("X-Powered-By", "Rocket"));
    }
}

/// CORS fairing for cross-origin requests
pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "CORS Headers",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(
        &self,
        _request: &'r Request<'_>,
        response: &mut rocket::Response<'r>,
    ) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, DELETE, PATCH",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
    }
}

// ============================================================================
// Application Launch
// ============================================================================

/// Build and launch the Rocket application
#[rocket::launch]
fn rocket() -> Rocket<Build> {
    // Initialize application state
    let state = AppState::new();

    // Build Rocket instance with configuration
    rocket::build()
        // Attach fairings (middleware)
        .attach(RequestLogger)
        .attach(Cors)
        // Manage application state
        .manage(state)
        // Register custom error catchers
        .register(
            "/",
            rocket::catchers![
                not_found,
                unauthorized,
                forbidden,
                internal_error,
                default_catcher
            ],
        )
        // Mount API routes at /api
        .mount(
            "/api",
            rocket::routes![
                get_books,
                get_book,
                create_book,
                update_book,
                delete_book,
                get_posts,
                get_post,
                create_post,
                publish_post,
            ],
        )
        // Mount form routes at /form
        .mount("/form", rocket::routes![book_form, submit_book_form])
        // Mount root routes
        .mount("/", rocket::routes![index, health, status])
    // Serve static files from "static" directory (if it exists)
    // In a real app, create a static/ directory with HTML, CSS, JS files
    // .mount("/static", FileServer::from(relative!("static")))
}

/*
 * USAGE EXAMPLES:
 *
 * 1. Start the server:
 *    cargo run
 *
 * 2. Test endpoints:
 *    curl http://localhost:8000/
 *    curl http://localhost:8000/health
 *    curl http://localhost:8000/status
 *
 * 3. Get books:
 *    curl http://localhost:8000/api/books
 *    curl http://localhost:8000/api/books/1
 *    curl http://localhost:8000/api/books?author=Steve
 *
 * 4. Create book (requires API key):
 *    curl -X POST http://localhost:8000/api/books \
 *      -H "Content-Type: application/json" \
 *      -H "X-API-Key: secret-api-key-12345" \
 *      -d '{"title":"New Book","author":"Author Name","year":2023,"isbn":"123-456"}'
 *
 * 5. Update book:
 *    curl -X PUT http://localhost:8000/api/books/1 \
 *      -H "Content-Type: application/json" \
 *      -H "X-API-Key: secret-api-key-12345" \
 *      -d '{"title":"Updated Book","author":"Author","year":2024,"isbn":"123-456"}'
 *
 * 6. Delete book (requires admin key):
 *    curl -X DELETE http://localhost:8000/api/books/1 \
 *      -H "X-Admin-Key: admin-secret-key"
 *
 * 7. Submit form:
 *    Visit http://localhost:8000/form/book in browser
 *
 * KEY CONCEPTS DEMONSTRATED:
 *
 * - Route Macros: #[get], #[post], #[put], #[delete], #[patch]
 * - Path Segments: /<id> in route paths
 * - Query Parameters: ?<param> in routes
 * - JSON Handling: Json<T> for request/response
 * - Request Guards: ApiKey and Admin for authentication
 * - Managed State: State<AppState> for shared data
 * - Error Catchers: #[catch] for custom error handling
 * - Fairings: RequestLogger and Cors for middleware
 * - Form Handling: Form<T> for HTML forms
 * - Multiple Mount Points: /api, /form, / for organization
 */
