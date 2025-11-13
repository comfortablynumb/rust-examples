use rand::{Rng, RngCore, distributions::Alphanumeric, rngs::OsRng};

fn main() {
    println!("Cryptographically Secure Random Numbers\n");

    // Generate random bytes
    println!("=== Random Bytes ===");
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    println!("256-bit key: {}", hex::encode(key));

    // Generate random numbers
    println!("\n=== Random Numbers ===");
    let random_u64: u64 = OsRng.gen();
    println!("Random u64: {}", random_u64);

    let random_range: u32 = OsRng.gen_range(1..=100);
    println!("Random 1-100: {}", random_range);

    // Generate random string
    println!("\n=== Random Strings ===");
    let token: String = (0..32)
        .map(|_| OsRng.sample(Alphanumeric) as char)
        .collect();
    println!("Token: {}", token);

    // Session ID
    let session_id: String = (0..16)
        .map(|_| format!("{:02x}", OsRng.gen::<u8>()))
        .collect();
    println!("Session ID: {}", session_id);

    println!("\n=== Use Cases ===");
    println!("✓ Cryptographic keys");
    println!("✓ Session tokens");
    println!("✓ Nonces");
    println!("✓ Salts");
    println!("✓ Initialization vectors");
}
