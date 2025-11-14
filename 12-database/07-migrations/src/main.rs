use sqlx::sqlite::SqlitePool;
use sqlx::{migrate::MigrateDatabase, Sqlite};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Database Migrations Example\n");

    let db_url = "sqlite://./test.db";

    // Create database if it doesn't exist
    if !Sqlite::database_exists(db_url).await? {
        println!("Creating database...");
        Sqlite::create_database(db_url).await?;
        println!("Database created!");
    } else {
        println!("Database already exists");
    }

    // Connect to database
    let pool = SqlitePool::connect(db_url).await?;

    // Run migrations
    println!("\nRunning migrations...");
    run_migrations(&pool).await?;

    // Verify schema
    println!("\nVerifying schema...");
    verify_schema(&pool).await?;

    // Test the schema
    println!("\nTesting schema...");
    test_schema(&pool).await?;

    println!("\nAll operations completed successfully!");

    // Cleanup
    pool.close().await;

    Ok(())
}

async fn run_migrations(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    // Run migrations from the migrations directory
    let migrations = std::path::Path::new("./migrations");

    if migrations.exists() {
        // SQLx migrations
        sqlx::migrate!("./migrations").run(pool).await?;

        println!("Migrations completed!");
    } else {
        println!("No migrations directory found, running inline migrations...");

        // Alternative: inline migrations for demonstration
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL UNIQUE,
                email TEXT NOT NULL,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
        )
        .execute(pool)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS posts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER NOT NULL,
                title TEXT NOT NULL,
                body TEXT NOT NULL,
                published BOOLEAN NOT NULL DEFAULT 0,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
            )",
        )
        .execute(pool)
        .await?;

        println!("Inline migrations completed!");
    }

    Ok(())
}

async fn verify_schema(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    // Get list of tables
    let tables: Vec<(String,)> =
        sqlx::query_as("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
            .fetch_all(pool)
            .await?;

    println!("Tables in database:");
    for (table,) in tables {
        println!("  - {}", table);

        // Get columns for each table
        let columns: Vec<(i32, String, String, i32, Option<String>, i32)> =
            sqlx::query_as(&format!("PRAGMA table_info({})", table))
                .fetch_all(pool)
                .await?;

        for (_cid, name, type_, notnull, dflt_value, _pk) in columns {
            let nullable = if notnull == 1 { "NOT NULL" } else { "NULL" };
            let default = dflt_value
                .map(|v| format!(" DEFAULT {}", v))
                .unwrap_or_default();
            println!("      {} {} {}{}", name, type_, nullable, default);
        }
    }

    Ok(())
}

async fn test_schema(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    // Insert a user
    sqlx::query("INSERT INTO users (username, email) VALUES (?, ?)")
        .bind("alice")
        .bind("alice@example.com")
        .execute(pool)
        .await?;

    let user_id = sqlx::query_scalar::<_, i64>("SELECT id FROM users WHERE username = ?")
        .bind("alice")
        .fetch_one(pool)
        .await?;

    println!("Created user with ID: {}", user_id);

    // Insert a post
    sqlx::query("INSERT INTO posts (user_id, title, body) VALUES (?, ?, ?)")
        .bind(user_id)
        .bind("My First Post")
        .bind("This is the content of my first post!")
        .execute(pool)
        .await?;

    println!("Created post for user");

    // Query posts
    let posts: Vec<(i64, String)> = sqlx::query_as("SELECT id, title FROM posts WHERE user_id = ?")
        .bind(user_id)
        .fetch_all(pool)
        .await?;

    println!("Posts:");
    for (id, title) in posts {
        println!("  [{}] {}", id, title);
    }

    Ok(())
}
