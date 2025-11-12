#![allow(dead_code)]
#![allow(unused_variables)]

//! Comprehensive Reqwest HTTP Client Example
//!
//! This example demonstrates various HTTP client operations using the reqwest library.
//! Reqwest is a high-level HTTP client that provides both async and blocking APIs.
//!
//! Features covered:
//! - Basic GET, POST, PUT, PATCH, DELETE requests
//! - Query parameters and custom headers
//! - JSON serialization/deserialization
//! - Authentication (Bearer tokens)
//! - File uploads with multipart forms
//! - Response handling and error management
//! - Async request patterns

use reqwest::{multipart, Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

// ============================================================================
// Data Structures for JSONPlaceholder API
// ============================================================================

/// Represents a post from JSONPlaceholder API
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Post {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<u32>,
    #[serde(rename = "userId")]
    user_id: u32,
    title: String,
    body: String,
}

/// Represents a user from JSONPlaceholder API
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    username: String,
    email: String,
    phone: String,
    website: String,
}

/// Represents a comment from JSONPlaceholder API
#[derive(Debug, Serialize, Deserialize)]
struct Comment {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<u32>,
    #[serde(rename = "postId")]
    post_id: u32,
    name: String,
    email: String,
    body: String,
}

/// Custom API response wrapper
#[derive(Debug, Deserialize)]
struct ApiResponse {
    success: bool,
    message: String,
}

// ============================================================================
// Basic GET Requests
// ============================================================================

/// Demonstrates a simple GET request
///
/// This is the most basic HTTP operation - fetching a resource from a server.
/// The response is converted to JSON and deserialized into a Rust struct.
async fn basic_get_request() -> Result<(), Box<dyn Error>> {
    println!("\n=== Basic GET Request ===");

    // Create a client - this can be reused for multiple requests
    let client = Client::new();

    // Make a GET request to fetch a single post
    let response = client
        .get("https://jsonplaceholder.typicode.com/posts/1")
        .send()
        .await?;

    // Check the status code
    println!("Status: {}", response.status());

    // Parse the JSON response into our Post struct
    let post: Post = response.json().await?;
    println!("Post: {:?}", post);

    Ok(())
}

/// Demonstrates GET request with response handling
///
/// Shows how to access different parts of the HTTP response including
/// status code, headers, and body in various formats.
async fn get_with_response_handling() -> Result<(), Box<dyn Error>> {
    println!("\n=== GET with Response Handling ===");

    let client = Client::new();
    let response = client
        .get("https://jsonplaceholder.typicode.com/users/1")
        .send()
        .await?;

    // Access response status
    let status = response.status();
    println!("Status Code: {}", status);
    println!("Status is success: {}", status.is_success());

    // Access response headers
    println!("\nResponse Headers:");
    for (name, value) in response.headers() {
        println!("  {}: {:?}", name, value);
    }

    // Get content type
    if let Some(content_type) = response.headers().get("content-type") {
        println!("\nContent-Type: {:?}", content_type);
    }

    // Parse body as JSON
    let user: User = response.json().await?;
    println!("\nUser: {:#?}", user);

    Ok(())
}

/// Demonstrates fetching multiple resources
async fn get_multiple_resources() -> Result<(), Box<dyn Error>> {
    println!("\n=== GET Multiple Resources ===");

    let client = Client::new();

    // Fetch all posts (returns an array)
    let posts: Vec<Post> = client
        .get("https://jsonplaceholder.typicode.com/posts")
        .send()
        .await?
        .json()
        .await?;

    println!("Fetched {} posts", posts.len());
    println!("First 3 posts:");
    for post in posts.iter().take(3) {
        println!("  - [{}] {}", post.id.unwrap_or(0), post.title);
    }

    Ok(())
}

// ============================================================================
// Query Parameters
// ============================================================================

