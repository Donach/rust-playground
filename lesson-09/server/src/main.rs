use library::MessageType;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io;

fn main() {
    let mut listener = TcpListener::bind("127.0.0.1:11111").unwrap();
    // file:///home/donach/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/std/net/struct.TcpStream.html#method.set_nonblocking
    for connection in listener.incoming() {
        let mut connection = connection.unwrap(); // TODO: Handle errors
        let mut len_bytes = [0u8; 4];
        connection.read_exact(&mut len_bytes).unwrap();

        let len = u32::from_be_bytes(len_bytes) as usize;
        let mut buffer = vec![0u8; len];
        connection.read_exact(&mut buffer).unwrap();

        let my_message = MessageType::deserialize(&buffer).unwrap();

        println!("Received: {:?}", my_message);
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Received: {:?}", String::from_utf8_lossy(&buffer[..]));
}

fn handle_connection(mut stream: TcpStream, address: &str, clients: &mut HashMap<SocketAddr, TcpStream>) {
    let addr = stream.peer_addr().unwrap();
    clients.insert(addr.clone(), stream);

    let message = handle_client(clients.get(&addr).unwrap().try_clone().unwrap());
    // Here you can further process this message as per your requirements
    println!("{:?}", message);
}
fn listen_and_accept(address: &str) {
    let listener = TcpListener::bind(address).unwrap();
    let mut clients: HashMap<SocketAddr, TcpStream> = HashMap::new();

    // Tracking connected clients
    for stream in listener.incoming() {
        let stream = stream;
        stream.unwrap().set_nonblocking(true).expect("failed to initiate non-blocking");
        match stream {
            Ok(s) => {
                handle_connection(s, &address, &mut clients);
                
            }
            Err (e) if e.kind() == io::ErrorKind::WouldBlock => {
                // Wait until socket is ready
                //wait_for_fd();
                continue;
            }
            Err (ee) => panic!("IO error! {}", ee),
        }
    }
}
