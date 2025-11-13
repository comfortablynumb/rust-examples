use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding};
use rand::rngs::OsRng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Asymmetric Encryption (RSA) Example\n");

    let mut rng = OsRng;

    // Generate keypair
    println!("=== Key Generation ===");
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits)?;
    let public_key = RsaPublicKey::from(&private_key);

    println!("Generated {}-bit RSA keypair", bits);
    println!("Public key modulus: {} bits", public_key.size() * 8);

    // Export keys
    let private_pem = private_key.to_pkcs8_pem(LineEnding::LF)?;
    let public_pem = public_key.to_public_key_pem(LineEnding::LF)?;

    println!("\nPublic Key (PEM):\n{}", public_pem);

    // Encryption
    println!("=== Encryption ===");
    let data = b"Hello, RSA!";
    println!("Plaintext: {:?}", std::str::from_utf8(data)?);

    let ciphertext = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, data)?;
    println!("Ciphertext: {}", hex::encode(&ciphertext));

    // Decryption
    println!("\n=== Decryption ===");
    let decrypted = private_key.decrypt(Pkcs1v15Encrypt, &ciphertext)?;
    println!("Decrypted: {:?}", std::str::from_utf8(&decrypted)?);

    assert_eq!(data, &decrypted[..]);
    println!("✓ Decryption successful");

    // Key properties
    println!("\n=== RSA Properties ===");
    println!("✓ Public key can encrypt, private key decrypts");
    println!("✓ Can't decrypt without private key");
    println!("✓ Slower than symmetric encryption");
    println!("✓ Typically used to exchange symmetric keys");

    Ok(())
}