/// Demonstrates adding query parameters to requests
///
/// Query parameters are key-value pairs appended to the URL after a '?'.
/// Reqwest provides multiple ways to add them.
async fn get_with_query_params() -> Result<(), Box<dyn Error>> {
    println!("\n=== GET with Query Parameters ===");

    let client = Client::new();

    // Method 1: Using query() with a slice of tuples
    let response1 = client
        .get("https://jsonplaceholder.typicode.com/posts")
        .query(&[("userId", "1"), ("_limit", "5")])
        .send()
        .await?;

    let posts1: Vec<Post> = response1.json().await?;
    println!(
        "Method 1 - Posts by user 1 (limited to 5): {}",
        posts1.len()
    );

    // Method 2: Using query() with a HashMap
    let mut params = HashMap::new();
    params.insert("postId", "1");

    let response2 = client
        .get("https://jsonplaceholder.typicode.com/comments")
        .query(&params)
        .send()
        .await?;

    let comments: Vec<Comment> = response2.json().await?;
    println!("Method 2 - Comments for post 1: {}", comments.len());

    // Method 3: Using a serializable struct (with serde)
    #[derive(Serialize)]
    struct QueryParams {
        #[serde(rename = "userId")]
        user_id: u32,
        _limit: u32,
    }

    let params = QueryParams {
        user_id: 2,
        _limit: 3,
    };

    let response3 = client
        .get("https://jsonplaceholder.typicode.com/posts")
        .query(&params)
        .send()
        .await?;

    let posts2: Vec<Post> = response3.json().await?;
    println!(
        "Method 3 - Posts by user 2 (limited to 3): {}",
        posts2.len()
    );

    Ok(())
}

// ============================================================================
// Custom Headers
// ============================================================================

/// Demonstrates adding custom headers to requests
///
/// Headers provide metadata about the request or client.
/// Common headers include Content-Type, Accept, User-Agent, etc.
async fn get_with_custom_headers() -> Result<(), Box<dyn Error>> {
    println!("\n=== GET with Custom Headers ===");

    let client = Client::new();

    let response = client
        .get("https://jsonplaceholder.typicode.com/posts/1")
        .header("User-Agent", "Rust-Reqwest-Example/1.0")
        .header("Accept", "application/json")
        .header("X-Custom-Header", "custom-value")
        .send()
        .await?;

    println!("Status: {}", response.status());

    // The request headers we sent aren't in the response, but we can verify
    // the server accepted our request
    let post: Post = response.json().await?;
    println!("Received post: {}", post.title);

    Ok(())
}

// ============================================================================
// Authentication
// ============================================================================

/// Demonstrates Bearer token authentication
///
/// Bearer tokens are commonly used for API authentication.
/// The token is sent in the Authorization header.
async fn get_with_bearer_auth() -> Result<(), Box<dyn Error>> {
    println!("\n=== GET with Bearer Token Authentication ===");

    let client = Client::new();

    // JSONPlaceholder doesn't actually require authentication,
    // but this shows the pattern for APIs that do
    let token = "your-secret-bearer-token-here";

    let response = client
        .get("https://jsonplaceholder.typicode.com/posts/1")
        .bearer_auth(token)
        .send()
        .await?;

    println!("Status: {}", response.status());

    // Alternative: manually set the Authorization header
    let response2 = client
        .get("https://jsonplaceholder.typicode.com/posts/1")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    println!("Alternative method status: {}", response2.status());

    Ok(())
}

/// Demonstrates Basic authentication
async fn get_with_basic_auth() -> Result<(), Box<dyn Error>> {
    println!("\n=== GET with Basic Authentication ===");

    let client = Client::new();

    // Basic auth: username and password
    let response = client
        .get("https://jsonplaceholder.typicode.com/posts/1")
        .basic_auth("username", Some("password"))
        .send()
        .await?;

    println!("Status: {}", response.status());

    Ok(())
}

// ============================================================================
// POST Requests
// ============================================================================

