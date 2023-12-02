use simple_logger;

use std::{env, error::Error};

use library::get_addr;

use crate::main_multi::start_multithreaded;
mod input_handler;
mod main_multi;

#[tokio::main]
async fn main()-> Result<(), Box<dyn Error>> {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    // This runs lib function to parse hostname and port, or use default
    // It is inherited from previous design, using two threads - one to read from stdin, one to send data to server
    start_multithreaded(get_addr(env::args().collect()).unwrap()).await
}
