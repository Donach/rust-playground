use std::{error::Error, fs::File, io::Read, path::Path};

use anyhow::Result;
use library::MessageType;
use log;

#[derive(Debug)]
pub enum Operation {
    File,
    Image,
    Quit,
    Text,
}
impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        return match value.to_lowercase().trim() {
            ".file" => {
                log::trace!("Operation: File");
                Operation::File
            }
            ".image" => {
                log::trace!("Operation: Image");
                Operation::Image
            }
            ".quit" => {
                log::trace!("Operation: Quit");
                Operation::Quit
            }
            ".q" => {
                log::trace!("Operation: Quit");
                Operation::Quit
            }
            _ => {
                log::trace!("Operation: Text");
                Operation::Text
            }
        };
    }
}

fn read_file_to_bytes(filename: &String) -> Result<Vec<u8>, Box<dyn Error>> {
    log::info!("File: {}", filename);
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(error) => {
            log::error!("Failed to open file: {}", error);
            return Err(Box::new(error));
        }
    };

    let mut input: Vec<u8> = Vec::new();
    let _ = match file.read_to_end(&mut input) {
        Ok(_) => Ok(&input),
        Err(error) => {
            log::error!("Failed to read file: {}", error);
            Err(Box::new(error))
        }
    };
    Ok(input)
}

fn get_file_as_messagetype(filename: String) -> Result<MessageType, Box<dyn Error>> {
    let input = read_file_to_bytes(&filename)?;
    let path = Path::new(filename.as_str());
    Ok(MessageType::File(
        path.file_name().unwrap().to_str().unwrap().to_string(),
        input,
    ))
}

fn get_image_as_messagetype(filename: String) -> Result<MessageType, Box<dyn Error>> {
    let input = read_file_to_bytes(&filename)?;
    Ok(MessageType::Image(input))
}

fn handle_text(input: &str) -> Result<MessageType, Box<dyn Error>> {
    Ok(MessageType::Text(input.to_string()))
}

fn handle_file(input: &str) -> Result<MessageType, Box<dyn Error>> {
    let (_left, right) = match input.splitn(2, ' ').collect::<Vec<&str>>().as_slice() {
        [left, right] => (*left, *right),
        _ => {
            log::error!("Error: Invalid input");
            ("", "")
        }
    };
    get_file_as_messagetype(right.to_string())
}

fn handle_image(input: &str) -> Result<MessageType, Box<dyn Error>> {
    let (_left, right) = match input.splitn(2, ' ').collect::<Vec<&str>>().as_slice() {
        [left, right] => (*left, *right),
        _ => {
            log::error!("Error: Invalid input");
            ("", "")
        }
    };
    get_image_as_messagetype(right.to_string())
}

fn handle_operation(operation: &Operation, input: &str) -> Result<MessageType, Box<dyn Error>> {
    match operation {
        Operation::File => handle_file(input),
        Operation::Image => handle_image(input),
        Operation::Quit => Err("Exitting...".into()),
        Operation::Text => handle_text(input),
    }
}

pub fn handle_vec_input(input: Vec<String>) -> Result<MessageType, Box<dyn Error>> {
    let operation = Operation::from(input[0].as_str());
    handle_operation(&operation, &input.join(" "))
}
