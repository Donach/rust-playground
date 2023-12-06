//! Library functions for chat application
//! 
//! This library provides functions for chat application. Every functionality that is somehow duplicated 
//! between the client and the server is implemented here.
//! 
//! 
use anyhow::Context;
use std::{
    env,
    io::{self, Cursor},
    net::{Ipv4Addr, SocketAddrV4},
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{
    fs::{self, File},
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
};

#[cfg(not(debug_assertions))]
use ::anyhow as eyre;

#[cfg(debug_assertions)]
use color_eyre::eyre;
use image::{codecs::png::PngEncoder, io::Reader as ImageReader, ImageError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use eyre::Result;


/// 
/// 

#[derive(Error, Debug)]
pub enum DataProcessingError {
    #[error("Data not found: {0}")]
    NotFound(String),
    #[error("Invalid data format")]
    InvalidFormat,
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("De/Serialize error - wrong data format or corrupted data")]
    Serde(#[from] serde_json::Error),
    #[error("Cannot process image - invalid image format")]
    ImageError(#[from] ImageError),
    #[error("Exitting")]
    Exit,
}

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("Server not found: {0}")]
    ServerNotFound(String),
    #[error("Server not found: {0}")]
    ClientDisconnected(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MessageType {
    Text(String),
    Image(Vec<u8>),
    File(String, Vec<u8>), // Filename and its content as bytes
    Error(String),
    Auth(String),
}

pub fn serialize_message(message: &MessageType) -> Result<String, crate::DataProcessingError> {
    Ok(serde_json::to_string(message)?)
}

pub fn deserialize_message(data: &[u8]) -> Result<MessageType, crate::DataProcessingError> {
    Ok(serde_json::from_slice(data)?)
}

pub fn serialize_message_as_bin(
    message: &MessageType,
) -> Result<Vec<u8>, crate::DataProcessingError> {
    Ok(bincode::serialize(message).unwrap())
}

pub fn deserialize_message_as_bin(data: &[u8]) -> Result<MessageType, crate::DataProcessingError> {
    Ok(bincode::deserialize(data).unwrap())
}

pub fn await_input() -> Result<String, crate::DataProcessingError> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_res) => {
            if input == ".quit" || input.is_empty() || input == ".q" {
                Err(DataProcessingError::Exit)
            } else {
                return Ok(input.trim().to_string());
            }
        }
        Err(err) => Err(err.into()),
    }
}

// Returns a SocketAddr, using default if cannot parse and/or no args given
pub fn get_addr(args: Vec<String>) -> Result<(SocketAddrV4, Uuid), crate::DataProcessingError> {
    // Evaluate args
    //println!("{:?}", args);
    // Validate args for hostname and port
    let hostname: Result<Ipv4Addr, std::net::AddrParseError>;
    let port: Result<u16, std::num::ParseIntError>;
    let uid: Result<Uuid, uuid::Error>;
    if args.len() < 4 {
        log::warn!("Usage: server/client <hostname> <port> <uid>; using default values now...");
        hostname = "127.0.0.1".parse::<Ipv4Addr>();
        port = "11111".parse::<u16>();
        uid = Ok(Uuid::new_v4());
    } else {
        hostname = args[1].parse::<Ipv4Addr>();
        match hostname {
            Ok(h) => {
                log::info!("Parsed hostname: {:?}", h);
            }
            Err(e) => {
                log::error!("Error parsing hostname: {:?}", e);
                panic!()
            }
        }
        port = args[2].parse::<u16>();
        match port {
            Ok(p) => {
                log::info!("Parsed port: {:?}", &p);
            }
            Err(e) => {
                log::error!("Error parsing port: {:?}", e);
                panic!()
            }
        }

        uid = args[3].parse::<Uuid>();
        match uid {
            Ok(p) => {
                log::info!("Parsed uid: {:?}", &p);
            }
            Err(e) => {
                log::error!("Error parsing uid: {:?}", e);
                panic!()
            }
        }
    }

    Ok((
        SocketAddrV4::new(hostname.unwrap(), port.to_owned().unwrap()),
        uid.unwrap(),
    ))
}

pub async fn read_from_stream(
    stream: &mut OwnedReadHalf,
) -> Result<MessageType, crate::DataProcessingError> {
    // Read first 4 bytes containing length of the rest of the message
    let mut len_bytes = [0u8; 4];
    match stream.read_exact(&mut len_bytes).await {
        Ok(it) => it,
        Err(err) => return Err(DataProcessingError::Io(err)),
    };

    let len = u32::from_be_bytes(len_bytes) as usize;
    if len > 0 {
        log::trace!("Receiving data...");
        let mut buffer = vec![0u8; len];
        match stream.read_exact(&mut buffer).await {
            Ok(it) => it,
            Err(err) => return Err(DataProcessingError::Io(err)),
        };

        let message = match deserialize_message(&buffer) {
            Ok(it) => it,
            Err(err) => MessageType::Error(format!("Error: {:?}", err)),
        };

        log::trace!("Data received!");
        Ok(message)
    } else {
        Err(DataProcessingError::NotFound("Empty stream".to_string()))
    }
}

