# Cryptographically Secure Random Numbers

Generate unpredictable random values for security purposes.

## Important

- **Always use `OsRng`** for security-critical randomness
- **Never use** `rand::thread_rng()` for crypto
- **Never use** predictable seeds

## Use Cases

- Generating encryption keys
- Creating session tokens
- Nonces for encryption
- Salts for hashing

## Running

```bash
cargo run
```
