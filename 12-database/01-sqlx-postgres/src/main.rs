use chrono::NaiveDateTime;
use sqlx::postgres::PgPoolOptions;
use sqlx::{FromRow, PgPool};

#[derive(Debug, FromRow)]
#[allow(dead_code)]
struct User {
    id: i32,
    username: String,
    email: String,
    created_at: NaiveDateTime,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Connection string (replace with your actual database)
    // DATABASE_URL=postgresql://user:password@localhost/mydb
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:password@localhost/testdb".to_string());

    println!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Create table
    create_table(&pool).await?;

    // Insert users
    let user_id1 = insert_user(&pool, "alice", "alice@example.com").await?;
    let user_id2 = insert_user(&pool, "bob", "bob@example.com").await?;
    println!("Inserted users with IDs: {}, {}", user_id1, user_id2);

    // Query users
    let users = get_all_users(&pool).await?;
    println!("\nAll users:");
    for user in users {
        println!("  {:?}", user);
    }

    // Find user by username
    if let Some(user) = find_user_by_username(&pool, "alice").await? {
        println!("\nFound user: {:?}", user);
    }

    // Update user
    update_user_email(&pool, user_id1, "alice.new@example.com").await?;
    println!("\nUpdated user email");

    // Delete user
    delete_user(&pool, user_id2).await?;
    println!("Deleted user with ID: {}", user_id2);

    // Count users
    let count = count_users(&pool).await?;
    println!("\nTotal users: {}", count);

    // Transaction example
    transaction_example(&pool).await?;

    println!("\nAll operations completed successfully!");

    Ok(())
}

async fn create_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username VARCHAR(50) UNIQUE NOT NULL,
            email VARCHAR(100) NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    println!("Table created/verified");
    Ok(())
}

async fn insert_user(pool: &PgPool, username: &str, email: &str) -> Result<i32, sqlx::Error> {
    let row: (i32,) = sqlx::query_as(
        "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING id",
    )
    .bind(username)
    .bind(email)
    .fetch_one(pool)
    .await?;

    Ok(row.0)
}

async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY id")
        .fetch_all(pool)
        .await?;

    Ok(users)
}

async fn find_user_by_username(
    pool: &PgPool,
    username: &str,
) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(username)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

async fn update_user_email(pool: &PgPool, id: i32, email: &str) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET email = $1 WHERE id = $2")
        .bind(email)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

async fn delete_user(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

async fn count_users(pool: &PgPool) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;

    Ok(row.0)
}

async fn transaction_example(pool: &PgPool) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query("INSERT INTO users (username, email) VALUES ($1, $2)")
        .bind("charlie")
        .bind("charlie@example.com")
        .execute(&mut *tx)
        .await?;

    sqlx::query("INSERT INTO users (username, email) VALUES ($1, $2)")
        .bind("diana")
        .bind("diana@example.com")
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    println!("Transaction committed successfully");
    Ok(())
}
