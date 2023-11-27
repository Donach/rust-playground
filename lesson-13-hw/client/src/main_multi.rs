use std::error::Error;

use std::net::{SocketAddrV4, TcpStream};
use std::thread::{self};

use anyhow::Context;
use flume::Sender;
use library::{await_input, handle_stream_message, read_from_stream, write_to_stream, MessageType};
use log;

use crate::input_handler::handle_vec_input;

// Currently can process only single line of text, known limitation
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
    match input_parsed[0] == ".quit" || input_parsed[0] == ".q" {
        true => {
            tx.send(input_parsed).unwrap();
            Ok(vec!["Exitting...".to_string()])
        }
        false => {
            if input_parsed[0].len() > 0 {
                tx.send(input_parsed).unwrap();
            }
            process_input(tx) // Recursive call
        }
    }
}

fn process_message(rx: flume::Receiver<Vec<String>>, address: &str) {
    match rx.recv() {
        Err(err) => {
            log::error!("Error: {:?}", err);
        }
        Ok(message) => {
            let result = match handle_vec_input(message) {
                Err(e) => {
                    log::info!("{}", e.to_string());
                    process_message(rx, address);
                    return;
                }
                Ok(result) => result,
            };

            // If input is parsed correctly, let's connect to server and send some data there
            if let Ok(stream) = TcpStream::connect(address) {
                log::info!("Sending data to server...");
                match write_to_stream(stream, &result) {
                    Err(e) => {
                        log::error!("Error: {:?}", e);
                    }
                    Ok(_stream) => {
                        // Read response from server
                        log::trace!("Data sent!");
                        /* let (_, msg) = read_from_stream(stream);
                        match msg {
                            MessageType::Error(_)
                            | MessageType::Image(_)
                            | MessageType::File(_, _) => {
                                log::info!("Invalid response - expected Text response.");
                            }
                            MessageType::Text(text) => {
                                log::info!("Response: {}", text);
                            }
                        }; */
                    }
                }
            } else {
                log::error!("Could not connect to server.");
            }

            // Lastly, continue recursively waiting for new user input
            process_message(rx, address); // Recursive call
        }
    }
}

fn receive_message(stream: TcpStream) {
    if let Ok(stream) = stream.try_clone() {
        let (stream, msg) = read_from_stream(stream);
        match msg {
            MessageType::Error(_e) => {
                log::error!("Server disconnected.");
            }
            _ => {
                handle_stream_message(msg);
                receive_message(stream);
            }
        }
    } else {
        log::error!("Server disconnected.");
    }
}

pub fn start_multithreaded(address: SocketAddrV4) -> Result<Vec<String>, Box<dyn Error>> {
    log::info!("Starting interactive mode...");
    let stream = TcpStream::connect(address).context("Failed to connect to server");

    let stream_clone = match stream {
        Ok(stream) => stream.try_clone().context("Stream clone failed"),
        Err(err) => {
            log::error!("Error: {:?}", err);
            Err(err)
        }
    };

    match stream_clone {
        Err(_err) => {
            log::info!("Exitting...");
        }
        Ok(stream_clone) => {
            let (tx, rx) = flume::unbounded();
            // Thread for reading from stdin
            let t_input = thread::spawn(move || {
                //log::info!("Starting process input thread.");
                let _ = process_input(tx);
            });
            // Thread that processes stdin and submits data to server
            let _t_process_input = thread::spawn(move || {
                //log::info!("Starting process message thread.");
                process_message(rx, address.to_string().as_str());
            });

            // Thread that reads data from server
            let _t_read = thread::spawn(move || {
                //log::info!("Starting process message thread.");
                receive_message(stream_clone);
            });
            t_input.join().unwrap();
        }
    };
    Ok(vec![])
}
