#![deny(missing_docs)]
//! Client for connecting to server
//! 
//! # Usage
//! 
//! ```
//! cargo run --bin client
//! ```
//! 
//! An async client chat application which can connect to server and send messages to other connected clients.
//! Currently has few limitations:
//! - Can only send single line of text
//! - Can only send single message at a time
//! - Cannot load "missed" messages from server
use std::error::Error;

use std::net::SocketAddrV4;

use flume::Sender;
use library::{
    await_input, handle_stream_message, read_from_stream, write_to_stream, ConnectionError,
    DataProcessingError, MessageType,
};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::time::{self, Duration};
use uuid::Uuid;

use crate::input_handler::handle_vec_input;

/// Currently can process only single line of text, known limitation
fn process_input(tx: Sender<Vec<String>>) -> Result<(), Box<dyn Error>> {
    loop {
        println!("Enter operation to perform: ");
        let input = await_input()?;

        let (left, right) = match input.splitn(2, ' ').collect::<Vec<&str>>().as_slice() {
            [left, right] => (*left, *right),
            [left] => (left.trim(), ""),
            _ => ("", ""),
        };
        let input_parsed: Vec<String> = match right {
            "" => vec![left.to_string()],
            _ => vec![left.to_string(), right.to_string()],
        };
        match input_parsed[0] == ".quit" || input_parsed[0] == ".q" {
            true => break Ok(log::info!("Quit")),
            false => {
                if !input_parsed[0].is_empty() {
                    match tx.send(input_parsed) {
                        Ok(_) => {}
                        Err(e) => break Ok(log::error!("Error: {}", e)),
                    }
                }
            }
        }
    }
}

async fn process_message(
    rx: flume::Receiver<Vec<String>>,
    stream: OwnedWriteHalf,
) -> Result<(), Box<dyn Error>> {
    let mut stream = stream;
    loop {
        match rx.recv() {
            Err(e) => {
                log::error!("Unhandled Error - server likely disconnected: {:?}", e);
                return Err(Box::new(e));
            }
            Ok(message) => {
                let result = match handle_vec_input(message) {
                    Err(e) => {
                        log::info!("Wrong input: {}", e.to_string());
                        //let _ = process_message(rx, stream);
                        return Err(e);
                    }
                    Ok(result) => result,
                };

                // If input is parsed correctly, let's connect to server and send some data there
                log::info!("Sending data to server...");
                let result = write_to_stream(&mut stream, &result).await;
                match result {
                    Ok(_s) => {
                        log::info!("Transfer complete!");
                    }
                    Err(e) => {
                        log::error!("Cannot send data to server: {}", e);
                        return Err(Box::new(e));
                    }
                }
            }
        }
    }
}

async fn receive_message(stream: &mut OwnedReadHalf) -> Result<MessageType, Box<dyn Error>> {
    //let stream = stream;
    loop {
        let res = read_from_stream(stream).await;
        match res {
            Ok(msg) => match &msg {
                MessageType::Error(e) => {
                    log::error!("Server disconnected: {}", e);
                    return Err(Box::new(ConnectionError::ClientDisconnected(e.to_string())));
                }
                MessageType::Auth(uid) => {
                    log::info!("Server Authenticated Client Success: {}", uid);
                    return Ok(msg);
                }
                _ => {
                    handle_stream_message(msg).await;
                }
            },
            Err(e) => {
                log::error!("Server disconnected: {}", e);
                return Err(Box::new(e));
            }
        }
    }
}
async fn handle_auth(
    uid: Uuid,
    writer: &mut OwnedWriteHalf,
    reader: &mut OwnedReadHalf,
) -> Result<bool, Box<dyn Error>> {
    log::info!("Starting authentication... {}", uid.to_string());
    match handle_vec_input(vec![".auth".to_string(), uid.to_string()]) {
        Err(e) => {
            log::error!("Authentication Error: {}", e.to_string());
        }
        Ok(auth_msg) => {
            match write_to_stream(writer, &auth_msg).await {
                Ok(_s) => {
                    log::info!("Authentication sent!");
                    // Wait for server reply
                    match receive_message(reader).await {
                        Ok(return_msg) => match auth_msg == return_msg {
                            true => {
                                log::info!("Authentication successful!");
                            }
                            false => {
                                log::error!("Authentication failed!");
                                return Err(Box::new(ConnectionError::ServerNotFound(
                                    "Authentication failed!".to_string(),
                                )));
                            }
                        },
                        Err(e) => {
                            log::error!("Authentication Error: Invalid login {}", e);
                            return Err(e);
                        }
                    }
                }
                Err(e) => {
                    log::error!("Authentication Error: Server Disconnected {}", e);
                    return Err(Box::new(e));
                }
            }
        }
    };
    Ok(true)
}

/// Start multi-threaded client application
/// 
/// # Arguments
/// (SocketAddrV4, Uuid) - The address of the server and the uid of the client, generated on the client side automatically,
/// if the user does not specify any. The uid is used for authentication and should be reused.
/// 
pub async fn start_multithreaded(conninfo: (SocketAddrV4, Uuid)) -> Result<(), Box<dyn Error>> {
    let (address, uid) = conninfo;
    log::info!("Starting interactive mode @{}", address);
    // Define the retry interval and total retry duration
    let retry_interval = Duration::from_secs(10);
    let total_retry_duration = Duration::from_secs(10 * 60); // 10 minutes
    let start_time = time::Instant::now();

    loop {
        match TcpStream::connect(address).await {
            Err(e) => {
                log::error!("Failed to connect: {}", e);

                // Check if total retry duration is exceeded
                if time::Instant::now().duration_since(start_time) > total_retry_duration {
                    log::error!("Failed to connect to server after 10 minutes, exiting.");
                    return Err(Box::new(e));
                }

                // Wait for the retry interval
                time::sleep(retry_interval).await;
            }
            Ok(stream) => {
                let (mut reader, mut writer) = stream.into_split();

                // Authentication
                if handle_auth(uid, &mut writer, &mut reader).await.is_ok() {
                    let write_task = tokio::spawn(async move {
                        log::info!("Starting write task...");
                        // Thread for reading from stdin
                        let (tx, rx) = flume::unbounded();
                        let _t_input = tokio::spawn(async move {
                            log::info!("Starting process_input task...");
                            match process_input(tx) {
                                Ok(_) => Ok(()),
                                Err(e) => {
                                    log::error!("Input Error: {}", e);
                                    Err(DataProcessingError::InvalidFormat)
                                }
                            }
                        });
                        // Thread that processes stdin and submits data to server
                        let _t_process_input = tokio::spawn(async move {
                            log::info!("Starting process_message task...");
                            match process_message(rx, writer).await {
                                Ok(_) => Ok(()),
                                Err(e) => {
                                    log::error!("Processing error: {}", e);
                                    Err(DataProcessingError::InvalidFormat)
                                }
                            }
                        });

                        //let _ = tokio::try_join!(t_input, t_process_input);
                    });
                    let read_task = tokio::spawn(async move {
                        log::info!("Starting reader task...");
                        // Thread that reads data from server
                        match receive_message(&mut reader).await {
                            Ok(msg) => {
                                log::info!("Message received: {:?}", msg);
                                Ok(msg)
                            }
                            Err(e) => Err(ConnectionError::ServerNotFound(format!(
                                "Server disconnected {}",
                                e
                            ))),
                        }
                    });
                    let _ = tokio::try_join!(read_task, write_task);
                    log::info!("Last Line");
                }
            }
        }
    }
}
