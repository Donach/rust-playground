use std::error::Error;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:11111").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            loop {
                let mut buffer = vec![0; 1024]; // 1024 elements, all set to 0
            
                let n = match socket.read(&mut buffer).await {
                    Ok(0) => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("It failted: {}", e);
                        return
                    },
                };

                println!(
                    "received message : {}",
                    String::from_utf8_lossy(&buffer[..n])
                );

                if socket.write_all(&buffer).await.is_err(){
                    break;
                }
            }
        });
    }




}