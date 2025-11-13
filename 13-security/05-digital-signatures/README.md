# Digital Signatures

Verify authenticity and integrity of messages using Ed25519.

## Concepts

- **Signing Key** (private): Signs messages
- **Verifying Key** (public): Verifies signatures
- **Signature**: Proves message authenticity

## Properties

1. **Authentication**: Proves who sent it
2. **Integrity**: Detects tampering
3. **Non-repudiation**: Signer can't deny signing

## Ed25519 Advantages

- Fast signing and verification
- Small keys (32 bytes) and signatures (64 bytes)
- Modern, secure algorithm
- Used in SSH, TLS 1.3, cryptocurrencies

## Running

```bash
cargo run
```
