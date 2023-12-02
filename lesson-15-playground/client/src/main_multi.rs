use std::error::Error;

use std::net::{SocketAddrV4};
use std::thread::{self};

use anyhow::Context;
use flume::Sender;
use library::{await_input, handle_stream_message, read_from_stream, write_to_stream, MessageType};
use log;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::input_handler::handle_vec_input;

// Currently can process only single line of text, known limitation
fn process_input(tx: Sender<Vec<String>>) -> Result<Vec<String>, Box<dyn Error>> {
    loop {
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
                break Ok(vec!["Exitting...".to_string()])
                //Ok(vec!["Exitting...".to_string()])
            }
            false => {
                if input_parsed[0].len() > 0 {
                    tx.send(input_parsed).unwrap();
                }
                //process_input(tx) // Recursive call
            }
        }
    }
}

async fn process_message(rx: flume::Receiver<Vec<String>>, stream: tokio::io::WriteHalf<tokio::net::TcpStream>) {
    let mut stream = stream;
    loop {
        match rx.recv() {
            Err(err) => {
                log::error!("Error: {:?}", err);
            }
            Ok(message) => {
                let result = match handle_vec_input(message) {
                    Err(e) => {
                        log::info!("Error: {}", e.to_string());
                        //let _ = process_message(rx, stream);
                        break;
                    }
                    Ok(result) => result,
                };

                // If input is parsed correctly, let's connect to server and send some data there
                log::info!("Sending data to server...");
                let result = write_to_stream(stream, &result).await;
                match result {
                    Ok(s) => {
                        stream = s;
                        log::info!("Transfer complete!");
                    },
                    Err(e) => {
                        log::error!("Error: {}", e);
                        break
                    },
                }
                /* match write_to_stream(stream, &result).await {
                    Err(e) => {
                        log::error!("Error: {:?}", e);
                    }
                    Ok(stream) => {
                        // Read response from server
                        log::trace!("Data sent!");
                        // Lastly, continue recursively waiting for new user input
                        //let _ = process_message(rx, stream); // Recursive call
                    }
                }; */
            }
        }
    }
}

async fn receive_message(stream: tokio::io::ReadHalf<tokio::net::TcpStream>) {
    let mut stream = stream;
    loop {
        let (s, msg) = read_from_stream(stream).await;
        stream = s;
        match msg {
            MessageType::Error(_e) => {
                log::error!("Server disconnected.");
                break;
            }
            _ => {
                handle_stream_message(msg);
            }
        }
    }
}

pub async fn start_multithreaded(address: SocketAddrV4) -> Result<(), Box<dyn Error>> {
    log::info!("Starting interactive mode @{}", address);
    let stream = TcpStream::connect(address).await?;
    let (reader, writer) = tokio::io::split(stream);

    let write_task = tokio::spawn(async move {
        log::info!("Starting write task...");
        // Thread for reading from stdin
        let (tx, rx) = flume::unbounded();
        let t_input = tokio::spawn(async move {
            //log::info!("Starting process input thread.");
            log::info!("Starting process_input task...");
            let _ = process_input(tx);
        });
        // Thread that processes stdin and submits data to server
        let t_process_input = tokio::spawn(async move {
            //log::info!("Starting process message thread.");
            log::info!("Starting process_message task...");
            process_message(rx, writer).await
        });

        let _ = tokio::try_join!(t_input, t_process_input);
    });

    let read_task = tokio::spawn(async move {
        log::info!("Starting reader task...");
        // Thread that reads data from server
            //log::info!("Starting process message thread.");
        
        //receive_message(reader).await
        let mut stream = reader;
        loop {
            let (s, msg) = read_from_stream(stream).await;
            stream = s;
            match msg {
                MessageType::Error(e) => {
                    log::error!("Server disconnected: {}", e);
                    break
                }
                _ => {
                    handle_stream_message(msg);
                }
            }
        }
        
    });
    let _ = tokio::join!(read_task, write_task);
    log::info!("Last Line");
    Ok(())
}
