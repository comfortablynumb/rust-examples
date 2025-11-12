# DNS Server and Client

This example demonstrates DNS (Domain Name System) operations using the hickory-dns library.

## Concepts Covered

### DNS Basics
- **Name Resolution**: Convert hostnames to IP addresses
- **Record Types**: A, AAAA, MX, TXT, CNAME, NS, SOA
- **DNS Queries**: Request information about a domain
- **DNS Responses**: Answer queries with records
- **TTL**: Time To Live - how long to cache records

### Implementation
- DNS client making queries
- DNS server responding to queries
- Record types (A, AAAA, MX, TXT, CNAME)
- UDP-based communication (port 53)
- Binary protocol encoding/decoding

## Running the Examples

### DNS Client

Query public DNS servers:
```bash
cargo run --bin client
```

This queries Google's public DNS (8.8.8.8) for various domains and record types.

### DNS Resolver

Simple hostname resolution:
```bash
cargo run --bin resolver
```

### DNS Server

**Terminal 1 - Server:**
```bash
cargo run --bin server
```

**Terminal 2 - Test with dig:**
```bash
# Query the local DNS server
dig @127.0.0.1 -p 5353 example.local
dig @127.0.0.1 -p 5353 test.local

# Or use nslookup
nslookup example.local 127.0.0.1:5353
```

## DNS Record Types

| Type | Purpose | Example |
|------|---------|---------|
| A | IPv4 address | 93.184.216.34 |
| AAAA | IPv6 address | 2606:2800:220:1:248:1893:25c8:1946 |
| CNAME | Canonical name (alias) | www -> example.com |
| MX | Mail exchange server | Priority 10: mail.example.com |
| TXT | Text information | "v=spf1 include:_spf.google.com" |
| NS | Name server | ns1.example.com |
| SOA | Start of authority | Primary nameserver info |
| PTR | Reverse lookup | IP -> hostname |

## Key Takeaways

1. **DNS uses UDP port 53** - Fast, connectionless queries
2. **Hierarchical system** - Root -> TLD -> Domain -> Subdomain
3. **Caching is critical** - TTL determines cache duration
4. **Multiple records** - One domain can have many IPs
5. **Security matters** - DNSSEC prevents tampering

## DNS Query Process

1. **Client** sends query to DNS server
2. **Recursive resolver** queries authoritative servers if needed
3. **Root servers** direct to TLD servers (.com, .org, etc.)
4. **TLD servers** direct to authoritative nameservers
5. **Authoritative server** returns answer
6. **Resolver** caches and returns result to client

## Common Use Cases

### A Record Lookup
```rust
let name = Name::from_str("example.com")?;
let response = client.query(&name, DNSClass::IN, RecordType::A)?;
```

### MX Record Lookup (Mail Servers)
```rust
let response = client.query(&name, DNSClass::IN, RecordType::MX)?;
for record in response.answers() {
    if let RData::MX(mx) = record.data() {
        println!("{}: {}", mx.preference(), mx.exchange());
    }
}
```

### Reverse DNS (PTR)
```rust
// Lookup hostname from IP
let name = Name::from_str("34.216.184.93.in-addr.arpa")?;
let response = client.query(&name, DNSClass::IN, RecordType::PTR)?;
```

## Public DNS Servers

| Provider | IPv4 | IPv6 |
|----------|------|------|
| Google | 8.8.8.8, 8.8.4.4 | 2001:4860:4860::8888 |
| Cloudflare | 1.1.1.1, 1.0.0.1 | 2606:4700:4700::1111 |
| Quad9 | 9.9.9.9 | 2620:fe::fe |
| OpenDNS | 208.67.222.222 | 2620:119:35::35 |

## Testing Tools

### dig (Domain Information Groper)
```bash
# Basic query
dig example.com

# Specific record type
dig example.com MX

# Use specific DNS server
dig @8.8.8.8 example.com

# Reverse lookup
dig -x 93.184.216.34

# Trace query path
dig +trace example.com
```

### nslookup
```bash
# Interactive mode
nslookup
> example.com
> set type=MX
> example.com

# Direct query
nslookup example.com 8.8.8.8
```

### host
```bash
# Simple lookup
host example.com

# Specific type
host -t MX example.com

# Use specific server
host example.com 8.8.8.8
```

## DNS Security

### DNSSEC
- Cryptographic signatures on DNS records
- Prevents DNS spoofing and cache poisoning
- Chain of trust from root to domain

### DNS over HTTPS (DoH)
- Encrypt DNS queries over HTTPS
- Port 443 instead of 53
- Prevents eavesdropping

### DNS over TLS (DoT)
- Encrypt DNS queries over TLS
- Port 853
- Similar to DoH but different protocol

## Performance Considerations

1. **Caching** - Respect TTL values
2. **Parallel queries** - Query multiple servers simultaneously
3. **Connection pooling** - Reuse connections when possible
4. **Timeouts** - Set reasonable query timeouts
5. **Fallback servers** - Have backup DNS servers

## Common Issues

### DNS Resolution Fails
- Check network connectivity
- Verify DNS server is reachable
- Check firewall rules (UDP port 53)
- Try different DNS server

### Slow Resolution
- DNS server overloaded
- Network latency
- No caching
- Too many redirects

### NXDOMAIN (Domain Not Found)
- Domain doesn't exist
- Typo in domain name
- DNS propagation delay (new domains)

## Best Practices

1. **Use public DNS** for testing
2. **Implement caching** - Respect TTL
3. **Handle errors gracefully** - Timeouts, NXDOMAIN
4. **Use DoH/DoT** for privacy
5. **Monitor DNS health** - Check availability
6. **Set appropriate TTLs** - Balance freshness vs load

## Production Considerations

For production DNS servers:
- **Authoritative DNS** - BIND, PowerDNS, Knot DNS
- **Recursive DNS** - Unbound, dnsmasq
- **Cloud DNS** - AWS Route53, Cloudflare DNS, Google Cloud DNS
- **Load balancing** - GeoDNS, anycast
- **DDoS protection** - Rate limiting, filtering
- **Monitoring** - Query logs, performance metrics

## Resources

- [DNS RFC 1035](https://tools.ietf.org/html/rfc1035)
- [hickory-dns documentation](https://docs.rs/hickory-client)
- [DNS over HTTPS (DoH)](https://tools.ietf.org/html/rfc8484)
- [DNSSEC](https://www.dnssec.net/)
- [Root servers](https://www.iana.org/domains/root/servers)
