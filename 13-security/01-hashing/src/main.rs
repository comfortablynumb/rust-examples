use sha2::{Sha256, Sha512, Digest};
use blake3::Hasher as Blake3Hasher;

fn main() {
    println!("Cryptographic Hashing Examples\n");

    let data = b"Hello, Rust cryptography!";

    // SHA-256
    println!("=== SHA-256 ===");
    let sha256_hash = hash_sha256(data);
    println!("Data: {:?}", std::str::from_utf8(data).unwrap());
    println!("SHA-256: {}\n", sha256_hash);

    // SHA-512
    println!("=== SHA-512 ===");
    let sha512_hash = hash_sha512(data);
    println!("SHA-512: {}\n", sha512_hash);

    // BLAKE3
    println!("=== BLAKE3 ===");
    let blake3_hash = hash_blake3(data);
    println!("BLAKE3: {}\n", blake3_hash);

    // File integrity check simulation
    println!("=== File Integrity Check ===");
    let file_data = b"Important file content";
    let original_hash = hash_sha256(file_data);
    println!("Original hash: {}", original_hash);

    // Simulate file modification
    let modified_data = b"Important file content!";  // Added '!'
    let new_hash = hash_sha256(modified_data);
    println!("New hash:      {}", new_hash);

    if original_hash == new_hash {
        println!("✓ File integrity verified");
    } else {
        println!("✗ File has been modified!");
    }

    // Password verification (NOT recommended for passwords - use password hashing!)
    println!("\n=== Password Hashing (DEMO ONLY - Use Argon2 for real passwords!) ===");
    let password = "my_secret_password";
    let stored_hash = hash_sha256(password.as_bytes());
    println!("Stored hash: {}", stored_hash);

    let login_attempt = "my_secret_password";
    let attempt_hash = hash_sha256(login_attempt.as_bytes());

    if stored_hash == attempt_hash {
        println!("✓ Password correct");
    } else {
        println!("✗ Password incorrect");
    }

    // Demonstrate hash collision resistance
    println!("\n=== Hash Properties ===");
    let msg1 = "Hello";
    let msg2 = "Hello!";  // Slightly different
    let hash1 = hash_sha256(msg1.as_bytes());
    let hash2 = hash_sha256(msg2.as_bytes());

    println!("Message 1: '{}' -> {}", msg1, hash1);
    println!("Message 2: '{}' -> {}", msg2, hash2);
    println!("Completely different hashes from similar inputs!");

    // Hash performance comparison
    println!("\n=== Performance Comparison ===");
    let large_data = vec![0u8; 1_000_000];  // 1 MB

    let start = std::time::Instant::now();
    hash_sha256(&large_data);
    println!("SHA-256:  {:?}", start.elapsed());

    let start = std::time::Instant::now();
    hash_blake3(&large_data);
    println!("BLAKE3:   {:?}", start.elapsed());

    // HMAC example
    println!("\n=== HMAC (Hash-based Message Authentication Code) ===");
    use sha2::Sha256;
    use hmac::{Hmac, Mac};

    type HmacSha256 = Hmac<Sha256>;

    let key = b"secret_key";
    let message = b"message to authenticate";

    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(message);
    let result = mac.finalize();
    let code_bytes = result.into_bytes();

    println!("Message: {:?}", std::str::from_utf8(message).unwrap());
    println!("HMAC:    {}", hex::encode(code_bytes));
}

fn hash_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

fn hash_sha512(data: &[u8]) -> String {
    let mut hasher = Sha512::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

fn hash_blake3(data: &[u8]) -> String {
    let mut hasher = Blake3Hasher::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result.as_bytes())
}
