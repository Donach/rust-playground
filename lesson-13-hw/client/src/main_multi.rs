use std::error::Error;
use std::io::Write;
use std::net::{SocketAddrV4, TcpStream};
use std::thread::{self, JoinHandle};

use flume::Sender;
use library::{await_input, serialize_message, MessageType, DataProcessingError};
use log;

use crate::input_handler::handle_vec_input;

fn process_input(tx: Sender<Vec<String>>) -> Result<Vec<String>, Box<dyn Error>> {
    println!("Enter operation to perform: ");
    let input = await_input()?;

    let (left, right) = match input.splitn(2, ' ').collect::<Vec<&str>>().as_slice() {
        [left, right] => (*left, *right),
        [left] => (left.trim(), ""),
        _ => ("", ""),
    };
    let input_parsed: Vec<String> = match right {
        "" => vec![left.to_string()],
        _ => vec![left.to_string(), right.to_string()],
    };
    //println!("{:?}", &input_parsed);
    match input_parsed[0].is_empty() || input_parsed[0] == ".quit" {
        false => {
            tx.send(input_parsed).unwrap();
            process_input(tx) // Recursive call
        }
        true => {
            tx.send(input_parsed).unwrap();
            Ok(vec!["Done".to_string()])
        }
    }
}

fn start_input_thread(tx: flume::Sender<Vec<String>>) -> JoinHandle<()> {
    thread::spawn(move || {
        //log::info!("Starting process input thread.");
        let _ = process_input(tx);
    })
}

fn process_message(rx: flume::Receiver<Vec<String>>, address: &str) {
    let message = rx.recv();
    match &message {
        Ok(_res) => {
            let message = message.unwrap();
            let result = handle_vec_input(message);
            match &result {
                Err(_e) => {
                    log::info!("Exitting...");
                }
                _ => {
                    log::info!("Sending data to server...");
                    let ser_msg = serialize_message(&result.unwrap());
                    match ser_msg {
                        Ok(m) => {
                            send_message(address, m);
                            process_message(rx, address) // Recursive call

                        }
                        Err(e) => {
                            log::error!("Error: {:?}", e);
                            ()
                        }
                    }
                }
            }
        }
        Err(err) => {
            log::error!("Error: {:?}", err);
        }
    };
}

fn start_process_message_thread(
    rx: flume::Receiver<Vec<String>>,
    address: SocketAddrV4,
) -> JoinHandle<()> {
    thread::spawn(move || {
        //log::info!("Starting process message thread.");
        process_message(rx, address.to_string().as_str());
    })
}
// Takes already serialized message in String object
fn send_message(address: &str, ser_message: String) {
    if let Ok(mut stream) = TcpStream::connect(address) {
        // Send the length of the serialized message (as 4-byte value).
        let len = ser_message.len() as u32;
        stream.write_all(&len.to_be_bytes()).unwrap();

        // Send the serialized message.
        stream.write_all(ser_message.as_bytes()).unwrap();

        log::info!("Transfer complete!");
    } else {
        log::error!("Could not connect to server.");
    }
}

pub fn start_multithreaded(address: SocketAddrV4) -> Result<Vec<String>, Box<dyn Error>> {
    log::info!("Starting interactive mode...");
    let (tx, rx) = flume::unbounded();
    // Thread for reading from stdin
    let t_input = start_input_thread(tx);
    // Thread that processes stdin and submits data to server
    let _t_process_input = start_process_message_thread(rx, address);
    t_input.join().unwrap();
    Ok(vec![])
}
