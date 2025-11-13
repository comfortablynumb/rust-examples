use futures::future::join_all;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    println!("Connection Pooling Example\n");

    // Create connection pool
    println!("Creating connection pool...");
    let pool = SqlitePoolOptions::new()
        .max_connections(5)                    // Maximum connections
        .min_connections(2)                     // Minimum connections
        .acquire_timeout(Duration::from_secs(3)) // Timeout for acquiring connection
        .idle_timeout(Duration::from_secs(600))  // Close idle connections after 10 min
        .max_lifetime(Duration::from_secs(1800)) // Max connection lifetime (30 min)
        .connect("sqlite::memory:")
        .await?;

    println!("Pool created with max_connections: 5");
    println!("Idle connections: {}\n", pool.size());

    // Setup database
    setup_database(&pool).await?;

    // Sequential queries (slow)
    println!("=== Sequential Execution ===");
    let start = Instant::now();
    for i in 0..10 {
        query_data(&pool, i).await?;
    }
    println!("Sequential time: {:?}\n", start.elapsed());

    // Parallel queries with connection pooling (fast)
    println!("=== Parallel Execution (with pooling) ===");
    let start = Instant::now();

    let tasks: Vec<_> = (0..10)
        .map(|i| {
            let pool = pool.clone();
            tokio::spawn(async move {
                query_data(&pool, i).await
            })
        })
        .collect();

    let results = join_all(tasks).await;

    for result in results {
        result??;
    }

    println!("Parallel time: {:?}\n", start.elapsed());

    // Monitor pool statistics
    println!("=== Pool Statistics ===");
    println!("Pool size: {}", pool.size());
    println!("Idle connections: {}", pool.num_idle());

    // Stress test
    println!("\n=== Stress Test ===");
    let start = Instant::now();

    let tasks: Vec<_> = (0..100)
        .map(|i| {
            let pool = pool.clone();
            tokio::spawn(async move {
                insert_data(&pool, i).await
            })
        })
        .collect();

    join_all(tasks).await;
    println!("Inserted 100 records in {:?}", start.elapsed());

    // Count total records
    let count = count_records(&pool).await?;
    println!("Total records: {}", count);

    // Test connection timeout
    println!("\n=== Testing Connection Acquisition ===");
    println!("Max pool size: 5");
    println!("Attempting to acquire 7 connections simultaneously...");

    let mut handles = vec![];
    for i in 0..7 {
        let pool = pool.clone();
        let handle = tokio::spawn(async move {
            match pool.acquire().await {
                Ok(conn) => {
                    println!("Connection {} acquired", i);
                    tokio::time::sleep(Duration::from_millis(100)).await;
                    drop(conn);
                    println!("Connection {} released", i);
                }
                Err(e) => println!("Connection {} failed: {}", i, e),
            }
        });
        handles.push(handle);
    }

    join_all(handles).await;

    println!("\n=== Pool Health Check ===");
    // Test pool health
    let is_closed = pool.is_closed();
    println!("Pool closed: {}", is_closed);
    println!("Current pool size: {}", pool.size());

    pool.close().await;
    println!("Pool closed gracefully");

    Ok(())
}

async fn setup_database(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS data (
            id INTEGER PRIMARY KEY,
            value INTEGER
        )"
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn query_data(pool: &SqlitePool, id: i32) -> Result<(), sqlx::Error> {
    // Simulate some work
    sqlx::query("SELECT 1")
        .execute(pool)
        .await?;

    tokio::time::sleep(Duration::from_millis(10)).await;

    Ok(())
}

async fn insert_data(pool: &SqlitePool, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO data (id, value) VALUES (?, ?)")
        .bind(id)
        .bind(id * 10)
        .execute(pool)
        .await?;

    Ok(())
}

async fn count_records(pool: &SqlitePool) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM data")
        .fetch_one(pool)
        .await?;

    Ok(row.0)
}
