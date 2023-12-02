use std::error::Error;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use uuid::Uuid;
use library::{get_addr, read_from_stream, write_to_stream, MessageType};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:11111").await?;
    let (tx, _rx) = broadcast::channel(10);

    loop {
        let (socket, _) = listener.accept().await?;
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        let client_id = Uuid::new_v4();

        tokio::spawn(async move {
            let (mut reader, mut writer) = socket.into_split();
            let mut buffer = vec![0; 1024];

            loop {
                tokio::select! {
                    /*
                    result = reader.read(&mut buffer) => {
                        let n = match result {
                            Ok(0) => return,
                            Ok(n) => n,
                            Err(e) => {
                                eprintln!("Failed to read from socket: {}", e);
                                return;
                            }
                        };

                        let msg = match String::from_utf8(buffer[..n].to_vec()) {
                            Ok(msg) => msg,
                            Err(e) => {
                                eprintln!("Failed to parse message: {}", e);
                                continue;
                            }
                        };

                        let broadcast_msg = (client_id, msg);

                        if tx.send(broadcast_msg).is_err() {
                            break;
                        }
                    }
                     */
                    result = read_from_stream(reader) => {
                        let n = match &result {
                            Ok((_, msg)) => {
                                match &msg {
                                    MessageType::Error(e) => {
                                        return log::error!("Error: {}", e)
                                    }
                                    _ => (),
                                }
                            },
                            Err(e) => {
                                return log::error!("Error: {}", e)
                            }
                        };
                        let (r, msg) = result.unwrap();
                        let broadcast_msg = (client_id, msg);
                        reader = r;

                        if tx.send(broadcast_msg).is_err() {
                            break;
                        }
                    }
                    result = rx.recv() => {
                        let received = match result {
                            Ok(received) => received,
                            Err(e) => {
                                eprintln!("Failed to receive broadcast message: {}", e);
                                continue;
                            }
                        };

                        let (sender_id, msg) = received;

                        if sender_id != client_id {
                            let result = write_to_stream(writer, &msg).await;
                            match result {
                                Ok(_) => (),
                                Err(e) => {
                                    log::error!("Disconnecting client: {}", e);
                                    ()
                                },
                            }
                        }
                    }
                }
            }
        });
    }
}
