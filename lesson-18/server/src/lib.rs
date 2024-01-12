//! Server of chat application, able to receive and send messages from/to clients
//!
//! # Usage
//!
//! ```
//! cargo run --bin server <hostname> <port>
//! ```
//!
//!
use library::db_client::{auth_client, save_message, setup_database_pool};
use library::{get_addr, read_from_stream, write_to_stream, MessageType};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::sync::{broadcast, Mutex as TokioMutex};
use uuid::Uuid;

use library::metrics::{dec_client_count, inc_client_count, inc_msg_count};

#[tokio::main]
pub async fn server_main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let _ = simple_logger::SimpleLogger::new().env().init();
    let _ = dotenvy::dotenv();

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
            let (mut reader, writer) = socket.into_split();
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
            let writer_mutex = Arc::new(TokioMutex::new(writer));
            loop {
                let clients = Arc::clone(&clients);
                let tx = tx.clone();
                let db_pool = db_pool.clone();
                let writer_mutex = writer_mutex.clone();
                tokio::select! {
                    result = read_from_stream(&mut reader) => tokio::spawn(async move {
                        match &result {
                            Ok(msg) => {
                                inc_msg_count();
                                match &msg {
                                    MessageType::Error(e) => {
                                        return log::error!("Error #0: {}", e)
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
                                                        //break;
                                                        unimplemented!()
                                                    }
                                                    clients.lock().unwrap().insert(socket_addr, Uuid::try_parse(&uid.to_string()).unwrap());
                                                    inc_client_count();

                                                },
                                                Err(e) => {
                                                    return log::error!("Error #1: {}", e)
                                                }
                                            };
                                        }
                                    }
                                    _ => {
                                        let msg = result.unwrap();
                                        let addr = socket_addr.to_string().clone();
                                        let broadcast_msg = (addr, msg.clone());

                                        if tx.send(broadcast_msg).is_err() {
                                            //break;
                                            unimplemented!()
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
                                dec_client_count();
                                return log::error!("Error #2: {}", e)
                            }
                        };
                    }),
                    result = rx.recv() => tokio::spawn(async move {
                        let received = match result {
                            Ok(received) => received,
                            Err(e) => {
                                eprintln!("Failed to receive broadcast message: {}", e);
                                //continue;
                                unimplemented!()
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
                            let mut writer = writer_mutex.lock().await;
                            let result = write_to_stream(&mut writer, &msg).await;
                            drop(writer);
                            match result {
                                Ok(_) => (),
                                Err(e) => {
                                    log::error!("Disconnecting client: {}", e);
                                },
                            }
                        }
                    }),
                };
            }
        });
        //clients = clients.clone();
    }
}
