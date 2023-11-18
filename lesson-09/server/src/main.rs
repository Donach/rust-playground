use std::collections::HashMap;
use std::{env, fs};
use std::fs::File;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{prelude::*, self, empty};
use std::error::Error;

use library::{deserialize_message, MessageType};


fn handle_client(mut stream: TcpStream, clients: &mut HashMap<SocketAddr, TcpStream>) -> MessageType {    
    let addr = stream.peer_addr().unwrap();
    clients.insert(addr.clone(), stream.try_clone().unwrap());

    let mut len_bytes = [0u8; 4];
    match stream.read_exact(&mut len_bytes) {
        Ok(it) => it,
        Err(err) => return MessageType::Empty,
    };
    let len = u32::from_be_bytes(len_bytes) as usize;
    println!("Len: {}", len);

    if len > 0 {
        let mut buffer = vec![0u8; len];
        stream.read_exact(&mut buffer).unwrap();
        println!("Buffer: {:?}", buffer);

        let message: MessageType = deserialize_message(&buffer);
        match &message {
            MessageType::File(name, file) => {
                // Write file into files/ dir
                let mut path = env::current_dir().unwrap();
                path.push("files");
                fs::create_dir_all(&path);
                path.push(&name);
                println!("File path: {:?}", path);
                let mut tgt_file = match File::create(path) {
                    Ok(file) => file,
                    Err(error) => {
                        println!("Failed to open file: {}", error);
                        Err(error).unwrap()
                    }
                };
                tgt_file.write_all(&file).unwrap();
                message
            }
            MessageType::Image( file) => {
                todo!("Implement .image")
            }
            _ => {
                message
            }
        }
    } else {
        MessageType::Empty
    }
}

fn listen_and_accept(address: &str) {
    let listener = TcpListener::bind(address).unwrap();
    listener.set_nonblocking(true).expect("failed to initiate non-blocking");
    

    let mut clients: HashMap<SocketAddr, TcpStream> = HashMap::new();

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                let message = handle_client(s, &mut clients);
                println!("{:?}", message);
            }
            Err (e) if e.kind() == io::ErrorKind::WouldBlock => {
                // Wait until socket is ready
                // println!("Waiting for socket to be ready...");
                std::thread::sleep(std::time::Duration::from_secs(1));
                
                continue;
            }
            Err (ee) => panic!("IO error! {}", ee),
        }

        // let message = handle_client(stream.unwrap(), &mut clients);
        // Here, you can further process this message as per your requirements
        // println!("{:?}", message);
    }
}

fn main() {
    let hostname = "127.0.0.1";
    let port = 11111;
    let addr = format!("{}:{}", hostname, port);
    
    listen_and_accept(&addr);
}
