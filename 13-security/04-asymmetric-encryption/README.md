# Asymmetric Encryption (RSA)

Public-key cryptography for secure key exchange and encryption.

## Concepts

- **Public Key**: Can be shared, used to encrypt
- **Private Key**: Must be kept secret, used to decrypt
- **RSA**: Most common asymmetric algorithm

## Use Cases

- Key exchange (encrypt symmetric keys)
- Digital signatures
- SSH authentication
- TLS/SSL certificates

## Important Notes

- Much slower than symmetric encryption
- Limited message size (typically < key size)
- Use hybrid: RSA for key exchange, AES for data

## Running

```bash
cargo run
```
