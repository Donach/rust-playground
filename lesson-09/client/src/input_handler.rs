use std::{
    error::Error,
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

use library::MessageType;

#[derive(Debug)]
pub enum Operation {
    NoCommand,
    File,
    Image,
    Quit,
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

fn read_file_to_bytes(filename: &String) -> Result<Vec<u8>, Box<dyn Error>> {
    println!("File: {}", filename);
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(error) => {
            println!("Failed to open file: {}", error);
            return Err(Box::new(error));
        }
    };

    let mut input: Vec<u8> = Vec::new();
    let _ = match file.read_to_end(&mut input) {
        Ok(_) => Ok(&input),
        Err(error) => {
            eprintln!("Failed to read file: {}", error);
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
            eprintln!("Error: Invalid input");
            ("", "")
        }
    };
    get_file_as_messagetype(right.to_string())
}

fn handle_image(input: &str) -> Result<MessageType, Box<dyn Error>> {
    let (_left, right) = match input.splitn(2, ' ').collect::<Vec<&str>>().as_slice() {
        [left, right] => (*left, *right),
        _ => {
            eprintln!("Error: Invalid input");
            ("", "")
        }
    };
    get_image_as_messagetype(right.to_string())
}

fn handle_operation(operation: &Operation, input: &str) -> Result<MessageType, Box<dyn Error>> {
    match operation {
        Operation::File => handle_file(input),
        Operation::Image => handle_image(input),
        Operation::Quit => Err(".quit".into()),
        Operation::NoCommand => handle_text(input),
    }
}

pub fn await_input() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_res) => {
            if input == "q" || input.is_empty() {
                Err(".quit".into())
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
        Operation::NoCommand => handle_operation(operation, input),
        _ => {
            let input = if input.is_empty() {
                await_input()?
            } else {
                input.to_string()
            };
            handle_operation(operation, &input)
        }
    };

    if let Ok(_output) = &result {
        println!("Success - operation '{:?}'", operation);
    }
    result
}

pub fn handle_vec_input(input: Vec<String>) -> Result<MessageType, Box<dyn Error>> {
    let operation = Operation::from(input[0].as_str());
    handle_input(&operation, &input.join(" "))
}
