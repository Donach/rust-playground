use library::{get_addr, read_from_stream, write_to_stream};
use std::collections::HashMap;
use std::error::Error;
//use std::collections::HashMap;
use std::io::{self};
use std::net::{SocketAddr, /*SocketAddr, */ SocketAddrV4};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use simple_logger;
use log;
use std::sync::{Arc, Mutex};
use std::{env, thread};

//static mut clients: HashMap<SocketAddr, TcpStream> = HashMap::new();

async fn handle_client(sender_stream: TcpStream, clients: Arc<Mutex<HashMap<SocketAddr, TcpStream>>>) {
    let (reader, writer) = tokio::io::split(sender_stream);
    let (s, msg_type) = read_from_stream(reader).await;

    let mut clients = clients.lock().unwrap();
    // Remove disconnected clients and Send reponse to all clients
    clients.retain(|&addr, stream| {
        if addr != sender_stream.peer_addr().unwrap() {
            let result = write_to_stream(stream, &msg_type);
            match result {
                Ok(_) => true,
                Err(_e) => false,
            }
        } else {
            true
        }
    });
}

async fn listen_and_accept(address: SocketAddrV4) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(address).await?;
    /* listener
        .set_nonblocking(true)
        .expect("failed to initiate non-blocking"); */

    //let (tx, rx) = mpsc::channel();
    //let rx = Arc::new(Mutex::new(rx));
    let clients: Arc<Mutex<HashMap<SocketAddr, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let addr: SocketAddr = stream.peer_addr().unwrap();
                clients
                    .lock()
                    .unwrap()
                    .insert(addr, stream.try_clone().unwrap());

                //let tx = tx.clone();
                //let rx = Arc::clone(&rx);
                let clients = clients.clone();
                let _t = tokio::spawn(async {
                    handle_client(stream, clients);
                });
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                // Wait until socket is ready - not sure if wait should be here?
                //std::thread::sleep(std::time::Duration::from_secs(1));

                continue;
            }
            Err(ee) => panic!("IO error! {}", ee),
        };
    };
    Ok(())
}

fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    // This runs lib function to parse hostname and port, or use default
    listen_and_accept(get_addr(env::args().collect()).unwrap());
}
