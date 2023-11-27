use library::{get_addr, handle_stream_message, read_from_stream, write_to_stream};
//use std::collections::HashMap;
use std::io::{self};
use std::net::{/*SocketAddr, */ SocketAddrV4, TcpListener, TcpStream};

use std::{env, thread};

use simple_logger;

//static mut clients: HashMap<SocketAddr, TcpStream> = HashMap::new();

fn handle_client(stream: TcpStream) {
    //let addr = stream.peer_addr().unwrap();
    //clients.insert(addr, stream.try_clone().unwrap());

    let (stream, msg_type) = read_from_stream(stream);
    let response = handle_stream_message(msg_type);
    // Send reponse back to client
    let _ = write_to_stream(stream, &response);
}

fn listen_and_accept(address: SocketAddrV4) {
    let listener = TcpListener::bind(address).unwrap();
    listener
        .set_nonblocking(true)
        .expect("failed to initiate non-blocking");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                let t = thread::spawn(|| {
                    handle_client(s);
                });
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
