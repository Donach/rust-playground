use library::{get_addr, read_from_stream, write_to_stream};
use std::collections::HashMap;
//use std::collections::HashMap;
use std::io::{self};
use std::net::{SocketAddr, /*SocketAddr, */ SocketAddrV4, TcpListener, TcpStream};

use simple_logger;
use std::sync::{Arc, Mutex};
use std::{env, thread};

//static mut clients: HashMap<SocketAddr, TcpStream> = HashMap::new();

fn handle_client(sender_stream: TcpStream, clients: Arc<Mutex<HashMap<SocketAddr, TcpStream>>>) {
    let sender_stream = match sender_stream.try_clone() {
        Ok(s) => s,
        Err(_) => {
            log::error!("Failed to clone stream");
            return;
            //break;
        }
    };
    let (sender_stream, msg_type) = read_from_stream(sender_stream);

    let mut clients = clients.lock().unwrap();
    // Remove disconnected clients and Send reponse to all clients
    clients.retain(|&addr, stream| {
        if addr != sender_stream.peer_addr().unwrap() {
            let stream = stream.try_clone().unwrap();
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

fn listen_and_accept(address: SocketAddrV4) {
    let listener = TcpListener::bind(address).unwrap();
    listener
        .set_nonblocking(true)
        .expect("failed to initiate non-blocking");

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
                let _t = thread::spawn(|| {
                    handle_client(stream, clients);
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
