use anyhow::Result;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;

fn main() -> Result<()> {
    println!("Password Hashing Examples\n");

    let password = b"hunter2"; // User's password

    // Argon2 (Recommended - winner of Password Hashing Competition)
    println!("=== Argon2 (Recommended) ===");
    argon2_example(password)?;

    println!("\n=== bcrypt ===");
    bcrypt_example(password)?;

    println!("\n=== Password Security Best Practices ===");
    println!("✓ Use Argon2id (balanced against side-channel and GPU attacks)");
    println!("✓ Each password gets unique salt (automatic in Argon2/bcrypt)");
    println!("✓ Never store plaintext passwords");
    println!("✓ Use high cost parameters (balanced with UX)");
    println!("✗ Don't use fast hashes (SHA-256, MD5) for passwords");
    println!("✗ Don't implement your own crypto");

    Ok(())
}

fn argon2_example(password: &[u8]) -> Result<()> {
    let argon2 = Argon2::default();

    // Generate salt
    let salt = SaltString::generate(&mut OsRng);

    // Hash password
    let password_hash = argon2
        .hash_password(password, &salt)
        .map_err(|e| anyhow::anyhow!("{}", e))?
        .to_string();

    println!("Password:      {:?}", std::str::from_utf8(password)?);
    println!("Hashed:        {}", password_hash);
    println!("Length:        {} characters", password_hash.len());

    // Verify correct password
    let parsed_hash = PasswordHash::new(&password_hash).map_err(|e| anyhow::anyhow!("{}", e))?;
    match argon2.verify_password(password, &parsed_hash) {
        Ok(()) => println!("✓ Password verified"),
        Err(_) => println!("✗ Password verification failed"),
    }

    // Verify wrong password
    let wrong_password = b"wrong_password";
    match argon2.verify_password(wrong_password, &parsed_hash) {
        Ok(()) => println!("✗ Wrong password accepted (BUG!)"),
        Err(_) => println!("✓ Wrong password rejected"),
    }

    // Show hash components
    println!("\nHash components:");
    println!("  Algorithm: {}", parsed_hash.algorithm);
    println!("  Salt:      {:?}", parsed_hash.salt);

    Ok(())
}

fn bcrypt_example(password: &[u8]) -> Result<()> {
    // Hash with default cost (12)
    let hashed = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;

    println!("Password:      {:?}", std::str::from_utf8(password)?);
    println!("Hashed:        {}", hashed);
    println!("Cost:          {}", bcrypt::DEFAULT_COST);

    // Verify correct password
    if bcrypt::verify(password, &hashed)? {
        println!("✓ Password verified");
    } else {
        println!("✗ Password verification failed");
    }

    // Verify wrong password
    let wrong_password = b"wrong_password";
    if bcrypt::verify(wrong_password, &hashed)? {
        println!("✗ Wrong password accepted (BUG!)");
    } else {
        println!("✓ Wrong password rejected");
    }

    Ok(())
}
