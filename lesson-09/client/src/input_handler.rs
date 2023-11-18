use std::{
    error::Error,
    io::{self, Write},
};

use library::MessageType;
use image::codecs::png::PngEncoder;
use image::io::Reader as ImageReader;

use crate::csv_wrapper::{handle_file_to_string};

const OPERATIONS: [&str; 4] = [
    ".file",
    ".image",
    ".quit",
    "<any> for regular text",
];

#[derive(Debug)]
pub enum Operation {
    NoCommand,
    Invalid,
    File,
    Image,
    Quit
}
impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        return match value.to_lowercase().as_str() {
            ".file" => {
                println!("Operation: File");
                Operation::File
            }
            ".image" => {
                println!("Operation: Image");
                Operation::Image
            }
            ".quit" => {
                println!("Operation: Quit");
                Operation::Quit
            }
            "" => {
                println!("Empty command");
                Operation::NoCommand
            }
            _ => {
                println!("Any text");
                Operation::NoCommand
            }
        };
    }
}

fn handle_text(input: &str) -> Result<MessageType, Box<dyn Error>> {
    Ok(MessageType::Text(input.to_string()))
}

fn handle_file(input: &str) -> Result<MessageType, Box<dyn Error>> {
    let (_left, right) = match input.splitn(2, ' ').collect::<Vec<&str>>().as_slice() {
        [left, right] => (*left, *right),
        _ => {eprintln!("Error: Invalid input"); ("", "")},
    };
    handle_file_to_string(right.to_string())
}

fn handle_image(input: &str) -> Result<MessageType, Box<dyn Error>> {
    let (_left, right) = match input.splitn(2, ' ').collect::<Vec<&str>>().as_slice() {
        [left, right] => (*left, *right),
        _ => {eprintln!("Error: Invalid input"); ("", "")},
    };
    let mut bytes: Vec<u8> = Vec::new();
    let img = ImageReader::open(right)?.decode()?;
    match img.write_with_encoder(PngEncoder::new(&mut bytes)) {
        Ok(_res) => {
            Ok(MessageType::Image(bytes))
        },
        Err(err) => {
            eprintln!("Error: Cannot encode image to PNG {:?}", err);
            Ok(MessageType::Empty)
        }
    }
}

fn handle_operation(operation: &Operation, input: &str) -> Result<MessageType, Box<dyn Error>> {
    match operation {
        Operation::File => handle_file(input),
        Operation::Image => handle_image(input),
        Operation::Quit => Err("Exiting...".into()),
        Operation::NoCommand => handle_text(input),
        _ => panic!("Invalid operation!"),
    }
}

pub fn await_input() -> Result<String, Box<dyn Error>> {
    print!("Enter text to transmute: ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_res) => {
            if input == "q" || input.is_empty() {
                Err("Exiting...".into())
            } else {
                return Ok(input.trim().to_string());
            }
        }
        Err(err) => Err(err.into()),
    }
}

fn handle_input(operation: &Operation, input: &String) -> Result<MessageType, Box<dyn Error>> {
    io::stdout().flush().unwrap();

    let result = match &operation {
        Operation::NoCommand => {
            handle_operation(operation, input)
        }
        _ => {
            let input = if input.is_empty() {
                await_input()?
            } else {
                input.to_string()
            };
            handle_operation(operation, &input)
        }
    };

    match &result {
        Ok(output) => {
            println!(
                "Success - operation '{:?}'",
                operation
            );
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    };
    result
}

pub fn handle_vec_input(input: Vec<String>) -> Result<MessageType, Box<dyn Error>> {
    let operation = Operation::from(input[0].as_str());
    handle_input(&operation, &input.join(" "))
}