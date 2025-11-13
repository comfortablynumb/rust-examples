use quinn::{ClientConfig, Endpoint, ServerConfig};
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("QUIC Protocol Example\n");

    // Start server in background
    let server_handle = tokio::spawn(async {
        run_server().await.expect("Server failed");
    });

    // Give server time to start
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    // Run client
    run_client().await?;

    server_handle.abort();

    Ok(())
}

async fn run_server() -> Result<(), Box<dyn Error>> {
    let addr: SocketAddr = "127.0.0.1:5000".parse()?;

    let (server_config, _) = configure_server()?;
    let endpoint = Endpoint::server(server_config, addr)?;

    println!("QUIC server listening on {}", addr);

    // Accept one connection for demo
    if let Some(conn) = endpoint.accept().await {
        let connection = conn.await?;
        println!("Connection established from: {}", connection.remote_address());

        // Accept bidirectional stream
        if let Ok((mut send, mut recv)) = connection.accept_bi().await {
            println!("Accepted bidirectional stream");

            // Read data
            let data = recv.read_to_end(1024).await?;
            let message = String::from_utf8_lossy(&data);
            println!("Received: {}", message);

            // Send response
            let response = format!("Echo: {}", message);
            send.write_all(response.as_bytes()).await?;
            send.finish().await?;
            println!("Sent response");
        }
    }

    Ok(())
}

async fn run_client() -> Result<(), Box<dyn Error>> {
    let server_addr: SocketAddr = "127.0.0.1:5000".parse()?;

    let client_config = configure_client();
    let mut endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;
    endpoint.set_default_client_config(client_config);

    println!("\nConnecting to QUIC server at {}", server_addr);

    let connection = endpoint
        .connect(server_addr, "localhost")?
        .await?;

    println!("Connected to server");

    // Open bidirectional stream
    let (mut send, mut recv) = connection.open_bi().await?;
    println!("Opened bidirectional stream");

    // Send data
    let message = "Hello, QUIC!";
    send.write_all(message.as_bytes()).await?;
    send.finish().await?;
    println!("Sent: {}", message);

    // Receive response
    let response = recv.read_to_end(1024).await?;
    let response_str = String::from_utf8_lossy(&response);
    println!("Received: {}", response_str);

    // Demonstrate multiple streams
    println!("\n=== Multiple Streams ===");
    for i in 1..=3 {
        let (mut send, mut recv) = connection.open_bi().await?;
        let msg = format!("Message {}", i);
        send.write_all(msg.as_bytes()).await?;
        send.finish().await?;
        println!("Sent on stream {}: {}", i, msg);
    }

    // Close connection
    connection.close(0u32.into(), b"done");
    println!("\nConnection closed");

    // Wait for graceful shutdown
    endpoint.wait_idle().await;

    Ok(())
}

fn configure_server() -> Result<(ServerConfig, Vec<u8>), Box<dyn Error>> {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
    let cert_der = cert.serialize_der()?;
    let priv_key = cert.serialize_private_key_der();
    let priv_key = rustls::PrivateKey(priv_key);
    let cert_chain = vec![rustls::Certificate(cert_der.clone())];

    let mut server_config = ServerConfig::with_single_cert(cert_chain, priv_key)?;

    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
    transport_config.max_concurrent_uni_streams(0_u8.into());

    Ok((server_config, cert_der))
}

fn configure_client() -> ClientConfig {
    let crypto = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(SkipServerVerification::new())
        .with_no_client_auth();

    ClientConfig::new(Arc::new(crypto))
}

// Skip certificate verification for demo (DON'T use in production!)
struct SkipServerVerification;

impl SkipServerVerification {
    fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl rustls::client::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}
