use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Digital Signatures (Ed25519) Example\n");

    let mut csprng = OsRng;

    // Generate keypair
    println!("=== Key Generation ===");
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key: VerifyingKey = (&signing_key).into();

    println!("Signing key:    {}", hex::encode(signing_key.to_bytes()));
    println!("Verifying key:  {}", hex::encode(verifying_key.to_bytes()));

    // Sign a message
    println!("\n=== Signing ===");
    let message = b"This is an authentic message";
    println!("Message: {:?}", std::str::from_utf8(message)?);

    let signature: Signature = signing_key.sign(message);
    println!("Signature: {}", hex::encode(signature.to_bytes()));

    // Verify signature
    println!("\n=== Verification ===");
    match verifying_key.verify(message, &signature) {
        Ok(()) => println!("✓ Signature valid"),
        Err(_) => println!("✗ Signature invalid"),
    }

    // Try to verify with modified message
    let tampered = b"This is a TAMPERED message";
    match verifying_key.verify(tampered, &signature) {
        Ok(()) => println!("✗ Tampered message verified (BUG!)"),
        Err(_) => println!("✓ Tampered message rejected"),
    }

    // Multiple messages
    println!("\n=== Multiple Signatures ===");
    let messages = [
        b"Transaction: Send $100 to Alice" as &[u8],
        b"Transaction: Send $50 to Bob",
        b"Document v1.2 approved",
    ];

    for msg in messages.iter() {
        let sig = signing_key.sign(msg);
        match verifying_key.verify(msg, &sig) {
            Ok(()) => println!("✓ {:?} - Valid", std::str::from_utf8(msg)?),
            Err(_) => println!("✗ {:?} - Invalid", std::str::from_utf8(msg)?),
        }
    }

    println!("\n=== Use Cases ===");
    println!("✓ Software signing (verify authenticity)");
    println!("✓ Git commits (verify author)");
    println!("✓ Blockchain transactions");
    println!("✓ API authentication");
    println!("✓ Document signing");

    Ok(())
}
