use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,  // Subject (user ID)
    exp: usize,   // Expiration time
    iat: usize,   // Issued at
    role: String, // Custom claim
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("JSON Web Tokens (JWT) Example\n");

    let secret = "my_secret_key";

    // Create token
    println!("=== Creating JWT ===");
    let token = create_jwt(secret, "user123", "admin")?;
    println!("Token: {}\n", token);

    // Decode and display parts
    let parts: Vec<&str> = token.split('.').collect();
    println!("Token structure:");
    println!("  Header:    {}", parts[0]);
    println!("  Payload:   {}", parts[1]);
    println!("  Signature: {}", parts[2]);

    // Verify token
    println!("\n=== Verifying JWT ===");
    match verify_jwt(&token, secret) {
        Ok(claims) => {
            println!("✓ Token valid");
            println!("  User:  {}", claims.sub);
            println!("  Role:  {}", claims.role);
            println!("  Exp:   {}", claims.exp);
        }
        Err(e) => println!("✗ Token invalid: {}", e),
    }

    // Try with wrong secret
    println!("\n=== Tamper Detection ===");
    match verify_jwt(&token, "wrong_secret") {
        Ok(_) => println!("✗ Token verified with wrong secret (BUG!)"),
        Err(_) => println!("✓ Token rejected with wrong secret"),
    }

    // Expired token
    println!("\n=== Expired Token ===");
    let expired_token = create_expired_jwt(secret, "user456", "user")?;
    match verify_jwt(&expired_token, secret) {
        Ok(_) => println!("✗ Expired token accepted (BUG!)"),
        Err(e) => println!("✓ Expired token rejected: {}", e),
    }

    println!("\n=== JWT Use Cases ===");
    println!("✓ API authentication");
    println!("✓ Single Sign-On (SSO)");
    println!("✓ Information exchange");
    println!("✓ Stateless sessions");

    Ok(())
}

fn create_jwt(
    secret: &str,
    user_id: &str,
    role: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration as usize,
        iat: Utc::now().timestamp() as usize,
        role: role.to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

fn create_expired_jwt(
    secret: &str,
    user_id: &str,
    role: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims {
        sub: user_id.to_string(),
        exp: (Utc::now() - Duration::hours(1)).timestamp() as usize,
        iat: (Utc::now() - Duration::hours(2)).timestamp() as usize,
        role: role.to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

fn verify_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )?;

    Ok(token_data.claims)
}
