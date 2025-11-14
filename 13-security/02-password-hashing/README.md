# Password Hashing

Secure password storage using specialized password hashing algorithms.

## Algorithms

### Argon2 (Recommended)
- Winner of Password Hashing Competition (2015)
- Resistant to GPU/ASIC attacks
- Configurable memory, time, parallelism
- Three variants: Argon2d, Argon2i, Argon2id

### bcrypt
- Time-tested and widely used
- Adaptive cost factor
- Resistant to rainbow tables

## Why Not Use SHA-256 for Passwords?

❌ **Too fast**: Enables brute-force attacks
❌ **No salt built-in**: Vulnerable to rainbow tables
❌ **Designed for speed**: Passwords need slow hashing

✅ **Use Argon2/bcrypt**: Designed specifically for passwords

## Usage

```rust
use argon2::{Argon2, PasswordHasher, PasswordVerifier};

// Hash
let salt = SaltString::generate(&mut OsRng);
let hash = Argon2::default()
    .hash_password(password, &salt)?
    .to_string();

// Verify
let parsed = PasswordHash::new(&hash)?;
Argon2::default().verify_password(password, &parsed)?;
```

## Security Properties

1. **Slow by design**: Thwarts brute-force
2. **Unique salt per password**: Prevents rainbow tables
3. **Adaptive cost**: Increase difficulty over time
4. **Memory-hard** (Argon2): Resists custom hardware

## Best Practices

- Use Argon2id for new applications
- Never decrease cost parameters
- Store full hash string (includes salt, params)
- Implement rate limiting on login
- Use HTTPS to protect password in transit

## Running

```bash
cargo run
```

## References

- [Argon2](https://github.com/P-H-C/phc-winner-argon2)
- [OWASP Password Storage](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html)
