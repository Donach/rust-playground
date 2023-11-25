use library::{deserialize_message, get_addr, MessageType, DataProcessingError};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, prelude::*, Cursor};
use std::net::{SocketAddr, SocketAddrV4, TcpListener, TcpStream};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, fs};

use image::codecs::png::PngEncoder;
use image::io::Reader as ImageReader;
use log;
use simple_logger;

fn get_timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string()
}

fn prepare_path(message: &MessageType, file_name: &str, current_timestamp: &str) -> PathBuf {
    let mut path: std::path::PathBuf = env::current_dir().unwrap();
    path.push("files");
    let _create_dir_all = fs::create_dir_all(&path);
    if let MessageType::Image(_i) = message {
        path.push(String::from(current_timestamp) + ".png");
    } else {
        path.push(file_name);
    }
    path
}

fn write_file(message: &MessageType, file: &[u8], file_name: &str) -> Result<(), library::DataProcessingError> {
    let path = prepare_path(message, file_name, "");
    let tgt_file = File::create(&path);
    match tgt_file {
        Ok(mut tgt_file) => {
            tgt_file.write_all(file).unwrap();
            log::info!(
                "Received file {} written to: {:?}",
                String::from(file_name),
                path
            );
            Ok(())
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

fn write_image(message: &MessageType, file: &[u8]){
    let current_timestamp = get_timestamp();
    let path = prepare_path(message, "", &current_timestamp);

    let mut bytes: Vec<u8> = Vec::new();
    //let img = BufReader::new(file);
    let data = Cursor::new(file);
    let img = ImageReader::new(data)
        .with_guessed_format()
        .expect("This will never fail using Cursor");
    let img = img.decode().unwrap();
    match img.write_with_encoder(PngEncoder::new(&mut bytes)) {
        Ok(_res) => {
            if let Ok(mut tgt_file) = File::create(&path) {
                tgt_file.write_all(&bytes).unwrap();
                log::info!(
                    "Received image {} written to: {:?}",
                    current_timestamp + ".png",
                    path
                );
            } else {
                log::error!(
                    "Failed to open target path: {}",
                    path.as_os_str().to_os_string().to_str().unwrap()
                );
            };
        }
        Err(err) => {
            log::error!("Error: Cannot encode image to PNG {:?}", err);
        }
    }
}

fn handle_client(
    mut stream: TcpStream,
    clients: &mut HashMap<SocketAddr, TcpStream>,
) -> MessageType {
    let addr = stream.peer_addr().unwrap();
    clients.insert(addr, stream.try_clone().unwrap());

    let mut len_bytes = [0u8; 4];
    match stream.read_exact(&mut len_bytes) {
        Ok(it) => it,
        Err(_err) => return MessageType::Empty,
    };
    let len = u32::from_be_bytes(len_bytes) as usize;
    //println!("Len: {}", len);

    if len > 0 {
        log::info!("Receiving data...");
        let mut buffer = vec![0u8; len];
        stream.read_exact(&mut buffer).unwrap();
        //println!("Buffer: {:?}", buffer);

        let message = deserialize_message(&buffer);
        let msg_type = message.unwrap();
        match &msg_type {
            MessageType::File(name, file) => {
                // Write file into files/ dir
                write_file(&msg_type, &file, &name);
                msg_type
            }
            MessageType::Image(file) => {
                // Write image into files/ dir
                write_image(&msg_type, &file);
                msg_type
            }
            MessageType::Text(_t) => {
                log::info!("Received message: {:?}", &msg_type);
                msg_type
            }
            MessageType::Empty => MessageType::Empty,
        }
    } else {
        MessageType::Empty
    }
}

fn listen_and_accept(address: SocketAddrV4) {
    let listener = TcpListener::bind(address).unwrap();
    listener
        .set_nonblocking(true)
        .expect("failed to initiate non-blocking");

    let mut clients: HashMap<SocketAddr, TcpStream> = HashMap::new();

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                let _message = handle_client(s, &mut clients);
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                // Wait until socket is ready - not sure if wait should be here?
                //std::thread::sleep(std::time::Duration::from_secs(1));

                continue;
            }
            Err(ee) => panic!("IO error! {}", ee),
        }
    }
}

fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    // This runs lib function to parse hostname and port, or use default
    listen_and_accept(get_addr(env::args().collect()).unwrap());
}
