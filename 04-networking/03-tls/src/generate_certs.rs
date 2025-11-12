// Certificate Generation Helper
//
// This is a placeholder that shows how to generate certificates.
// In practice, use OpenSSL or similar tools.

fn main() {
    println!("=== Certificate Generation Guide ===\n");

    println!("To generate self-signed certificates for testing, use OpenSSL:\n");

    println!("1. Generate private key and certificate:");
    println!("   openssl req -x509 -newkey rsa:4096 -nodes \\");
    println!("     -keyout key.pem -out cert.pem -days 365 \\");
    println!("     -subj '/CN=localhost'\n");

    println!("2. Verify the certificate:");
    println!("   openssl x509 -in cert.pem -text -noout\n");

    println!("3. Files generated:");
    println!("   - key.pem: Private key (keep secret!)");
    println!("   - cert.pem: Public certificate\n");

    println!("For production use:");
    println!("- Get certificates from Let's Encrypt (free)");
    println!("- Use a commercial Certificate Authority");
    println!("- Never commit private keys to version control");
    println!("- Use proper hostname instead of 'localhost'");
    println!("- Set appropriate expiration dates\n");

    println!("Alternative tools:");
    println!("- mkcert: For local development");
    println!("- certbot: For Let's Encrypt");
    println!("- cfssl: CloudFlare's PKI toolkit\n");
}
