use std::error::Error;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let stream = TcpStream::connect("127.0.0.1:8080").await?;

    let (mut reader, mut writer) = tokio::io::split(stream);

    let write_task = tokio::spawn(async move {
        let mut input = String::new();

        loop {
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap(); // blocking read -> not an issue with Tokio though
            writer
                .write_all(input.as_bytes())
                .await
                .expect("Failed to send bytes");
        }
    });

    let read_task = tokio::spawn(async move {
        let mut buffer = vec![0; 1024]; // 1024 elements, all set to 0
        
        loop {
            let n = match reader.read(&mut buffer).await {
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
            )
        }
    });

    let _ = tokio::try_join!(write_task, read_task);

    Ok(())
}
