# TLS (Transport Layer Security)

This example demonstrates TLS encrypted communication using rustls, a modern TLS library written in Rust.

## Concepts Covered

### TLS/SSL
- **Encryption**: Protects data from eavesdropping
- **Authentication**: Verifies server/client identity
- **Integrity**: Prevents tampering with data
- **Modern protocols**: TLS 1.2 and 1.3

### Implementation
- rustls - Pure Rust TLS implementation
- Certificate loading (PEM format)
- Private key management
- TLS handshake
- Encrypted stream communication

## Setup

### Generate Certificates

Before running the server, generate test certificates:

```bash
# Generate self-signed certificate (for testing only)
openssl req -x509 -newkey rsa:4096 -nodes \
  -keyout key.pem -out cert.pem -days 365 \
  -subj '/CN=localhost'
```

This creates:
- `cert.pem` - Public certificate
- `key.pem` - Private key

### View Certificate Generation Guide

```bash
cargo run --bin generate_certs
```

## Running the Example

**Terminal 1 - Server:**
```bash
cargo run --bin server
```

**Terminal 2 - Client:**
```bash
cargo run --bin client
```

## Key Takeaways

1. **Certificates required** - Server needs certificate and private key
2. **Root certificates** - Client validates server certificate
3. **Handshake** - TLS negotiation before data exchange
4. **Transparent encryption** - Same API as TCP after setup
5. **Performance cost** - Encryption adds CPU overhead

## Security Considerations

### For Testing
- Self-signed certificates work but show warnings
- Browser/tools will warn about untrusted certificates
- Fine for localhost development

### For Production
- **Use proper certificates** from Let's Encrypt or CA
- **Keep private keys secret** - never commit to git
- **Set expiration dates** appropriately
- **Use TLS 1.2+** (TLS 1.0/1.1 deprecated)
- **Validate certificates** properly
- **Use strong ciphers** (rustls does this by default)

## Common Patterns

### Server Setup
```rust
// Load certificate and key
let certs = load_certs("cert.pem")?;
let key = load_private_key("key.pem")?;

// Configure TLS
let config = ServerConfig::builder()
    .with_no_client_auth()
    .with_single_cert(certs, key)?;

// Accept connections
let mut conn = ServerConnection::new(Arc::new(config))?;
let mut tls_stream = StreamOwned::new(conn, tcp_stream);
```

### Client Setup
```rust
// Load root certificates
let mut root_store = RootCertStore::empty();
root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

// Configure TLS
let config = ClientConfig::builder()
    .with_root_certificates(root_store)
    .with_no_client_auth();

// Connect
let server_name = ServerName::try_from("example.com")?;
let mut conn = ClientConnection::new(Arc::new(config), server_name)?;
let mut tls_stream = StreamOwned::new(conn, tcp_stream);
```

## Certificate Formats

### PEM (Base64 encoded)
```
-----BEGIN CERTIFICATE-----
MIIDXTCCAkWgAwIBAgIJAKL...
-----END CERTIFICATE-----
```

### Key Types
- **RSA**: Traditional, widely supported
- **ECDSA**: Faster, smaller keys
- **Ed25519**: Modern, recommended

## Error Handling

Common TLS errors:
- **Certificate validation failed**: Expired, wrong hostname, untrusted CA
- **Handshake failure**: Protocol mismatch, incompatible ciphers
- **Connection closed**: Peer closed connection abruptly
- **Bad certificate**: Invalid format, corrupted file

## Tools

### OpenSSL Commands
```bash
# Generate self-signed cert
openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365

# View certificate
openssl x509 -in cert.pem -text -noout

# View certificate chain
openssl s_client -connect example.com:443 -showcerts

# Test TLS connection
openssl s_client -connect localhost:8443
```

### mkcert (for local development)
```bash
# Install mkcert
brew install mkcert  # macOS
# or download from https://github.com/FiloSottile/mkcert

# Generate local CA
mkcert -install

# Generate certificate
mkcert localhost 127.0.0.1
```

## TLS Versions

| Version | Status | Notes |
|---------|--------|-------|
| SSL 3.0 | Deprecated | Insecure, don't use |
| TLS 1.0 | Deprecated | Insecure, don't use |
| TLS 1.1 | Deprecated | Insecure, don't use |
| TLS 1.2 | Current | Widely supported |
| TLS 1.3 | Current | Faster, more secure |

rustls supports TLS 1.2 and 1.3 by default.

## Best Practices

1. **Use Let's Encrypt** for free, trusted certificates
2. **Enable HTTP/2** over TLS for better performance
3. **Implement certificate pinning** for mobile apps
4. **Monitor certificate expiration**
5. **Use secure random** for key generation
6. **Rotate certificates** before expiration
7. **Test with SSL Labs** (https://www.ssllabs.com/ssltest/)

## rustls vs OpenSSL

| Feature | rustls | OpenSSL |
|---------|--------|---------|
| Language | Pure Rust | C |
| Memory safety | Yes | No |
| TLS 1.3 | Yes | Yes (1.1.1+) |
| Size | Smaller | Larger |
| FIPS | No | Yes |
| Maturity | Newer | Very mature |

## Resources

- [rustls documentation](https://docs.rs/rustls)
- [Let's Encrypt](https://letsencrypt.org/)
- [Mozilla SSL Configuration Generator](https://ssl-config.mozilla.org/)
- [TLS 1.3 RFC](https://tools.ietf.org/html/rfc8446)
