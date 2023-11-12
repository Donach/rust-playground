use std::error::Error;
use std::thread::{self, JoinHandle};

use flume::Sender;

use crate::input_handler::{await_input, handle_vec_input};

fn process_input(tx: Sender<Vec<String>>) -> Result<Vec<String>, Box<dyn Error>> {
    println!("Enter operation to perform followed by text to transmute:");
    let input = await_input()?;

    let (left, right) = match input.splitn(2, ' ').collect::<Vec<&str>>().as_slice() {
        [left, right] => (*left, *right),
        _ => ("", ""),
    };
    let input_parsed = vec![left.to_string(), right.to_string()];
    println!("{:?}", &input_parsed);
    match input_parsed[0].is_empty() {
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

fn process_message(rx: flume::Receiver<Vec<String>>) {
    let message = rx.recv();
    match &message {
        Ok(res) => {
            println!("Received: {:?}", res);
            let _ = handle_vec_input(message.unwrap());
            process_message(rx) // Recursive call
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    };
}

fn start_process_message_thread(rx: flume::Receiver<Vec<String>>) -> JoinHandle<()> {
    thread::spawn(move || {
        println!("Starting process message thread.");
        process_message(rx);
    })
}

pub fn start_multithreaded() -> Result<Vec<String>, Box<dyn Error>> {
    println!("Starting interactive mode...");
    let (tx, rx) = flume::unbounded();
    let t_input = start_input_thread(tx);

    let _t_process_input = start_process_message_thread(rx);
    //t_process_input.join().unwrap();
    t_input.join().unwrap();
    Ok(vec![])
}
