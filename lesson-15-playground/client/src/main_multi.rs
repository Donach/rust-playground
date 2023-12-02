use std::error::Error;

use std::net::{SocketAddrV4};
use std::thread::{self};

use anyhow::Context;
use flume::Sender;
use library::{await_input, handle_stream_message, read_from_stream, write_to_stream, MessageType};
use log;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedWriteHalf, OwnedReadHalf};
use tokio::time::{self, Duration};

use crate::input_handler::handle_vec_input;

// Currently can process only single line of text, known limitation
fn process_input(tx: Sender<Vec<String>>) -> Result<(), Box<dyn Error>> {
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
                //tx.send(input_parsed).unwrap();
                break Ok(log::info!("Quit"))
                //Ok(vec!["Exitting...".to_string()])
            }
            false => {
                if input_parsed[0].len() > 0 {
                    match tx.send(input_parsed) {
                        Ok(_) => {

                        }
                        Err(e) => {
                            break Ok(log::error!("Error: {}", e))
                        }
                    }
                }
                //process_input(tx) // Recursive call
            }
        }
    }
}

async fn process_message(rx: flume::Receiver<Vec<String>>, stream: OwnedWriteHalf) {
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
                let result = write_to_stream(&mut stream, &result).await;
                match result {
                    Ok(_s) => {
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

async fn receive_message(stream: OwnedReadHalf) {
    let mut stream = stream;
    loop {
        let msg = read_from_stream(&mut stream).await.unwrap();
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
    // Define the retry interval and total retry duration
    let retry_interval = Duration::from_secs(10);
    let total_retry_duration = Duration::from_secs(10 * 60); // 10 minutes
    let start_time = time::Instant::now();

    loop {
        match TcpStream::connect(address).await {
            Err(e) => {
                log::error!("Failed to connect: {}", e);

                // Check if total retry duration is exceeded
                if time::Instant::now().duration_since(start_time) > total_retry_duration {
                    log::error!("Failed to connect to server after 10 minutes, exiting.");
                    return Err(Box::new(e));
                }

                // Wait for the retry interval
                time::sleep(retry_interval).await;
            }
            Ok(stream) => {
                let (reader, writer) = stream.into_split();

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
                        let res = read_from_stream(&mut stream).await;
                        match res {
                            Ok(msg) => {
                                match msg {
                                    MessageType::Error(e) => {
                                        log::error!("Server disconnected: {}", e);
                                        return
                                    }
                                    _ => {
                                        handle_stream_message(msg);
                                    }
                                }
                            },
                            Err(e) => {
                                log::error!("Server disconnected: {}", e);
                                return
                            }
                        }
                        
                    }
                    
                });
                let _ = tokio::join!(read_task, write_task);
                log::info!("Last Line");
                //break; // Break out of the loop once the connection is established
            },
        }
    }
        
    
    Ok(())
}
