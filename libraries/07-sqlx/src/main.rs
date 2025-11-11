#![allow(dead_code)]
#![allow(unused_variables)]

//! # SQLx Database Example
//!
//! This example demonstrates comprehensive database operations using SQLx with SQLite.
//! SQLx is an async, pure Rust SQL crate featuring compile-time checked queries.
//!
//! Key features demonstrated:
//! - Database connection and pooling
//! - Schema migrations
//! - CRUD operations (Create, Read, Update, Delete)
//! - Transactions
//! - Prepared statements
//! - Query macros for compile-time verification
//! - Error handling

use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, FromRow, Row};

// ============================================================================
// Data Models
// ============================================================================

/// Product entity representing items in our database
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
struct Product {
    id: i64,
    name: String,
    description: Option<String>,
    price: f64,
    quantity: i32,
}

/// User entity for demonstrating additional CRUD operations
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
struct User {
    id: i64,
    username: String,
    email: String,
    active: bool,
}

/// Struct for creating new products (without ID)
#[derive(Debug, Clone)]
struct NewProduct {
    name: String,
    description: Option<String>,
    price: f64,
    quantity: i32,
}

// ============================================================================
// Database Setup
// ============================================================================

/// Initialize the database connection pool
///
/// SQLx uses connection pooling for efficient database access.
/// The pool manages multiple connections and reuses them.
async fn init_pool() -> Result<SqlitePool, sqlx::Error> {
    // Using in-memory database for portability
    // For persistent storage, use: "sqlite:database.db"
    let pool = SqlitePool::connect("sqlite::memory:").await?;

    println!("✓ Database connection pool established");
    Ok(pool)
}

/// Create database schema (manual migration)
///
/// In production, you would use SQLx migrations via CLI:
/// `sqlx migrate add create_tables`
async fn create_schema(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Create products table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT,
            price REAL NOT NULL,
            quantity INTEGER NOT NULL DEFAULT 0
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create users table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            active BOOLEAN NOT NULL DEFAULT 1
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create index for better query performance
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_products_name ON products(name)")
        .execute(pool)
        .await?;

    println!("✓ Database schema created");
    Ok(())
}

// ============================================================================
// INSERT Operations
// ============================================================================

/// Insert a single product using query builder
async fn insert_product(
    pool: &SqlitePool,
    product: NewProduct,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        INSERT INTO products (name, description, price, quantity)
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(&product.name)
    .bind(&product.description)
    .bind(product.price)
    .bind(product.quantity)
    .execute(pool)
    .await?;

    // Get the ID of the inserted row
    let id = result.last_insert_rowid();
    println!("✓ Inserted product '{}' with ID: {}", product.name, id);

    Ok(id)
}

/// Insert a user using prepared statement pattern
async fn insert_user(
    pool: &SqlitePool,
    username: &str,
    email: &str,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO users (username, email, active) VALUES (?, ?, ?)"
    )
    .bind(username)
    .bind(email)
    .bind(true)
    .execute(pool)
    .await?;

    let id = result.last_insert_rowid();
    println!("✓ Inserted user '{}' with ID: {}", username, id);

    Ok(id)
}

/// Batch insert multiple products efficiently
async fn batch_insert_products(
    pool: &SqlitePool,
    products: Vec<NewProduct>,
) -> Result<(), sqlx::Error> {
    // Begin a transaction for atomic batch insert
    let mut tx = pool.begin().await?;

    for product in products {
        sqlx::query(
            "INSERT INTO products (name, description, price, quantity) VALUES (?, ?, ?, ?)"
        )
        .bind(&product.name)
        .bind(&product.description)
        .bind(product.price)
        .bind(product.quantity)
        .execute(&mut *tx)
        .await?;
    }

    // Commit the transaction
    tx.commit().await?;
    println!("✓ Batch insert completed");

    Ok(())
}

// ============================================================================
// SELECT Operations
// ============================================================================

/// Fetch a single product by ID
async fn get_product_by_id(
    pool: &SqlitePool,
    id: i64,
) -> Result<Option<Product>, sqlx::Error> {
    let product = sqlx::query_as::<_, Product>(
        "SELECT id, name, description, price, quantity FROM products WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    match &product {
        Some(p) => println!("✓ Found product: {:?}", p),
        None => println!("✗ Product with ID {} not found", id),
    }

    Ok(product)
}

/// Fetch all products from the database
async fn get_all_products(pool: &SqlitePool) -> Result<Vec<Product>, sqlx::Error> {
    let products = sqlx::query_as::<_, Product>(
        "SELECT id, name, description, price, quantity FROM products ORDER BY id"
    )
    .fetch_all(pool)
    .await?;

    println!("✓ Retrieved {} products", products.len());
    Ok(products)
}

