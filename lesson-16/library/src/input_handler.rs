//! Module for handling various input types
//!
//! # Examples
//! handle_vec_input(vec![".file".to_string(), "/full/path/to/file.txt".to_string()])
//! handle_vec_input(vec![".image".to_string(), "/full/path/to/image.png".to_string()])
//! handle_vec_input(vec![".quit".to_string()])
//! handle_vec_input(vec![".text".to_string(), "Hello World".to_string()])
//!
//!
//! There are several defined operations which can be used.
//! For each operation a function is defined.
//! "Auth" is only used for authentication, and is not intended to be used manually (but right now it could be).
use std::{error::Error, fs::File, io::Read, path::Path};

use anyhow::Result;
use crate::MessageType;

#[derive(Debug)]
pub enum Operation {
    File,
    Image,
    Quit,
    Text,
    Auth, // TODO: Add LoadAll - load all missed messages by this client
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
            ".auth" => {
                log::trace!("Operation: Authenticaiton");
                Operation::Auth
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

fn handle_auth(input: &str) -> Result<MessageType, Box<dyn Error>> {
    Ok(MessageType::Auth(
        input.split(' ').collect::<Vec<&str>>()[1].to_string(),
    ))
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
        Operation::Auth => handle_auth(input),
    }
}
/// Handles the input from the user
///
/// # Arguments
/// `input` - The input from the user, it will be pre-parsed to two parts - "command" as "left" and the rest as "right" part
/// # Examples
/// handle_vec_input(vec![".file".to_string(), "/full/path/to/file.txt".to_string()])
/// handle_vec_input(vec![".image".to_string(), "/full/path/to/image.png".to_string()])
/// handle_vec_input(vec![".quit".to_string()])
/// handle_vec_input(vec![".text".to_string(), "Hello World".to_string()])
pub fn handle_vec_input(input: Vec<String>) -> Result<MessageType, Box<dyn Error>> {
    let operation = Operation::from(input[0].as_str());
    handle_operation(&operation, &input.join(" "))
}
