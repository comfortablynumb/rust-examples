use serde::{Deserialize, Serialize};
use sled::{Db, IVec};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Sled Embedded Database Example\n");

    // Open database (creates if doesn't exist)
    let db: Db = sled::open("my_database")?;
    println!("Database opened");

    // Basic key-value operations
    println!("\n=== Basic Operations ===");
    db.insert("greeting", "Hello, Sled!")?;

    if let Some(value) = db.get("greeting")? {
        println!("GET greeting: {}", String::from_utf8(value.to_vec())?);
    }

    // Store numbers
    db.insert("counter", &42_u64.to_be_bytes())?;
    if let Some(value) = db.get("counter")? {
        let num = u64::from_be_bytes(value.as_ref().try_into()?);
        println!("GET counter: {}", num);
    }

    // Increment
    let new_value = db.update_and_fetch("counter", |old| {
        let number = match old {
            Some(bytes) => {
                let arr: [u8; 8] = bytes.try_into().unwrap();
                u64::from_be_bytes(arr)
            }
            None => 0,
        };
        Some((number + 1).to_be_bytes().to_vec())
    })?;

    if let Some(value) = new_value {
        let num = u64::from_be_bytes(value.as_ref().try_into()?);
        println!("INCR counter: {}", num);
    }

    // Delete
    db.remove("greeting")?;
    println!("DELETE greeting");

    // Structured data with bincode
    println!("\n=== Structured Data ===");

    let user1 = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    let user2 = User {
        id: 2,
        name: "Bob".to_string(),
        email: "bob@example.com".to_string(),
    };

    // Serialize and store
    db.insert("user:1", bincode::serialize(&user1)?)?;
    db.insert("user:2", bincode::serialize(&user2)?)?;
    println!("Stored 2 users");

    // Retrieve and deserialize
    if let Some(data) = db.get("user:1")? {
        let user: User = bincode::deserialize(&data)?;
        println!("Retrieved user: {:?}", user);
    }

    // Iteration
    println!("\n=== Iteration ===");
    println!("All keys starting with 'user:':");
    for result in db.scan_prefix("user:") {
        let (key, value) = result?;
        let key_str = String::from_utf8(key.to_vec())?;
        let user: User = bincode::deserialize(&value)?;
        println!("  {} -> {:?}", key_str, user);
    }

    // Range queries
    println!("\n=== Range Queries ===");
    for result in db.range("user:".."user:~") {
        let (key, _) = result?;
        println!("  Key in range: {}", String::from_utf8(key.to_vec())?);
    }

    // Transactions
    println!("\n=== Transactions ===");
    let tree = db.clone();
    tree.transaction(|tx_db| {
        tx_db.insert(b"account:1", &1000_u64.to_be_bytes())?;
        tx_db.insert(b"account:2", &500_u64.to_be_bytes())?;
        Ok(())
    })?;
    println!("Transaction committed");

    // Compare and swap (CAS)
    println!("\n=== Compare and Swap ===");
    let old_value = db.get("counter")?;
    let new_val = 100_u64;
    match db.compare_and_swap("counter", old_value.clone(), Some(new_val.to_be_bytes().to_vec()))? {
        Ok(_) => println!("CAS successful: counter = {}", new_val),
        Err(_) => println!("CAS failed: value changed"),
    }

    // Batch operations
    println!("\n=== Batch Operations ===");
    let mut batch = sled::Batch::default();
    batch.insert(b"batch:1", b"value1");
    batch.insert(b"batch:2", b"value2");
    batch.insert(b"batch:3", b"value3");
    db.apply_batch(batch)?;
    println!("Applied batch with 3 insertions");

    // Statistics
    println!("\n=== Statistics ===");
    println!("Total keys: {}", db.len());
    println!("Size on disk: {} bytes", db.size_on_disk()?);

    // Flush to disk
    db.flush()?;
    println!("\nFlushed to disk");

    // Cleanup (optional - for demo purposes)
    // drop(db);
    // std::fs::remove_dir_all("my_database")?;

    println!("\nAll operations completed!");

    Ok(())
}