/// Fetch products with a WHERE clause (filter by price)
async fn get_products_by_price_range(
    pool: &SqlitePool,
    min_price: f64,
    max_price: f64,
) -> Result<Vec<Product>, sqlx::Error> {
    let products = sqlx::query_as::<_, Product>(
        r#"
        SELECT id, name, description, price, quantity
        FROM products
        WHERE price BETWEEN ? AND ?
        ORDER BY price
        "#
    )
    .bind(min_price)
    .bind(max_price)
    .fetch_all(pool)
    .await?;

    println!(
        "✓ Found {} products in price range ${:.2} - ${:.2}",
        products.len(),
        min_price,
        max_price
    );

    Ok(products)
}

/// Fetch a single user by username
async fn get_user_by_username(
    pool: &SqlitePool,
    username: &str,
) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, email, active FROM users WHERE username = ?"
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

/// Demonstrate using query! macro for compile-time checked queries
///
/// Note: query! macro requires DATABASE_URL environment variable and
/// database to exist at compile time. For this example, we use query_as
/// which provides runtime checking.
async fn get_products_with_low_stock(
    pool: &SqlitePool,
    threshold: i32,
) -> Result<Vec<Product>, sqlx::Error> {
    // Using query_as for runtime-checked queries
    let products = sqlx::query_as::<_, Product>(
        "SELECT id, name, description, price, quantity FROM products WHERE quantity < ?"
    )
    .bind(threshold)
    .fetch_all(pool)
    .await?;

    println!("✓ Found {} products with low stock", products.len());
    Ok(products)
}

/// Fetch single row using Row trait for custom mapping
async fn get_product_count(pool: &SqlitePool) -> Result<i64, sqlx::Error> {
    let row = sqlx::query("SELECT COUNT(*) as count FROM products")
        .fetch_one(pool)
        .await?;

    let count: i64 = row.get("count");
    println!("✓ Total products in database: {}", count);

    Ok(count)
}

// ============================================================================
// UPDATE Operations
// ============================================================================

/// Update product price
async fn update_product_price(
    pool: &SqlitePool,
    id: i64,
    new_price: f64,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("UPDATE products SET price = ? WHERE id = ?")
        .bind(new_price)
        .bind(id)
        .execute(pool)
        .await?;

    let updated = result.rows_affected() > 0;
    if updated {
        println!("✓ Updated product {} price to ${:.2}", id, new_price);
    } else {
        println!("✗ Product {} not found", id);
    }

    Ok(updated)
}

/// Update product quantity (inventory management)
async fn update_product_quantity(
    pool: &SqlitePool,
    id: i64,
    quantity_change: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE products SET quantity = quantity + ? WHERE id = ?")
        .bind(quantity_change)
        .bind(id)
        .execute(pool)
        .await?;

    println!("✓ Updated product {} quantity by {}", id, quantity_change);
    Ok(())
}

/// Update full product record
async fn update_product(
    pool: &SqlitePool,
    id: i64,
    name: &str,
    description: Option<&str>,
    price: f64,
    quantity: i32,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE products
        SET name = ?, description = ?, price = ?, quantity = ?
        WHERE id = ?
        "#
    )
    .bind(name)
    .bind(description)
    .bind(price)
    .bind(quantity)
    .bind(id)
    .execute(pool)
    .await?;

    let updated = result.rows_affected() > 0;
    if updated {
        println!("✓ Updated product {}", id);
    }

    Ok(updated)
}

// ============================================================================
// DELETE Operations
// ============================================================================

/// Delete a product by ID
async fn delete_product(pool: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM products WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    let deleted = result.rows_affected() > 0;
    if deleted {
        println!("✓ Deleted product {}", id);
    } else {
        println!("✗ Product {} not found", id);
    }

    Ok(deleted)
}

/// Delete products with zero quantity
async fn delete_out_of_stock_products(pool: &SqlitePool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM products WHERE quantity = 0")
        .execute(pool)
        .await?;

    let count = result.rows_affected();
    println!("✓ Deleted {} out-of-stock products", count);

    Ok(count)
}

// ============================================================================
// Transactions
// ============================================================================