/// Demonstrates POST request with JSON body
///
/// POST requests are used to create new resources.
/// The json() method automatically serializes the struct and sets Content-Type.
async fn post_with_json() -> Result<(), Box<dyn Error>> {
    println!("\n=== POST with JSON Body ===");

    let client = Client::new();

    // Create a new post
    let new_post = Post {
        id: None,
        user_id: 1,
        title: "My New Post".to_string(),
        body: "This is the content of my new post.".to_string(),
    };

    println!("Sending POST request with: {:#?}", new_post);

    let response = client
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&new_post)
        .send()
        .await?;

    println!("Status: {}", response.status());

    // The API returns the created post with an ID
    let created_post: Post = response.json().await?;
    println!("Created post: {:#?}", created_post);

    Ok(())
}

/// Demonstrates POST with form data
async fn post_with_form() -> Result<(), Box<dyn Error>> {
    println!("\n=== POST with Form Data ===");

    let client = Client::new();

    // Create form data
    let params = [
        ("title", "Form Post"),
        ("body", "Posted via form data"),
        ("userId", "1"),
    ];

    let response = client
        .post("https://jsonplaceholder.typicode.com/posts")
        .form(&params)
        .send()
        .await?;

    println!("Status: {}", response.status());
    let text = response.text().await?;
    println!("Response: {}", text);

    Ok(())
}

// ============================================================================
// PUT and PATCH Requests
// ============================================================================

/// Demonstrates PUT request (full update)
///
/// PUT requests replace the entire resource.
/// All fields should be provided.
async fn put_request() -> Result<(), Box<dyn Error>> {
    println!("\n=== PUT Request (Full Update) ===");

    let client = Client::new();

    let updated_post = Post {
        id: Some(1),
        user_id: 1,
        title: "Updated Title".to_string(),
        body: "Completely updated content.".to_string(),
    };

    let response = client
        .put("https://jsonplaceholder.typicode.com/posts/1")
        .json(&updated_post)
        .send()
        .await?;

    println!("Status: {}", response.status());

    let result: Post = response.json().await?;
    println!("Updated post: {:#?}", result);

    Ok(())
}

/// Demonstrates PATCH request (partial update)
///
/// PATCH requests update only specific fields.
/// Only the fields to be updated need to be sent.
async fn patch_request() -> Result<(), Box<dyn Error>> {
    println!("\n=== PATCH Request (Partial Update) ===");

    let client = Client::new();

    // Only update the title
    let partial_update = serde_json::json!({
        "title": "Patched Title"
    });

    let response = client
        .patch("https://jsonplaceholder.typicode.com/posts/1")
        .json(&partial_update)
        .send()
        .await?;

    println!("Status: {}", response.status());

    let result: Post = response.json().await?;
    println!("Patched post: {:#?}", result);

    Ok(())
}

// ============================================================================
// DELETE Requests
// ============================================================================

/// Demonstrates DELETE request
///
/// DELETE requests remove a resource.
/// They typically return a 200/204 status code on success.
async fn delete_request() -> Result<(), Box<dyn Error>> {
    println!("\n=== DELETE Request ===");

    let client = Client::new();

    let response = client
        .delete("https://jsonplaceholder.typicode.com/posts/1")
        .send()
        .await?;

    println!("Status: {}", response.status());

    // Check if deletion was successful
    if response.status().is_success() {
        println!("Post deleted successfully!");
    }

    // Some APIs return the deleted resource
    let text = response.text().await?;
    println!("Response body: {}", text);

    Ok(())
}

// ============================================================================
// File Upload (Multipart)
// ============================================================================

/// Demonstrates file upload using multipart/form-data
///
/// Multipart forms are used to upload files along with other form data.
/// This is commonly used for file uploads in web applications.
async fn upload_file() -> Result<(), Box<dyn Error>> {
    println!("\n=== File Upload (Multipart) ===");

    let client = Client::new();

    // Create sample file content
    let file_content = "This is the content of my uploaded file.\nLine 2\nLine 3";

    // Build multipart form
    let form = multipart::Form::new()
        .text("title", "My File Upload")
        .text("description", "Uploading a text file")
        .part(
            "file",
            multipart::Part::bytes(file_content.as_bytes().to_vec())
                .file_name("sample.txt")
                .mime_str("text/plain")?,
        );

    // Note: JSONPlaceholder doesn't support file uploads,
    // so this would fail in practice. This demonstrates the pattern.
    println!("Multipart form created with file 'sample.txt'");
    println!("In a real scenario, you would send to an endpoint that accepts uploads:");
    println!("  POST https://example.com/upload");

    // Example of how you would send it (commented out as the endpoint doesn't exist):
    /*
    let response = client
        .post("https://example.com/upload")
        .multipart(form)
        .send()
        .await?;

    println!("Status: {}", response.status());
    */

    Ok(())
}

