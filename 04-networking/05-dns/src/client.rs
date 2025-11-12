#![allow(clippy::useless_vec)]

// DNS Client Example
//
// Demonstrates performing DNS queries to resolve hostnames to IP addresses.

use hickory_client::client::{Client, SyncClient};
use hickory_client::rr::{DNSClass, Name, RData, RecordType};
use hickory_client::udp::UdpClientConnection;
use std::net::{Ipv4Addr, SocketAddr};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== DNS Client ===\n");

    // Connect to Google's public DNS (8.8.8.8)
    let dns_server = SocketAddr::from((Ipv4Addr::new(8, 8, 8, 8), 53));
    println!("Using DNS server: {}\n", dns_server);

    let conn = UdpClientConnection::new(dns_server)?;
    let client = SyncClient::new(conn);

    // Domains to resolve
    let domains = vec!["google.com", "github.com", "rust-lang.org", "localhost"];

    for domain in domains {
        println!("{:=<60}", "");
        println!("Resolving: {}", domain);
        println!("{:=<60}", "");

        // Create domain name
        let name = Name::from_str(domain)?;

        // Query A records (IPv4)
        println!("\n[A Records - IPv4]");
        match client.query(&name, DNSClass::IN, RecordType::A) {
            Ok(response) => {
                let answers = response.answers();
                if answers.is_empty() {
                    println!("  No A records found");
                } else {
                    for record in answers {
                        if let Some(RData::A(addr)) = record.data() {
                            println!("  {} -> {}", domain, addr);
                        }
                    }
                }
            }
            Err(e) => println!("  Error: {}", e),
        }

        // Query AAAA records (IPv6)
        println!("\n[AAAA Records - IPv6]");
        match client.query(&name, DNSClass::IN, RecordType::AAAA) {
            Ok(response) => {
                let answers = response.answers();
                if answers.is_empty() {
                    println!("  No AAAA records found");
                } else {
                    for record in answers {
                        if let Some(RData::AAAA(addr)) = record.data() {
                            println!("  {} -> {}", domain, addr);
                        }
                    }
                }
            }
            Err(e) => println!("  Error: {}", e),
        }

        // Query MX records (Mail servers)
        println!("\n[MX Records - Mail Servers]");
        match client.query(&name, DNSClass::IN, RecordType::MX) {
            Ok(response) => {
                let answers = response.answers();
                if answers.is_empty() {
                    println!("  No MX records found");
                } else {
                    for record in answers {
                        if let Some(RData::MX(mx)) = record.data() {
                            println!("  Priority {}: {}", mx.preference(), mx.exchange());
                        }
                    }
                }
            }
            Err(e) => println!("  Error: {}", e),
        }

        // Query TXT records
        println!("\n[TXT Records]");
        match client.query(&name, DNSClass::IN, RecordType::TXT) {
            Ok(response) => {
                let answers = response.answers();
                if answers.is_empty() {
                    println!("  No TXT records found");
                } else {
                    for record in answers {
                        if let Some(RData::TXT(txt)) = record.data() {
                            for data in txt.iter() {
                                println!("  {}", String::from_utf8_lossy(data));
                            }
                        }
                    }
                }
            }
            Err(e) => println!("  Error: {}", e),
        }

        println!();
    }

    Ok(())
}
