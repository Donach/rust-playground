use std::env;
use std::net::{Ipv4Addr, SocketAddrV4};

use crate::main_multi::start_multithreaded;
mod input_handler;
mod main_multi;
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

    let _ = start_multithreaded(addr);
}
