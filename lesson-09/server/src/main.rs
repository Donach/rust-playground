use library::{deserialize_message, MessageType};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, Cursor};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, fs};

use image::codecs::png::PngEncoder;
use image::io::Reader as ImageReader;

fn write_file(message: &MessageType, file: &[u8], file_name: &str) {
    let current_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string();
    let mut path = env::current_dir().unwrap();
    path.push("files");
    let _create_dir_all = fs::create_dir_all(&path);
    if let MessageType::Image(_i) = message {
        path.push(String::from(&current_timestamp) + ".png");
    } else {
        path.push(file_name);
    }

    if let Ok(mut tgt_file) = File::create(&path) {
        tgt_file.write_all(file).unwrap();
        println!(
            "Received file {} written to: {:?}",
            String::from(file_name) + ".png",
            path
        );
    } else {
        println!(
            "Failed to open target path: {}",
            path.as_os_str().to_os_string().to_str().unwrap()
        );
    };
}

fn write_image(
    _message: &MessageType,
    file: &[u8],
) -> Result<MessageType, Box<dyn std::error::Error>> {
    let current_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string();
    let mut path = env::current_dir().unwrap();
    path.push("files");
    let _create_dir_all = fs::create_dir_all(&path);
    path.push(String::from(&current_timestamp) + ".png");

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
                println!(
                    "Received image {} written to: {:?}",
                    current_timestamp + ".png",
                    path
                );
            } else {
                println!(
                    "Failed to open target path: {}",
                    path.as_os_str().to_os_string().to_str().unwrap()
                );
            };
            Ok(MessageType::Image(bytes))
        }
        Err(err) => {
            eprintln!("Error: Cannot encode image to PNG {:?}", err);
            Ok(MessageType::Empty)
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
    println!("Len: {}", len);

    if len > 0 {
        println!("Receiving data...");
        let mut buffer = vec![0u8; len];
        stream.read_exact(&mut buffer).unwrap();
        //println!("Buffer: {:?}", buffer);

        let message: MessageType = deserialize_message(&buffer);
        match &message {
            MessageType::File(name, file) => {
                // Write file into files/ dir
                write_file(&message, file, name);
                message
            }
            MessageType::Image(file) => {
                // Write file into files/ dir
                let _write_image = write_image(&message, file);
                message
            }
            MessageType::Text(_t) => {
                println!("Received message: {:?}", message);
                message
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
                // Wait until socket is ready
                //std::thread::sleep(std::time::Duration::from_secs(1));

                continue;
            }
            Err(ee) => panic!("IO error! {}", ee),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // Evaluate args
    println!("{:?}", args);
    // Validate args for hostname and port
    let hostname: Result<Ipv4Addr, std::net::AddrParseError>;
    let port: Result<u16, std::num::ParseIntError>;
    if args.len() < 3 {
        println!("Usage: server <hostname> <port>; using default values now...");
        hostname = "127.0.0.1".parse::<Ipv4Addr>();
        port = "11111".parse::<u16>();
    } else {
        hostname = args[1].parse::<Ipv4Addr>(); //Ipv4Addr::from_str(&args[1]).unwrap();
        match hostname {
            Ok(h) => {
                println!("Parsed hostname: {:?}", h);
            }
            Err(e) => {
                eprintln!("Error parsing hostname: {:?}", e);
                panic!()
            }
        }
        port = args[2].parse::<u16>();
        match port {
            Ok(p) => {
                println!("Parsed port: {:?}", &p);
            }
            Err(e) => {
                eprintln!("Error parsing port: {:?}", e);
                panic!()
            }
        }
    }

    let addr = SocketAddrV4::new(hostname.unwrap(), port.to_owned().unwrap());

    listen_and_accept(addr);
}
