use rcgen::{Certificate, CertificateParams, DistinguishedName, DnType};
use std::time::SystemTime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("TLS Certificate Generation Example\n");

    // Generate self-signed certificate
    println!("=== Generating Self-Signed Certificate ===");
    let cert = generate_self_signed_cert()?;

    // Display certificate info
    println!("Certificate generated!");
    println!("\nCertificate (PEM):\n{}", cert.serialize_pem()?);
    println!("Private Key (PEM):\n{}", cert.serialize_private_key_pem());

    // Certificate info
    let params = cert.get_params();
    println!("\n=== Certificate Information ===");
    println!("Not Before: {:?}", params.not_before);
    println!("Not After:  {:?}", params.not_after);

    Ok(())
}

fn generate_self_signed_cert() -> Result<Certificate, Box<dyn std::error::Error>> {
    let mut params = CertificateParams::new(vec!["localhost".to_string()]);

    // Set subject (who the certificate is for)
    let mut dn = DistinguishedName::new();
    dn.push(DnType::CommonName, "localhost");
    dn.push(DnType::OrganizationName, "Example Org");
    dn.push(DnType::CountryName, "US");
    params.distinguished_name = dn;

    // Set validity period
    let now = SystemTime::now();
    let one_year = std::time::Duration::from_secs(365 * 24 * 60 * 60);
    params.not_before = now;
    params.not_after = now + one_year;

    // Generate certificate
    let cert = Certificate::from_params(params)?;

    Ok(cert)
}
