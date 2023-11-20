use std::{
    error::Error,
    io,
    net::{Ipv4Addr, SocketAddrV4},
};

use log;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Text(String),
    Image(Vec<u8>),
    File(String, Vec<u8>), // Filename and its content as bytes
    Empty,
}

pub fn serialize_message(message: &MessageType) -> String {
    serde_json::to_string(message).unwrap()
}

pub fn deserialize_message(data: &[u8]) -> MessageType {
    serde_json::from_slice(data).expect("Failed to deserialize message")
}

pub fn await_input() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_res) => {
            if input == "q" || input.is_empty() {
                Err(".quit".into())
            } else {
                return Ok(input.trim().to_string());
            }
        }
        Err(err) => Err(err.into()),
    }
}

// Returns a SocketAddr, using default if cannot parse and/or no args given
pub fn get_addr(args: Vec<String>) -> Result<SocketAddrV4, Box<dyn Error>> {
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