/// Demonstrates multipart form with multiple files
async fn upload_multiple_files() -> Result<(), Box<dyn Error>> {
    println!("\n=== Upload Multiple Files ===");

    let client = Client::new();

    // Create multiple file parts
    let file1_content = "Content of first file";
    let file2_content = "Content of second file";

    let form = multipart::Form::new()
        .text("user_id", "123")
        .part(
            "files",
            multipart::Part::bytes(file1_content.as_bytes().to_vec())
                .file_name("file1.txt")
                .mime_str("text/plain")?,
        )
        .part(
            "files",
            multipart::Part::bytes(file2_content.as_bytes().to_vec())
                .file_name("file2.txt")
                .mime_str("text/plain")?,
        );

    println!("Created multipart form with 2 files");
    println!("Pattern for uploading to: POST https://example.com/upload-multiple");

    Ok(())
}

// ============================================================================
// Error Handling
// ============================================================================

/// Demonstrates comprehensive error handling
///
/// HTTP requests can fail in many ways: network errors, timeouts,
/// invalid responses, etc. Proper error handling is crucial.
async fn error_handling_examples() -> Result<(), Box<dyn Error>> {
    println!("\n=== Error Handling ===");

    let client = Client::new();

    // Example 1: Handle non-existent endpoint (404)
    println!("\n1. Handling 404 Not Found:");
    match client
        .get("https://jsonplaceholder.typicode.com/posts/99999")
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == StatusCode::NOT_FOUND {
                println!("  Resource not found (404)");
            } else {
                println!("  Status: {}", response.status());
            }
        }
        Err(e) => println!("  Request failed: {}", e),
    }

    // Example 2: Handle invalid URL
    println!("\n2. Handling invalid URL:");
    match client.get("not-a-valid-url").send().await {
        Ok(_) => println!("  Request succeeded (unexpected)"),
        Err(e) => println!("  Request failed: {}", e),
    }

    // Example 3: Check status and handle errors
    println!("\n3. Status-based error handling:");
    let response = client
        .get("https://jsonplaceholder.typicode.com/posts/1")
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => {
            println!("  Success! Processing response...");
            let post: Post = response.json().await?;
            println!("  Got post: {}", post.title);
        }
        StatusCode::NOT_FOUND => {
            println!("  Resource not found");
        }
        StatusCode::INTERNAL_SERVER_ERROR => {
            println!("  Server error occurred");
        }
        status => {
            println!("  Unexpected status: {}", status);
        }
    }

    // Example 4: Using error_for_status()
    println!("\n4. Using error_for_status():");
    let response = client
        .get("https://jsonplaceholder.typicode.com/posts/1")
        .send()
        .await?
        .error_for_status()?; // This will return Err for 4xx and 5xx status codes

    println!("  Status was successful: {}", response.status());

    Ok(())
}

// ============================================================================
// Async Request Patterns
// ============================================================================

/// Demonstrates concurrent requests using tokio::join!
///
/// When you need to make multiple independent requests,
/// you can run them concurrently for better performance.
async fn concurrent_requests() -> Result<(), Box<dyn Error>> {
    println!("\n=== Concurrent Requests ===");

    let client = Client::new();

    // Make three requests concurrently
    let (result1, result2, result3) = tokio::join!(
        client
            .get("https://jsonplaceholder.typicode.com/posts/1")
            .send(),
        client
            .get("https://jsonplaceholder.typicode.com/posts/2")
            .send(),
        client
            .get("https://jsonplaceholder.typicode.com/posts/3")
            .send(),
    );

    println!("Request 1 status: {}", result1?.status());
    println!("Request 2 status: {}", result2?.status());
    println!("Request 3 status: {}", result3?.status());

    Ok(())
}

