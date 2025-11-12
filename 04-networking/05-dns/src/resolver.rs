#![allow(clippy::useless_vec)]

// DNS Resolver Example
//
// Demonstrates using a DNS resolver to look up hostnames.
// This uses the system's DNS configuration automatically.

use hickory_client::client::{Client, SyncClient};
use hickory_client::rr::{DNSClass, Name, RecordType};
use hickory_client::udp::UdpClientConnection;
use std::net::{Ipv4Addr, SocketAddr};
use std::str::FromStr;

fn resolve_hostname(hostname: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "=".repeat(60));
    println!("Resolving: {}", hostname);
    println!("{}", "=".repeat(60));

    // Use Google DNS 8.8.8.8
    let dns_server = SocketAddr::from((Ipv4Addr::new(8, 8, 8, 8), 53));
    let conn = UdpClientConnection::new(dns_server)?;
    let client = SyncClient::new(conn);

    let name = Name::from_str(hostname)?;

    // Try A record (IPv4)
    match client.query(&name, DNSClass::IN, RecordType::A) {
        Ok(response) => {
            println!("\nIPv4 Addresses:");
            let answers = response.answers();
            if answers.is_empty() {
                println!("  (none)");
            } else {
                for record in answers {
                    if let Some(data) = record.data() {
                        println!("  {} (TTL: {}s)", data, record.ttl());
                    }
                }
            }
        }
        Err(e) => println!("\nIPv4 lookup failed: {}", e),
    }

    // Try CNAME record
    // CNAME not required, so we ignore errors
    if let Ok(response) = client.query(&name, DNSClass::IN, RecordType::CNAME) {
        let answers = response.answers();
        if !answers.is_empty() {
            println!("\nCanonical Name:");
            for record in answers {
                if let Some(data) = record.data() {
                    println!("  {}", data);
                }
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== DNS Resolver ===");

    // Resolve several hostnames
    let hostnames = vec![
        "example.com",
        "google.com",
        "github.com",
        "crates.io",
        "docs.rs",
    ];

    for hostname in hostnames {
        resolve_hostname(hostname)?;
    }

    println!("\n{}", "=".repeat(60));
    println!("\nDNS Record Types:");
    println!("  A     - IPv4 address");
    println!("  AAAA  - IPv6 address");
    println!("  CNAME - Canonical name (alias)");
    println!("  MX    - Mail exchange server");
    println!("  TXT   - Text records");
    println!("  NS    - Name server");
    println!("  SOA   - Start of authority");

    Ok(())
}
