use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject};
use axum::{
    extract::State,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;

#[derive(SimpleObject, Clone)]
struct Book {
    id: i32,
    title: String,
    author: String,
    year: i32,
}

#[derive(SimpleObject, Clone)]
struct Author {
    id: i32,
    name: String,
    country: String,
}

struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get a book by ID
    async fn book(&self, ctx: &Context<'_>, id: i32) -> Option<Book> {
        let books = ctx.data::<Vec<Book>>().unwrap();
        books.iter().find(|b| b.id == id).cloned()
    }

    /// Get all books
    async fn books(&self, ctx: &Context<'_>) -> Vec<Book> {
        ctx.data::<Vec<Book>>().unwrap().clone()
    }

    /// Get books by author name
    async fn books_by_author(&self, ctx: &Context<'_>, author: String) -> Vec<Book> {
        let books = ctx.data::<Vec<Book>>().unwrap();
        books
            .iter()
            .filter(|b| b.author == author)
            .cloned()
            .collect()
    }

    /// Get an author by ID
    async fn author(&self, ctx: &Context<'_>, id: i32) -> Option<Author> {
        let authors = ctx.data::<Vec<Author>>().unwrap();
        authors.iter().find(|a| a.id == id).cloned()
    }

    /// Get all authors
    async fn authors(&self, ctx: &Context<'_>) -> Vec<Author> {
        ctx.data::<Vec<Author>>().unwrap().clone()
    }

    /// Search books by title
    async fn search_books(&self, ctx: &Context<'_>, query: String) -> Vec<Book> {
        let books = ctx.data::<Vec<Book>>().unwrap();
        books
            .iter()
            .filter(|b| b.title.to_lowercase().contains(&query.to_lowercase()))
            .cloned()
            .collect()
    }
}

type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[derive(serde::Deserialize)]
struct GraphQLRequest {
    query: String,
    #[serde(default)]
    variables: serde_json::Value,
}

async fn graphql_handler(
    State(schema): State<Arc<AppSchema>>,
    Json(request): Json<GraphQLRequest>,
) -> Response {
    let response = schema
        .execute(
            async_graphql::Request::new(request.query)
                .variables(async_graphql::Variables::from_json(request.variables)),
        )
        .await;

    Json(json!({
        "data": response.data,
        "errors": response.errors,
    }))
    .into_response()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>GraphQL Playground</title>
            <style>body { margin: 0; font-family: Arial; padding: 20px; }</style>
        </head>
        <body>
            <div id="root">
                <h1>GraphQL API</h1>
                <p>Post queries to <code>/graphql</code></p>
                <h3>Example Query:</h3>
                <pre>
query {
  books {
    id
    title
    author
    year
  }
}
                </pre>
                <h3>Example Query with Parameter:</h3>
                <pre>
query {
  book(id: 1) {
    id
    title
    author
  }
}
                </pre>
                <h3>Search Example:</h3>
                <pre>
query {
  searchBooks(query: "the") {
    title
    author
  }
}
                </pre>
            </div>
        </body>
        </html>
        "#,
    )
}

#[tokio::main]
async fn main() {
    println!("GraphQL Server Example\n");

    // Sample data
    let books = vec![
        Book {
            id: 1,
            title: "The Rust Programming Language".to_string(),
            author: "Steve Klabnik".to_string(),
            year: 2018,
        },
        Book {
            id: 2,
            title: "Programming Rust".to_string(),
            author: "Jim Blandy".to_string(),
            year: 2021,
        },
        Book {
            id: 3,
            title: "Rust in Action".to_string(),
            author: "Tim McNamara".to_string(),
            year: 2021,
        },
    ];

    let authors = vec![
        Author {
            id: 1,
            name: "Steve Klabnik".to_string(),
            country: "USA".to_string(),
        },
        Author {
            id: 2,
            name: "Jim Blandy".to_string(),
            country: "USA".to_string(),
        },
        Author {
            id: 3,
            name: "Tim McNamara".to_string(),
            country: "New Zealand".to_string(),
        },
    ];

    // Build GraphQL schema
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(books)
        .data(authors)
        .finish();

    let schema = Arc::new(schema);

    // Build Axum app
    let app = Router::new()
        .route("/", get(graphql_playground))
        .route("/graphql", post(graphql_handler))
        .with_state(schema);

    let addr = "127.0.0.1:8000";
    println!("GraphQL server listening on http://{}", addr);
    println!("Open http://{} in your browser\n", addr);

    // Print some example queries
    println!("Example queries you can try:");
    println!("\n1. Get all books:");
    println!("   curl -X POST http://{}/graphql \\", addr);
    println!("     -H 'Content-Type: application/json' \\");
    println!("     -d '{{\"query\": \"{{ books {{ id title author }} }}\"}}'\n");

    println!("2. Get book by ID:");
    println!("   curl -X POST http://{}/graphql \\", addr);
    println!("     -H 'Content-Type: application/json' \\");
    println!("     -d '{{\"query\": \"{{ book(id: 1) {{ title author }} }}\"}}'\n");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
