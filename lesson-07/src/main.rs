use std::env;
mod input_handler;
use input_handler::handle_vec_input;

use crate::main_multi::start_multithreaded;
mod csv_wrapper;
mod main_multi;
fn main() {
    let args: Vec<String> = env::args().collect();
    // Evaluate args
    println!("{:?}", args);
    if args.len() <= 1 {
        // Interactive multithreaded mode
        let _ = start_multithreaded(); //should actually not continue towards the return
    } else {
        let _ = handle_vec_input(args[1..].to_vec());
    }
}