/// Demonstrate transaction for atomic operations
///
/// Transactions ensure that multiple operations either all succeed or all fail.
/// This is critical for maintaining data consistency.
async fn transfer_stock_between_products(
    pool: &SqlitePool,
    from_id: i64,
    to_id: i64,
    quantity: i32,
) -> Result<(), sqlx::Error> {
    // Begin transaction
    let mut tx = pool.begin().await?;

    // Check if source has enough stock
    let row = sqlx::query("SELECT quantity FROM products WHERE id = ?")
        .bind(from_id)
        .fetch_one(&mut *tx)
        .await?;

    let current_quantity: i32 = row.get("quantity");

    if current_quantity < quantity {
        // Rollback happens automatically when tx is dropped without commit
        return Err(sqlx::Error::RowNotFound);
    }

    // Decrease quantity from source product
    sqlx::query("UPDATE products SET quantity = quantity - ? WHERE id = ?")
        .bind(quantity)
        .bind(from_id)
        .execute(&mut *tx)
        .await?;

    // Increase quantity for destination product
    sqlx::query("UPDATE products SET quantity = quantity + ? WHERE id = ?")
        .bind(quantity)
        .bind(to_id)
        .execute(&mut *tx)
        .await?;

    // Commit transaction - both operations succeed together
    tx.commit().await?;

    println!(
        "✓ Transferred {} units from product {} to product {}",
        quantity, from_id, to_id
    );

    Ok(())
}

/// Complex transaction example: Create order (deduct stock)
async fn create_order_transaction(
    pool: &SqlitePool,
    product_id: i64,
    quantity: i32,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    // Lock the row for update to prevent race conditions
    let row = sqlx::query("SELECT quantity FROM products WHERE id = ? FOR UPDATE")
        .bind(product_id)
        .fetch_one(&mut *tx)
        .await?;

    let available: i32 = row.get("quantity");

    if available < quantity {
        println!("✗ Insufficient stock for order");
        // Transaction will rollback on drop
        return Err(sqlx::Error::RowNotFound);
    }

    // Update inventory
    sqlx::query("UPDATE products SET quantity = quantity - ? WHERE id = ?")
        .bind(quantity)
        .bind(product_id)
        .execute(&mut *tx)
        .await?;

    // In a real application, you would also insert into an orders table here

    tx.commit().await?;
    println!("✓ Order created, inventory updated");

    Ok(())
}

// ============================================================================
// Prepared Statements
// ============================================================================

/// Demonstrate prepared statements for repeated queries
///
/// SQLx automatically uses prepared statements for better performance.
/// The same query with different parameters reuses the prepared statement.
async fn demonstrate_prepared_statements(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    println!("\n--- Demonstrating Prepared Statements ---");

    // This query will be prepared once and reused
    let query = "SELECT id, name, price FROM products WHERE price > ?";

    // First execution - statement is prepared
    let products1 = sqlx::query_as::<_, Product>(query)
        .bind(10.0)
        .fetch_all(pool)
        .await?;
    println!("Found {} products over $10.00", products1.len());

    // Second execution - reuses prepared statement
    let products2 = sqlx::query_as::<_, Product>(query)
        .bind(50.0)
        .fetch_all(pool)
        .await?;
    println!("Found {} products over $50.00", products2.len());

    // Third execution - reuses prepared statement again
    let products3 = sqlx::query_as::<_, Product>(query)
        .bind(100.0)
        .fetch_all(pool)
        .await?;
    println!("Found {} products over $100.00", products3.len());

    Ok(())
}

// ============================================================================
// Error Handling Examples
// ============================================================================

