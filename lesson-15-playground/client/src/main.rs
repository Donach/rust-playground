use simple_logger;

use std::{env, error::Error};

use library::get_addr;

use crate::main_multi::start_multithreaded;
mod input_handler;
mod main_multi;

#[tokio::main]
async fn main()-> Result<(), Box<dyn Error>> {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    // For this version, I'm using static predefined UIDs for 2 clients
    let uids = vec!["a0048b4a-63ab-4662-b6c0-087921ec1de8", "164d5fff-1ec8-4989-9dda-ea16f5b2c637"];

    // This runs lib function to parse hostname and port, or use default
    // It is inherited from previous design, using two threads - one to read from stdin, one to send data to server
    start_multithreaded(get_addr(env::args().collect()).unwrap()).await
}