pub async fn write_to_stream(
    stream: &mut OwnedWriteHalf,
    message: &MessageType,
) -> Result<(), DataProcessingError> {
    let ser_message = serialize_message(message)
        .map_err(|e| log::error!("Error: {:?}", e))
        .unwrap();
    // Send the length of the serialized message (as 4-byte value).
    let len = ser_message.len() as u32;
    stream
        .write_all(&len.to_be_bytes())
        .await
        .expect("Failed to write to stream");

    // Send the serialized message.
    let s = stream.write_all(ser_message.as_bytes()).await;
    match s {
        Ok(it) => it,
        Err(err) => return Err(DataProcessingError::Io(err)),
    };

    log::info!("Transfer complete!");
    Ok(())
}

pub async fn handle_stream_message(message: MessageType) -> MessageType {
    match &message {
        MessageType::Auth(uid) => {
            log::info!("Authenticating user {:?}", &uid);
            // Save UID in DB
            MessageType::Auth(format!("{:?}", uid))
        }
        MessageType::File(name, file) => {
            // Write file into files/ dir
            let result = write_file(&message, file, name);
            match result.await {
                Err(e) => {
                    log::error!("Error: {:?}", e);
                    MessageType::Text(format!("Error: {:?}", e))
                }
                Ok(msg) => MessageType::Text(format!("{:?}", msg)),
            }
        }
        MessageType::Image(file) => {
            // Write image into files/ dir
            let result = write_image(&message, file);
            // If result is error, send message back to client
            match result.await {
                Err(e) => {
                    log::error!("Error: {:?}", e);
                    MessageType::Text(format!("Error: {:?}", e))
                }
                Ok(msg) => MessageType::Text(format!("{:?}", msg)),
            }
        }
        MessageType::Text(_t) => {
            log::info!("Received message: {:?}", &message);
            message
        }
        MessageType::Error(e) => MessageType::Error(format!("Error: {}", e)),
    }
}

fn get_timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string()
}

async fn prepare_path(
    message: &MessageType,
    file_name: &str,
    current_timestamp: &str,
) -> Result<PathBuf, DataProcessingError> {
    let path = env::current_dir();
    match path {
        Ok(mut path) => {
            path.push("files");
            match fs::create_dir_all(&path).await {
                Ok(_) => {}
                Err(e) => {
                    log::error!("Failed to create target path: {}", e);
                    return Err(DataProcessingError::Io(e));
                }
            };
            if let MessageType::Image(_i) = message {
                path.push(String::from(current_timestamp) + ".png");
            } else {
                path.push(file_name);
            }
            Ok(path)
        }
        Err(e) => Err(DataProcessingError::Io(e)),
    }
}

async fn write_file(
    message: &MessageType,
    file: &[u8],
    file_name: &str,
) -> Result<String, DataProcessingError> {
    let path = prepare_path(message, file_name, "").await?;
    let tgt_file = File::create(&path)
        .await
        .context(format!("Cannot create file at {:?}", &path.to_str()));
    match tgt_file.unwrap().write_all(file).await {
        Ok(_) => {
            let msg = format!(
                "Received file {} written to: {:?}",
                String::from(file_name),
                path
            );
            log::info!("{}", msg.as_str());
            Ok(msg)
        }
        Err(e) => {
            log::error!(
                "Failed to open target path: {}",
                path.as_os_str().to_os_string().to_str().unwrap()
            );
            Err(DataProcessingError::Io(e))
        }
    }
}

async fn write_image(message: &MessageType, file: &[u8]) -> Result<String, DataProcessingError> {
    let current_timestamp = get_timestamp();
    let path = prepare_path(message, "", &current_timestamp).await?;

    let mut bytes: Vec<u8> = Vec::new();
    //let img = BufReader::new(file);
    let data = Cursor::new(file);
    let img = ImageReader::new(data)
        .with_guessed_format()
        .expect("This will never fail using Cursor");
    let img = img.decode().unwrap();
    match img.write_with_encoder(PngEncoder::new(&mut bytes)) {
        Ok(_res) => {
            let tgt_file = File::create(&path)
                .await
                .context(format!("Cannot create file at {:?}", &path.to_str()));
            match tgt_file.unwrap().write_all(&bytes).await {
                Ok(_) => {
                    let msg = format!(
                        "Received image {} written to: {:?}",
                        current_timestamp + ".png",
                        path
                    );
                    log::info!("{}", msg.as_str());
                    Ok(msg)
                }
                Err(e) => {
                    log::error!(
                        "Failed to open target path: {}",
                        path.as_os_str().to_os_string().to_str().unwrap()
                    );
                    Err(DataProcessingError::Io(e))
                }
            }
        }
        Err(e) => {
            log::error!("Error: Cannot encode image to PNG {:?}", e);
            Err(DataProcessingError::ImageError(e))
        }
    }
}
