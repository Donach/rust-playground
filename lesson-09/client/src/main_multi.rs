use std::error::Error;
use std::io::Write;
use std::net::{TcpStream, SocketAddrV4};
use std::thread::{self, JoinHandle};

use flume::Sender;
use library::serialize_message;

use crate::input_handler::{await_input, handle_vec_input};

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
    println!("{:?}", &input_parsed);
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
        println!("Starting process input thread.");
        let _ = process_input(tx);
    })
}

fn process_message(rx: flume::Receiver<Vec<String>>, address: &str) {
    let message = rx.recv();
    match &message {
        Ok(_res) => {
            //println!("Received: {:?}", res);
            // Decode the message sent for the Operation type
            let message = message.unwrap();
            //let operation = Operation::from(message[0].as_str());
            let result = handle_vec_input(message);
            match &result {
                Err(e) => {
                    eprintln!("Exitting... {:?}", e);
                }
                _ => {
                    send_message(address, serialize_message(&result.unwrap()));
                    process_message(rx, address)
                } // Recursive call}
            }
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    };
}

fn start_process_message_thread(rx: flume::Receiver<Vec<String>>, address: SocketAddrV4) -> JoinHandle<()> {
    thread::spawn(move || {
        println!("Starting process message thread.");
        process_message(rx, address.to_string().as_str());
    })
}


fn send_message(address: &str, ser_message: String) {
    //let serialized = serialize_message(message);
    if let Ok(mut stream) = TcpStream::connect(address) {
        //println!("Sending message: {}", ser_message);
        // Send the length of the serialized message (as 4-byte value).
        println!("Sending data to server...");
        let len = ser_message.len() as u32;
        stream.write_all(&len.to_be_bytes()).unwrap();
    
        // Send the serialized message.
        stream.write_all(ser_message.as_bytes()).unwrap();

    }else {
        eprintln!("Could not connect to server.");
    }
}


pub fn start_multithreaded(address: SocketAddrV4) -> Result<Vec<String>, Box<dyn Error>> {
    println!("Starting interactive mode...");
    let (tx, rx) = flume::unbounded();
    let t_input = start_input_thread(tx);

    let _t_process_input = start_process_message_thread(rx, address);
    //t_process_input.join().unwrap();
    t_input.join().unwrap();
    Ok(vec![])
}