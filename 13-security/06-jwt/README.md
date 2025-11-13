# JSON Web Tokens (JWT)

Compact, URL-safe tokens for authentication and information exchange.

## Structure

`header.payload.signature`

- **Header**: Algorithm and type
- **Payload**: Claims (user data)
- **Signature**: Verification

## Use Cases

- Stateless authentication
- API tokens
- OAuth 2.0
- Single Sign-On

## Best Practices

- Use strong secret
- Set expiration
- Use HTTPS only
- Don't store sensitive data
- Validate on every request

## Running

```bash
cargo run
```
