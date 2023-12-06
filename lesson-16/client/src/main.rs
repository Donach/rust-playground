#![doc(html_logo_url = "https://imgs.search.brave.com/Jo2o-YdMHH_YqFBDUpMQnOBiqwtv-jYiIMyxh6UHTts/rs:fit:560:320:1/g:ce/aHR0cHM6Ly91cGxv/YWQud2lraW1lZGlh/Lm9yZy93aWtpcGVk/aWEvY29tbW9ucy90/aHVtYi80LzQ2L0Jp/dGNvaW4uc3ZnLzUx/MnB4LUJpdGNvaW4u/c3ZnLnBuZw")]
#![deny(missing_docs)]
//! Client for chat application
//! 
//! # Example
//! 
//! ```bash
//! ./client 127.0.0.1 8080 <guid>
//! ```
//! 
//! A simple client application to send messages to server, broadcasted to other clients.
//! The messages can be both text or files/images.
use std::{env, error::Error};

use library::get_addr;

use crate::main_multi::start_multithreaded;
mod input_handler;
mod main_multi;
mod test_input_handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    // This runs lib function to parse hostname and port, or use default
    // It is inherited from previous design, using two threads - one to read from stdin, one to send data to server
    start_multithreaded(get_addr(env::args().collect()).unwrap()).await
}
