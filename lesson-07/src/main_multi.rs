use std::{error::Error, io};
use std::thread::{self, JoinHandle};

use crate::input_handler::{await_input, handle_str_input};

pub static INTERACTIVE_MODE_ACTIVE: bool = false;
static INTERACTIVE_CONTINUE: bool = true;


fn process_input() -> Result<Vec<String>, Box<dyn Error>> {
    println!("Enter operation to perform:");
    let input = await_input()?;
    handle_str_input(input)
}

fn start_input_thread(tx: flume::Sender<Vec<String>>) -> JoinHandle<()> {
    let t = thread::spawn(move || {
        let message = "Hello from spawned thread!";
        let input = await_input();
        let result = handle_str_input(input.unwrap());
        println!("{:?}", result);
        match result {
            Ok(res) => tx.send(res).unwrap(),
            Err(err) => tx.send(vec!["Error: Invalid input".to_string()]).unwrap(),
        }
        println!("Message sent.");
    });
    t
}

fn process_message(tx: flume::Sender<Vec<String>>, rx: flume::Receiver<Vec<String>>) {
    
    let t = start_input_thread(tx);
    t.join().unwrap();
    let received_message = rx.recv().unwrap();
    println!("Received: {:?}", &received_message);
    
}

pub fn start_multithreaded() -> Result<Vec<String>, Box<dyn Error>> {
    println!("Starting interactive mode...");
    let (tx, rx) = flume::unbounded();
    process_message(tx, rx);


    todo!()
}
