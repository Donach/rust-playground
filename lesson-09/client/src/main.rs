use library::{serialize_message, MessageType};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::{env, io::Write, net::TcpStream, str::FromStr};

use crate::main_multi::start_multithreaded;
mod csv_wrapper;
mod main_multi;
mod input_handler;
fn main() {
    let args: Vec<String> = env::args().collect();
    // Evaluate args
    println!("{:?}", args);
    // Validate args for hostname and port
    let hostname: Result<Ipv4Addr, std::net::AddrParseError>;
    let port: Result<u16, std::num::ParseIntError>;
    if args.len() < 3 {
        println!("Usage: client <hostname> <port>; using default values now...");
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
    let message = MessageType::Text("Hello, server2!".to_string());

    //send_message(&addr.to_string(), &message);
    start_multithreaded(addr);
}


fn send_message(address: &str, message: &MessageType) {
    let ser_message = serialize_message(&message);
    let mut stream = TcpStream::connect(address).unwrap();
    println!("Sending message: {}", ser_message);
    // Send the length of the serialized message (as 4-byte value).
    let len = ser_message.len() as u32;
    stream.write(&len.to_be_bytes()).unwrap();

    // Send the serialized message.
    stream.write_all(ser_message.as_bytes()).unwrap();
}
