use tonic::{transport::Server, Request, Response, Status};

pub mod greeter {
    tonic::include_proto!("greeter");
}

use greeter::greeter_server::{Greeter, GreeterServer};
use greeter::{HelloReply, HelloRequest};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Received request from: {:?}", request.remote_addr());

        let name = request.into_inner().name;
        let reply = HelloReply {
            message: format!("Hello, {}!", name),
        };

        Ok(Response::new(reply))
    }

    type SayHelloStreamStream = tokio_stream::wrappers::ReceiverStream<Result<HelloReply, Status>>;

    async fn say_hello_stream(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<Self::SayHelloStreamStream>, Status> {
        let name = request.into_inner().name;
        let (tx, rx) = tokio::sync::mpsc::channel(4);

        tokio::spawn(async move {
            for i in 1..=5 {
                let reply = HelloReply {
                    message: format!("Hello {}, message {}", name, i),
                };

                if tx.send(Ok(reply)).await.is_err() {
                    break;
                }

                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
        });

        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("gRPC Example\n");

    let addr = "127.0.0.1:50051".parse()?;
    let greeter = MyGreeter::default();

    println!("Starting gRPC server on {}", addr);

    // Spawn server in background
    tokio::spawn(async move {
        Server::builder()
            .add_service(GreeterServer::new(greeter))
            .serve(addr)
            .await
            .expect("Server failed");
    });

    // Give server time to start
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    // Run client
    run_client().await?;

    Ok(())
}

async fn run_client() -> Result<(), Box<dyn std::error::Error>> {
    use greeter::greeter_client::GreeterClient;

    println!("\nConnecting to gRPC server...");
    let mut client = GreeterClient::connect("http://127.0.0.1:50051").await?;

    // Unary call
    println!("\n=== Unary Call ===");
    let request = Request::new(HelloRequest {
        name: "Alice".into(),
    });

    let response = client.say_hello(request).await?;
    println!("Response: {}", response.into_inner().message);

    // Streaming call
    println!("\n=== Server Streaming Call ===");
    let request = Request::new(HelloRequest {
        name: "Bob".into(),
    });

    let mut stream = client.say_hello_stream(request).await?.into_inner();

    while let Some(response) = stream.message().await? {
        println!("Received: {}", response.message);
    }

    println!("\nAll requests completed!");

    Ok(())
}
