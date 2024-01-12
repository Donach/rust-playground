//! Server of chat application, able to receive and send messages from/to clients
//!
//! # Usage
//!
//! ```
//! cargo run --bin server <hostname> <port>
//! ```
//!
//!
use server::server_main;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    server_main()
}
