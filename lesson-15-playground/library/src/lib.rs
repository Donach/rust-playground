use std::{
    env,
    fs::{self, File},
    io::{self, Cursor, Read, Write},
    net::{Ipv4Addr, SocketAddrV4},
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[cfg(not(debug_assertions))]
use ::anyhow as eyre;

#[cfg(debug_assertions)]
use color_eyre::eyre;
use image::{codecs::png::PngEncoder, io::Reader as ImageReader, ImageError};
use log;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use eyre::Result;

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

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Text(String),
    Image(Vec<u8>),
    File(String, Vec<u8>), // Filename and its content as bytes
    Error(String),
}

pub fn serialize_message(message: &MessageType) -> Result<String, crate::DataProcessingError> {
    Ok(serde_json::to_string(message).map_err(|e| e)?)
}

pub fn deserialize_message(data: &[u8]) -> Result<MessageType, crate::DataProcessingError> {
    Ok(serde_json::from_slice(data).map_err(|e| e)?)
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
pub fn get_addr(args: Vec<String>) -> Result<SocketAddrV4, crate::DataProcessingError> {
    // Evaluate args
    //println!("{:?}", args);
    // Validate args for hostname and port
    let hostname: Result<Ipv4Addr, std::net::AddrParseError>;
    let port: Result<u16, std::num::ParseIntError>;
    if args.len() < 3 {
        log::warn!("Usage: server <hostname> <port>; using default values now...");
        hostname = "127.0.0.1".parse::<Ipv4Addr>();
        port = "11111".parse::<u16>();
    } else {
        hostname = args[1].parse::<Ipv4Addr>(); //Ipv4Addr::from_str(&args[1]).unwrap();
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
    }

    Ok(SocketAddrV4::new(
        hostname.unwrap(),
        port.to_owned().unwrap(),
    ))
}

pub async fn read_from_stream(mut stream: tokio::io::ReadHalf<tokio::net::TcpStream>) -> (tokio::io::ReadHalf<tokio::net::TcpStream>, MessageType) {
    // Read first 4 bytes containing length of the rest of the message
    let mut len_bytes = [0u8; 4];
    match stream.read_exact(&mut len_bytes).await {
        Ok(it) => it,
        Err(err) => return (stream, MessageType::Error(format!("Error: {:?}", err))),
    };

    let len = u32::from_be_bytes(len_bytes) as usize;
    if len > 0 {
        log::info!("Receiving data...");
        let mut buffer = vec![0u8; len];
        match stream.read_exact(&mut buffer).await {
            Ok(it) => it,
            Err(err) => return (stream, MessageType::Error(format!("Error: {:?}", err))),
        };

        let message = match deserialize_message(&buffer) {
            Ok(it) => it,
            Err(err) => MessageType::Error(format!("Error: {:?}", err)),
        };

        (stream, message)
    } else {
        (
            stream,
            MessageType::Error(format!("Error: received no data !")),
        )
    }
}

pub async fn write_to_stream(
    mut stream: tokio::io::WriteHalf<tokio::net::TcpStream>,
    message: &MessageType,
) -> Result<tokio::io::WriteHalf<tokio::net::TcpStream>, DataProcessingError> {
    let ser_message = serialize_message(message)
        .map_err(|e| log::error!("Error: {:?}", e))
        .unwrap();
    // Send the length of the serialized message (as 4-byte value).
    let len = ser_message.len() as u32;
    stream.write_all(&len.to_be_bytes()).await.expect("Failed to write to stream");

    // Send the serialized message.
    let s = stream.write_all(ser_message.as_bytes()).await;
    match s {
        Ok(it) => it,
        Err(err) => return Err(DataProcessingError::Io(err)),
    };

    log::info!("Transfer complete!");
    Ok(stream)
}

pub fn handle_stream_message(message: MessageType) -> MessageType {
    match &message {
        MessageType::File(name, file) => {
            // Write file into files/ dir
            let result = write_file(&message, &file, &name);
            match result {
                Err(e) => {
                    log::error!("Error: {:?}", e);
                    MessageType::Text(format!("Error: {:?}", e))
                }
                Ok(msg) => MessageType::Text(format!("{:?}", msg)),
            }
        }
        MessageType::Image(file) => {
            // Write image into files/ dir
            let result = write_image(&message, &file);
            // If result is error, send message back to client
            match result {
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

fn prepare_path(
    message: &MessageType,
    file_name: &str,
    current_timestamp: &str,
) -> Result<PathBuf, DataProcessingError> {
    let path = env::current_dir();
    match path {
        Ok(mut path) => {
            path.push("files");
            let _create_dir_all = fs::create_dir_all(&path);
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

fn write_file(
    message: &MessageType,
    file: &[u8],
    file_name: &str,
) -> Result<String, DataProcessingError> {
    let path = prepare_path(message, file_name, "")?;
    let tgt_file = File::create(&path);
    match tgt_file {
        Ok(mut tgt_file) => {
            tgt_file.write_all(file).unwrap();
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

fn write_image(message: &MessageType, file: &[u8]) -> Result<String, DataProcessingError> {
    let current_timestamp = get_timestamp();
    let path = prepare_path(message, "", &current_timestamp).unwrap();

    let mut bytes: Vec<u8> = Vec::new();
    //let img = BufReader::new(file);
    let data = Cursor::new(file);
    let img = ImageReader::new(data)
        .with_guessed_format()
        .expect("This will never fail using Cursor");
    let img = img.decode().unwrap();
    match img.write_with_encoder(PngEncoder::new(&mut bytes)) {
        Ok(_res) => {
            let tgt_file = File::create(&path);
            match tgt_file {
                Ok(mut tgt_file) => {
                    tgt_file.write_all(&bytes).unwrap();
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
