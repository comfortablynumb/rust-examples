// API module

use super::database::User;  // Import from sibling module

// Public endpoint functions
pub fn get_users() {
    println!("  GET /api/users");
    let users = fetch_users();
    println!("  Found {} users", users.len());
}

pub fn create_user() {
    println!("  POST /api/users");
    let user = User::new("newuser", "newuser@example.com");
    save_user(user);
}

pub fn update_user(id: u32) {
    println!("  PUT /api/users/{}", id);
}

pub fn delete_user(id: u32) {
    println!("  DELETE /api/users/{}", id);
}

// Private helper functions
fn fetch_users() -> Vec<User> {
    vec![
        User::new("alice", "alice@example.com"),
        User::new("bob", "bob@example.com"),
    ]
}

fn save_user(user: User) {
    println!("  Saving user: {}", user.username());
}

fn validate_request() -> bool {
    true
}

// Request/Response types
pub struct Request {
    method: HttpMethod,
    path: String,
    body: Option<String>,
}

pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

impl Request {
    pub fn new(method: HttpMethod, path: &str) -> Self {
        Request {
            method,
            path: path.to_string(),
            body: None,
        }
    }

    pub fn with_body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }
}

pub struct Response {
    status: u16,
    body: String,
}

impl Response {
    pub fn new(status: u16, body: String) -> Self {
        Response { status, body }
    }

    pub fn ok(body: String) -> Self {
        Response { status: 200, body }
    }

    pub fn not_found() -> Self {
        Response {
            status: 404,
            body: "Not Found".to_string(),
        }
    }
}

// Middleware trait
pub trait Middleware {
    fn before_request(&self, req: &Request);
    fn after_response(&self, res: &Response);
}

// Router
pub struct Router {
    routes: Vec<Route>,
}

struct Route {
    path: String,
    handler: fn() -> Response,
}

impl Router {
    pub fn new() -> Self {
        Router { routes: Vec::new() }
    }

    pub fn add_route(&mut self, path: &str, handler: fn() -> Response) {
        self.routes.push(Route {
            path: path.to_string(),
            handler,
        });
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}
