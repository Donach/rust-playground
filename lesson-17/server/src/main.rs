//! Server of chat application, able to receive and send messages from/to clients
//! 
//! # Usage
//! 
//! ```
//! cargo run --bin server <hostname> <port>
//! ```
//! 
use library::db_client::{auth_client, save_message, setup_database_pool};
use library::{read_from_stream, write_to_stream, MessageType, get_addr};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    dotenvy::dotenv()?;
    let (addr, _) = get_addr(env::args().collect()).unwrap();
    let listener = TcpListener::bind(addr).await?;
    let (tx, _rx) = broadcast::channel(10);
    let clients: Arc<Mutex<HashMap<SocketAddr, Uuid>>> = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let clients = Arc::clone(&clients);
        let (socket, socket_addr) = listener.accept().await?;
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (mut reader, mut writer) = socket.into_split();
            let db_pool = setup_database_pool().await;
            match &db_pool {
                Ok(pool) => {
                    log::info!("Connected to database: {:?}", pool);
                }
                Err(e) => {
                    log::error!("Failed to connect to database: {}", e);
                    return;
                }
            };
            let db_pool = db_pool.unwrap();
            loop {
                tokio::select! {
                    result = read_from_stream(&mut reader) => {
                        match &result {
                            Ok(msg) => {
                                match &msg {
                                    MessageType::Error(e) => {
                                        return log::error!("Error: {}", e)
                                    }
                                    MessageType::Auth(client_id) => {
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
                                                    if tx.send(broadcast_msg).is_err() {
                                                        break;
                                                    }
                                                    clients.lock().unwrap().insert(socket_addr, Uuid::try_parse(&uid.to_string()).unwrap());

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
                                        let broadcast_msg = (addr, msg.clone());

                                        if tx.send(broadcast_msg).is_err() {
                                            break;
                                        } else {
                                            let uid = &clients.lock().unwrap().get(&socket_addr).unwrap().clone();
                                            // Save message to DB
                                            match save_message(&db_pool, uid.to_string(), &msg).await.is_err() {
                                                false => (),
                                                true => {
                                                    log::error!("Cannot save message to DB");
                                                }
                                            };

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

                        let send_msg = match &msg {
                            MessageType::Auth(s) => {
                                log::info!("Authenticating client: {}", s);
                                recv_socket_addr.as_str() == socket_addr.to_string()
                            }
                            _ => {
                                recv_socket_addr.as_str() != socket_addr.to_string()
                            }
                        };

                        if send_msg {
                            let result = write_to_stream(&mut writer, &msg).await;
                            match result {
                                Ok(_) => (),
                                Err(e) => {
                                    log::error!("Disconnecting client: {}", e);
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
