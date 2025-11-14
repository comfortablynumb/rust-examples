use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use chacha20poly1305::ChaCha20Poly1305;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Symmetric Encryption Examples\n");

    let plaintext = b"Secret message that needs encryption!";

    // AES-256-GCM
    println!("=== AES-256-GCM ===");
    aes_gcm_example(plaintext)?;

    // ChaCha20-Poly1305
    println!("\n=== ChaCha20-Poly1305 ===");
    chacha_example(plaintext)?;

    Ok(())
}

fn aes_gcm_example(plaintext: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    // Generate random key
    let key = Aes256Gcm::generate_key(OsRng);
    let cipher = Aes256Gcm::new(&key);

    // Generate random nonce (number used once)
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    // Encrypt
    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|e| format!("Encryption failed: {}", e))?;

    println!("Plaintext:  {:?}", std::str::from_utf8(plaintext)?);
    println!("Key:        {}", hex::encode(&key));
    println!("Nonce:      {}", hex::encode(&nonce));
    println!("Ciphertext: {}", hex::encode(&ciphertext));

    // Decrypt
    let decrypted = cipher
        .decrypt(&nonce, ciphertext.as_ref())
        .map_err(|e| format!("Decryption failed: {}", e))?;

    println!("Decrypted:  {:?}", std::str::from_utf8(&decrypted)?);
    println!("✓ Decryption successful");

    // Try to decrypt with wrong nonce (will fail)
    let wrong_nonce = Nonce::from_slice(&[0u8; 12]);
    match cipher.decrypt(wrong_nonce, ciphertext.as_ref()) {
        Ok(_) => println!("✗ Decryption with wrong nonce succeeded (BUG!)"),
        Err(_) => println!("✓ Decryption with wrong nonce failed (expected)"),
    }

    Ok(())
}

fn chacha_example(plaintext: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|e| format!("Encryption failed: {}", e))?;

    println!("Algorithm: ChaCha20-Poly1305");
    println!("Ciphertext: {}", hex::encode(&ciphertext));

    let decrypted = cipher
        .decrypt(&nonce, ciphertext.as_ref())
        .map_err(|e| format!("Decryption failed: {}", e))?;

    assert_eq!(plaintext, &decrypted[..]);
    println!("✓ Decryption successful");

    Ok(())
}