/// Demonstrates spawning multiple async tasks
async fn parallel_requests() -> Result<(), Box<dyn Error>> {
    println!("\n=== Parallel Requests ===");

    let client = Client::new();

    // Create a vector of tasks
    let mut tasks = vec![];

    for i in 1..=5 {
        let client_clone = client.clone();
        let task = tokio::spawn(async move {
            let url = format!("https://jsonplaceholder.typicode.com/posts/{}", i);
            let response = client_clone.get(&url).send().await?;
            let post: Post = response.json().await?;
            Ok::<_, Box<dyn Error + Send + Sync>>(post)
        });
        tasks.push(task);
    }

    // Wait for all tasks to complete
    println!("Fetching 5 posts in parallel...");
    for (i, task) in tasks.into_iter().enumerate() {
        match task.await? {
            Ok(post) => println!("  Task {}: {}", i + 1, post.title),
            Err(e) => println!("  Task {}: Error - {}", i + 1, e),
        }
    }

    Ok(())
}

/// Demonstrates client configuration and reuse
///
/// Creating a client with specific configuration and reusing it
/// for multiple requests is more efficient than creating new clients.
async fn client_configuration() -> Result<(), Box<dyn Error>> {
    println!("\n=== Client Configuration ===");

    // Build a client with custom configuration
    let client = Client::builder()
        .user_agent("Rust-Reqwest-Example/1.0")
        .timeout(std::time::Duration::from_secs(10))
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                "X-Custom-Header",
                reqwest::header::HeaderValue::from_static("default-value"),
            );
            headers
        })
        .build()?;

    println!("Created client with:");
    println!("  - Custom User-Agent");
    println!("  - 10 second timeout");
    println!("  - Default custom header");

    // Use the configured client
    let response = client
        .get("https://jsonplaceholder.typicode.com/posts/1")
        .send()
        .await?;

    println!("\nStatus: {}", response.status());

    Ok(())
}

/// Demonstrates streaming response body
async fn streaming_response() -> Result<(), Box<dyn Error>> {
    println!("\n=== Streaming Response ===");

    let client = Client::new();

    let response = client
        .get("https://jsonplaceholder.typicode.com/posts")
        .send()
        .await?;

    println!("Status: {}", response.status());
    println!("Content-Length: {:?}", response.content_length());

    // Get the raw bytes
    let bytes = response.bytes().await?;
    println!("Received {} bytes", bytes.len());

    // Convert to string
    let text = String::from_utf8_lossy(&bytes);
    println!("First 100 characters: {}", &text[..100.min(text.len())]);

    Ok(())
}

// ============================================================================
// Main Function - Demonstrates all examples
// ============================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║         Reqwest HTTP Client - Comprehensive Examples      ║");
    println!("╚════════════════════════════════════════════════════════════╝");

    // Basic GET requests
    basic_get_request().await?;
    get_with_response_handling().await?;
    get_multiple_resources().await?;

    // Query parameters and headers
    get_with_query_params().await?;
    get_with_custom_headers().await?;

    // Authentication
    get_with_bearer_auth().await?;
    get_with_basic_auth().await?;

    // POST requests
    post_with_json().await?;
    post_with_form().await?;

    // PUT, PATCH, DELETE
    put_request().await?;
    patch_request().await?;
    delete_request().await?;

    // File uploads
    upload_file().await?;
    upload_multiple_files().await?;

    // Error handling
    error_handling_examples().await?;

    // Async patterns
    concurrent_requests().await?;
    parallel_requests().await?;
    client_configuration().await?;
    streaming_response().await?;

    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║                   All Examples Completed!                  ║");
    println!("╚════════════════════════════════════════════════════════════╝");

    Ok(())
}
