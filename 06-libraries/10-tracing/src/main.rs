use tracing::{debug, error, info, instrument, span, trace, warn, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set subscriber");

    info!("Starting tracing example");

    // Basic logging
    basic_logging();

    // Structured logging
    structured_logging();

    // Spans
    span_example();

    // Instrumented functions
    instrumented_example().await;

    info!("Tracing example completed");
}

fn basic_logging() {
    let span = span!(Level::INFO, "basic_logging");
    let _enter = span.enter();

    trace!("This is a trace message (very verbose)");
    debug!("This is a debug message");
    info!("This is an info message");
    warn!("This is a warning message");
    error!("This is an error message");
}

fn structured_logging() {
    let span = span!(Level::INFO, "structured_logging");
    let _enter = span.enter();

    let user_id = 42;
    let username = "alice";

    info!(user_id, username, "User logged in");

    // With additional context
    info!(
        user_id = user_id,
        username = username,
        ip = "192.168.1.1",
        "Login successful"
    );

    // With computed values
    let status_code = 200;
    info!(
        status = status_code,
        success = status_code == 200,
        "HTTP request completed"
    );
}

fn span_example() {
    let span = span!(Level::INFO, "span_example");
    let _enter = span.enter();

    info!("Inside span_example");

    // Nested span
    let inner_span = span!(Level::INFO, "inner_operation", operation = "fetch_data");
    let _inner_enter = inner_span.enter();

    info!("Inside nested span");
    perform_work();
}

fn perform_work() {
    debug!("Performing some work");
    std::thread::sleep(std::time::Duration::from_millis(100));
    debug!("Work completed");
}

#[instrument]
async fn instrumented_example() {
    info!("Inside instrumented function");
    process_request(1, "GET").await;
}

#[instrument(fields(user_id, path))]
async fn process_request(user_id: u64, method: &str) {
    info!(method, "Processing request");

    // Simulate async work
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    if user_id == 0 {
        error!(user_id, "Invalid user ID");
        return;
    }

    query_database(user_id).await;

    info!("Request processed successfully");
}

#[instrument]
async fn query_database(user_id: u64) {
    debug!(user_id, "Querying database");

    // Simulate database query
    tokio::time::sleep(tokio::time::Duration::from_millis(30)).await;

    debug!(user_id, "Database query completed");
}
