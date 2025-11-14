use anyhow::Result;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Connecting to Redis...");

    // Connect to Redis (default: redis://127.0.0.1/)
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_multiplexed_async_connection().await?;

    println!("Connected to Redis!\n");

    // String operations
    println!("=== String Operations ===");
    con.set("greeting", "Hello, Redis!").await?;
    let greeting: String = con.get("greeting").await?;
    println!("GET greeting: {}", greeting);

    // Set with expiration (3600 seconds)
    con.set_ex("session:123", "user_data", 3600).await?;
    println!("SET session:123 with 3600s TTL");

    // Increment
    con.set("counter", 0).await?;
    let new_val: i32 = con.incr("counter", 1).await?;
    println!("INCR counter: {}", new_val);

    // List operations
    println!("\n=== List Operations ===");
    con.del("mylist").await?;
    con.rpush("mylist", &["item1", "item2", "item3"]).await?;
    let list: Vec<String> = con.lrange("mylist", 0, -1).await?;
    println!("LRANGE mylist: {:?}", list);

    let popped: Option<String> = con.lpop("mylist", None).await?;
    println!("LPOP mylist: {:?}", popped);

    // Set operations
    println!("\n=== Set Operations ===");
    con.del("myset").await?;
    con.sadd("myset", &["apple", "banana", "cherry"]).await?;
    let members: Vec<String> = con.smembers("myset").await?;
    println!("SMEMBERS myset: {:?}", members);

    let is_member: bool = con.sismember("myset", "banana").await?;
    println!("SISMEMBER myset banana: {}", is_member);

    // Hash operations
    println!("\n=== Hash Operations ===");
    con.hset_multiple(
        "user:1",
        &[
            ("name", "Alice"),
            ("email", "alice@example.com"),
            ("age", "30"),
        ],
    )
    .await?;

    let name: String = con.hget("user:1", "name").await?;
    println!("HGET user:1 name: {}", name);

    let user_data: Vec<(String, String)> = con.hgetall("user:1").await?;
    println!("HGETALL user:1: {:?}", user_data);

    // Sorted Set operations
    println!("\n=== Sorted Set Operations ===");
    con.del("leaderboard").await?;
    con.zadd("leaderboard", "Alice", 100).await?;
    con.zadd("leaderboard", "Bob", 85).await?;
    con.zadd("leaderboard", "Charlie", 95).await?;

    let top_players: Vec<(String, f64)> = con.zrevrange_withscores("leaderboard", 0, 2).await?;
    println!("Top 3 players:");
    for (i, (player, score)) in top_players.iter().enumerate() {
        println!("  {}. {} - {}", i + 1, player, score);
    }

    // JSON serialization example
    println!("\n=== JSON Storage ===");
    let user = User {
        id: 1,
        name: "Bob".to_string(),
        email: "bob@example.com".to_string(),
    };

    let json = serde_json::to_string(&user)?;
    con.set::<_, _, ()>("user:json:1", json).await?;

    let retrieved: String = con.get("user:json:1").await?;
    let user_back: User = serde_json::from_str(&retrieved)?;
    println!("Retrieved user: {:?}", user_back);

    // Pipelining
    println!("\n=== Pipeline ===");
    let (v1, v2, v3): (i32, i32, i32) = redis::pipe()
        .set("key1", 10)
        .ignore()
        .set("key2", 20)
        .ignore()
        .set("key3", 30)
        .ignore()
        .get("key1")
        .get("key2")
        .get("key3")
        .query_async(&mut con)
        .await?;
    println!("Pipeline results: {}, {}, {}", v1, v2, v3);

    // Pub/Sub example (simplified)
    println!("\n=== Pub/Sub ===");
    con.publish::<_, _, ()>("notifications", "New message!")
        .await?;
    println!("Published message to 'notifications' channel");

    // Key expiration
    println!("\n=== Expiration ===");
    con.set::<_, _, ()>("temp_key", "temporary").await?;
    con.expire::<_, ()>("temp_key", 60).await?;
    let ttl: i32 = con.ttl("temp_key").await?;
    println!("TTL for temp_key: {} seconds", ttl);

    // Transactions
    println!("\n=== Transactions ===");
    redis::pipe()
        .atomic()
        .set("account:1", 1000)
        .set("account:2", 500)
        .query_async::<_, ()>(&mut con)
        .await?;
    println!("Transaction completed");

    println!("\nAll Redis operations completed!");

    Ok(())
}
