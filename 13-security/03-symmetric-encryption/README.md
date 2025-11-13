# Symmetric Encryption

Encrypt and decrypt data using a shared secret key with AEAD ciphers.

## Algorithms

- **AES-256-GCM**: Industry standard, hardware-accelerated
- **ChaCha20-Poly1305**: Modern, software-optimized

Both are AEAD (Authenticated Encryption with Associated Data) providing:
- **Confidentiality**: Data is encrypted
- **Integrity**: Tampering is detected
- **Authentication**: Verifies sender

## Key Concepts

- **Key**: Secret shared between parties (256 bits)
- **Nonce**: Number used once (96 bits), must be unique per message
- **Ciphertext**: Encrypted data
- **Tag**: Authentication tag (128 bits)

## Usage

```rust
use aes_gcm::{Aes256Gcm, KeyInit, Aead};

let key = Aes256Gcm::generate_key(OsRng);
let cipher = Aes256Gcm::new(&key);
let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

// Encrypt
let ciphertext = cipher.encrypt(&nonce, plaintext)?;

// Decrypt
let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref())?;
```

## When to Use

- Encrypting data at rest
- Secure communication (with key exchange)
- File encryption
- Database field encryption

## Running

```bash
cargo run
```