/// Demonstrate various error handling patterns
async fn demonstrate_error_handling(pool: &SqlitePool) {
    println!("\n--- Demonstrating Error Handling ---");

    // Handle missing record
    match get_product_by_id(pool, 9999).await {
        Ok(Some(product)) => println!("Found: {:?}", product),
        Ok(None) => println!("Product not found (expected)"),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Handle constraint violation (duplicate username)
    match insert_user(pool, "alice", "alice@example.com").await {
        Ok(id) => println!("Created user with ID: {}", id),
        Err(e) => {
            if let sqlx::Error::Database(db_err) = &e {
                println!("Database constraint violation: {}", db_err);
            } else {
                eprintln!("Other error: {}", e);
            }
        }
    }

    // Handle transaction rollback
    let result = transfer_stock_between_products(pool, 1, 2, 1000).await;
    match result {
        Ok(_) => println!("Transfer succeeded"),
        Err(e) => println!("Transfer failed (expected): {:?}", e),
    }
}

// ============================================================================
// Main Function - Demonstrates all features
// ============================================================================

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    println!("=== SQLx Comprehensive Example ===\n");

    // 1. Initialize database connection pool
    let pool = init_pool().await?;

    // 2. Create schema (migrations)
    create_schema(&pool).await?;

    println!("\n--- INSERT Operations ---");

    // 3. Insert single products
    let laptop = NewProduct {
        name: "Laptop Pro".to_string(),
        description: Some("High-performance laptop with 16GB RAM".to_string()),
        price: 1299.99,
        quantity: 15,
    };
    let laptop_id = insert_product(&pool, laptop).await?;

    let mouse = NewProduct {
        name: "Wireless Mouse".to_string(),
        description: Some("Ergonomic wireless mouse".to_string()),
        price: 29.99,
        quantity: 50,
    };
    insert_product(&pool, mouse).await?;

    let keyboard = NewProduct {
        name: "Mechanical Keyboard".to_string(),
        description: None,
        price: 79.99,
        quantity: 30,
    };
    insert_product(&pool, keyboard).await?;

    // 4. Batch insert products
    let batch_products = vec![
        NewProduct {
            name: "Monitor 27\"".to_string(),
            description: Some("4K UHD Monitor".to_string()),
            price: 399.99,
            quantity: 10,
        },
        NewProduct {
            name: "USB-C Cable".to_string(),
            description: Some("2m USB-C charging cable".to_string()),
            price: 12.99,
            quantity: 100,
        },
        NewProduct {
            name: "Webcam HD".to_string(),
            description: Some("1080p webcam".to_string()),
            price: 69.99,
            quantity: 25,
        },
        NewProduct {
            name: "Desk Lamp".to_string(),
            description: None,
            price: 34.99,
            quantity: 0, // Out of stock
        },
    ];
    batch_insert_products(&pool, batch_products).await?;

    // 5. Insert users
    insert_user(&pool, "alice", "alice@example.com").await?;
    insert_user(&pool, "bob", "bob@example.com").await?;
    insert_user(&pool, "charlie", "charlie@example.com").await?;

    println!("\n--- SELECT Operations ---");

    // 6. Fetch single product
    get_product_by_id(&pool, laptop_id).await?;
    get_product_by_id(&pool, 999).await?; // Non-existent

    // 7. Fetch all products
    let all_products = get_all_products(&pool).await?;
    for product in &all_products {
        println!(
            "  - {} (${:.2}) - Stock: {}",
            product.name, product.price, product.quantity
        );
    }

    // 8. Fetch products by price range
    let mid_range = get_products_by_price_range(&pool, 20.0, 100.0).await?;
    println!("Mid-range products:");
    for product in &mid_range {
        println!("  - {} at ${:.2}", product.name, product.price);
    }

    // 9. Fetch products with low stock
    let low_stock = get_products_with_low_stock(&pool, 20).await?;
    println!("Low stock products:");
    for product in &low_stock {
        println!("  - {} (only {} left)", product.name, product.quantity);
    }

    // 10. Get product count
    get_product_count(&pool).await?;

    // 11. Fetch user
    if let Some(user) = get_user_by_username(&pool, "alice").await? {
        println!("✓ User found: {} ({})", user.username, user.email);
    }

    println!("\n--- UPDATE Operations ---");

    // 12. Update product price
    update_product_price(&pool, laptop_id, 1199.99).await?;

    // 13. Update product quantity
    update_product_quantity(&pool, laptop_id, -5).await?; // Sold 5 units

    // 14. Update full product
    update_product(
        &pool,
        laptop_id,
        "Laptop Pro X",
        Some("Updated: High-performance laptop with 32GB RAM"),
        1399.99,
        10,
    )
    .await?;

    println!("\n--- Transaction Operations ---");

    // 15. Transfer stock between products
    transfer_stock_between_products(&pool, 2, 3, 10).await?;

    // 16. Create order with transaction
    create_order_transaction(&pool, 2, 5).await?;

    println!("\n--- DELETE Operations ---");

    // 17. Delete out of stock products
    delete_out_of_stock_products(&pool).await?;

    // 18. Delete specific product (optional)
    // delete_product(&pool, 5).await?;

    // 19. Prepared statements demo
    demonstrate_prepared_statements(&pool).await?;

    // 20. Error handling demonstration
    demonstrate_error_handling(&pool).await;

    println!("\n--- Final State ---");
    let final_products = get_all_products(&pool).await?;
    println!("Final product count: {}", final_products.len());

    println!("\n=== Example Complete ===");
    println!("SQLx provides:");
    println!("  ✓ Async database operations");
    println!("  ✓ Compile-time query verification (with query! macro)");
    println!("  ✓ Connection pooling");
    println!("  ✓ Transaction support");
    println!("  ✓ Type-safe query results");
    println!("  ✓ Multiple database support (PostgreSQL, MySQL, SQLite)");

    Ok(())
}
