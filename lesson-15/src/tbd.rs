
/*
Tokio
- one of two Rust's prevalent async Frameworks
- ecosystem comprised of many libs
    tokio and tokio crates (tokio_stream)
    mio - low-level non-blocking I/O
    tracing - logging
    tower - modular service composition
    prost & tonic - protobuf & gRPC
    hyper - low-level HTTP
    axum - high-level web framework
- tokio tasks start immediately (no need to use "await")
*/

// This example illustrates handling TCP streams using Tokio.

use std::time::Duration;

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::select;
use tokio::time::timeout;
use anyhow::{Result, anyhow};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let listener = TcpListener::bind("127.0.0.1:8080").await?;
    
    loop {
        let (mut socket, _) = select! {
            conn = listener.accept() => conn?,
            _ = timeout(Duration::from_secs(5), async {}) => break,
        };
        //let (mut socket, _) = listener.accept().await?;
    
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                // In a real-world application, you should handle errors appropriately
                match socket.read(&mut buf).await {
                    Ok(_) => {
                        // Echo back to the client
                        socket.write_all(&buf).await.unwrap();
                    }
                    Err(e) => {println!("Failed to read from socket: {:?}", e); break},
                }
            }
        });
    }
    Ok(())
}