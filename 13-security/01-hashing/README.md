# Cryptographic Hashing

One-way hash functions for data integrity and verification.

## Hash Functions Covered

- **SHA-256**: Widely used, 256-bit output
- **SHA-512**: Higher security, 512-bit output
- **BLAKE3**: Modern, fast, parallelizable

## Use Cases

1. **File Integrity**: Verify files haven't been tampered with
2. **Data Deduplication**: Identify duplicate content
3. **Digital Signatures**: Part of signature schemes
4. **Checksums**: Verify data transmission
5. **HMAC**: Message authentication

## Properties of Cryptographic Hashes

1. **Deterministic**: Same input = same output
2. **Fast to compute**: Efficient hashing
3. **One-way**: Can't reverse to get original
4. **Avalanche effect**: Small change = completely different hash
5. **Collision resistant**: Hard to find two inputs with same hash

## Examples

### Basic Hashing

```rust
use sha2::{Sha256, Digest};

let mut hasher = Sha256::new();
hasher.update(b"hello world");
let result = hasher.finalize();
```

### HMAC (Message Authentication)

```rust
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

let mut mac = HmacSha256::new_from_slice(b"secret_key")?;
mac.update(b"message");
let code = mac.finalize().into_bytes();
```

## Important Notes

- **DON'T use for passwords**: Use Argon2, bcrypt instead
- **Use HMAC for authentication**: Plain hashes can be tampered
- **Choose appropriate algorithm**: SHA-256 for most cases, BLAKE3 for speed

## Running

```bash
cargo run
```

## References

- [SHA-2](https://en.wikipedia.org/wiki/SHA-2)
- [BLAKE3](https://github.com/BLAKE3-team/BLAKE3)
- [HMAC](https://en.wikipedia.org/wiki/HMAC)
