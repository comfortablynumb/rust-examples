use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, FromRow)]
struct Todo {
    id: i64,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    println!("Creating SQLite database...");

    // Create in-memory database (use "sqlite:///path/to/db.sqlite" for file)
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite::memory:")
        .await?;

    setup_database(&pool).await?;

    // Create todos
    let id1 = create_todo(&pool, "Learn Rust").await?;
    let id2 = create_todo(&pool, "Build a project").await?;
    let id3 = create_todo(&pool, "Master async").await?;

    println!("Created todos with IDs: {}, {}, {}", id1, id2, id3);

    // List all todos
    let todos = get_all_todos(&pool).await?;
    println!("\nAll todos:");
    for todo in &todos {
        println!("  [{}] {} - {}",
            todo.id,
            todo.title,
            if todo.completed { "✓" } else { " " }
        );
    }

    // Complete a todo
    complete_todo(&pool, id1).await?;
    println!("\nCompleted todo {}", id1);

    // Get active todos
    let active = get_active_todos(&pool).await?;
    println!("\nActive todos: {}", active.len());
    for todo in active {
        println!("  {}", todo.title);
    }

    // Update todo title
    update_todo_title(&pool, id2, "Build an awesome project").await?;

    // Delete todo
    delete_todo(&pool, id3).await?;

    // Final count
    let count = count_todos(&pool).await?;
    println!("\nTotal todos remaining: {}", count);

    Ok(())
}

async fn setup_database(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT 0
        )
        "#,
    )
    .execute(pool)
    .await?;

    println!("Database setup complete");
    Ok(())
}

async fn create_todo(pool: &SqlitePool, title: &str) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO todos (title) VALUES (?)")
        .bind(title)
        .execute(pool)
        .await?;

    Ok(result.last_insert_rowid())
}

async fn get_all_todos(pool: &SqlitePool) -> Result<Vec<Todo>, sqlx::Error> {
    sqlx::query_as::<_, Todo>("SELECT * FROM todos ORDER BY id")
        .fetch_all(pool)
        .await
}

async fn get_active_todos(pool: &SqlitePool) -> Result<Vec<Todo>, sqlx::Error> {
    sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE completed = 0")
        .fetch_all(pool)
        .await
}

async fn complete_todo(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE todos SET completed = 1 WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

async fn update_todo_title(pool: &SqlitePool, id: i64, title: &str) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE todos SET title = ? WHERE id = ?")
        .bind(title)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

async fn delete_todo(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

async fn count_todos(pool: &SqlitePool) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM todos")
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}
