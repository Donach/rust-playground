use crate::db_client::{setup_database_pool, auth_client, save_message};
use library::{get_addr, read_from_stream, write_to_stream, MessageType, serialize_message};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use uuid::Uuid;
mod db_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    dotenvy::dotenv()?;
   
    let listener = TcpListener::bind("127.0.0.1:11111").await?;
    let (tx, _rx) = broadcast::channel(10);
    let mut clients: Arc<Mutex<HashMap<SocketAddr, Uuid>>> = Arc::new(Mutex::new(HashMap::new()));
    

    loop {
        let (socket, socket_addr) = listener.accept().await?;
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        //let client_id = Uuid::new_v4();

        tokio::spawn(async move {
            let (mut reader, mut writer) = socket.into_split();
            let db_pool = setup_database_pool().await;
            match &db_pool {
                Ok(pool) => {
                    log::info!("Connected to database: {:?}", pool);
                },
                Err(e) => {
                    log::error!("Failed to connect to database: {}", e);
                    return
                }
            };
            let db_pool = db_pool.unwrap();
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
                    result = read_from_stream(&mut reader) => {
                        let n = match &result {
                            Ok(msg) => {
                                match &msg {
                                    MessageType::Error(e) => {
                                        return log::error!("Error: {}", e)
                                    }
                                    MessageType::Auth(client_id) => {
                                        // TODO: Auth user
                                        log::info!("Authenticating client: {}", client_id);
                                        let uid = Uuid::try_parse(client_id);
                                        if uid.is_err() {
                                            return log::error!("Wrong UID supplied by client: {}", uid.unwrap_err());
                                        } else {
                                            match auth_client(&db_pool, uid.unwrap()).await {
                                                Ok(uid) => {
                                                    log::info!("Authenticated client: {}", uid.to_string());
                                                    
                                                    let msg = result.unwrap();
                                                    let addr = socket_addr.to_string().clone();
                                                    let broadcast_msg = (addr, msg);
                                                    //&clients.lock().unwrap().insert(socket_addr, Uuid::try_parse(&uid.to_string()).unwrap());

                                                    if tx.send(broadcast_msg).is_err() {
                                                        break;
                                                    }
                                                },
                                                Err(e) => {
                                                    return log::error!("Error: {}", e)
                                                }
                                            };
                                        }
                                    }
                                    _ => {
                                        let msg = result.unwrap();
                                        let addr = socket_addr.to_string().clone();
                                        let broadcast_msg = (addr, msg);

                                        if tx.send(broadcast_msg).is_err() {
                                            break;
                                        } else {
                                            //let uid = &clients.lock().unwrap().get(&socket_addr).unwrap();
                                            // Save message to DB
                                            //save_message(&db_pool, uid.to_string(), serialize_message(&msg).unwrap());
                                        }

                                    },
                                }
                            },
                            Err(e) => {
                                return log::error!("Error: {}", e)
                            }
                        };
                    }
                    result = rx.recv() => {
                        let received = match result {
                            Ok(received) => received,
                            Err(e) => {
                                eprintln!("Failed to receive broadcast message: {}", e);
                                continue;
                            }
                        };

                        let (recv_socket_addr, msg) = received;
                        let mut send_msg = false;
                        match &msg {
                            MessageType::Auth(s) => {
                                log::info!("Authenticating client: {}", s);
                                send_msg = recv_socket_addr.as_str() == &socket_addr.to_string();
                            }
                            _ => {
                                send_msg = recv_socket_addr.as_str() != &socket_addr.to_string();
                            }
                        }

                        if send_msg {
                            let result = write_to_stream(&mut writer, &msg).await;
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
        //clients = clients.clone();
    }
}
