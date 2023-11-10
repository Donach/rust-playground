
use std::{io::{self, Write}, error::Error};

use slug::slugify;

use crate::csv_wrapper::handle_input as handle_csv;
use crate::main_multi::{INTERACTIVE_MODE_ACTIVE, start_multithreaded};


const OPERATIONS: [&str; 6] = [
    "uppercase",
    "lowercase",
    "no-spaces",
    "slugify",
    "csv",
    "<empty> for Interactive Mode",
];

#[derive(Debug)]
pub enum Operation {
    Uppercase,
    Lowercase,
    NoSpaces,
    Slugify,
    Csv,
    NoCommand,
    INVALID,
}
impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        return match value.to_lowercase().as_str() {
            "uppercase" => {
                println!("Operation: Uppercase");
                Operation::Uppercase
            }
            "lowercase" => {
                println!("Operation: Lowercase");
                Operation::Lowercase
            }
            "no-spaces" => {
                println!("Operation: NoSpaces");
                Operation::NoSpaces
            }
            "slugify" => {
                println!("Operation: Slugify");
                Operation::Slugify
            }
            "csv" => {
                println!("Operation: Csv");
                Operation::Csv
            }
            "" => {
                println!("Interactive mode");
                Operation::NoCommand
            }
            _ => {
                eprintln!(
                    "Invalid operation: '{}', valid operations are: {:?}",
                    value, OPERATIONS
                );
                Operation::INVALID
            }
        };
    }
}

fn handle_lowercase(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.to_lowercase())
}

fn handle_uppercase(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.to_uppercase())
}

fn handle_no_spaces(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.replace(' ', ""))
}

fn handle_slugify(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(slugify(input.trim()))
}

fn handle_operation(operation: &Operation, input: &str) -> Result<String, Box<dyn Error>> {
    match operation {
        Operation::Uppercase => handle_uppercase(input),
        Operation::Lowercase => handle_lowercase(input),
        Operation::NoSpaces => handle_no_spaces(input),
        Operation::Slugify => handle_slugify(input),
        Operation::NoCommand => Err("Empty commands are invalid!".into()),
        _ => panic!("Invalid operation!"),
    }
}

fn match_operation(operation: &Operation, input: &String) -> Result<Vec<String>, Box<dyn Error>> {
    match handle_operation(operation, &input) {
        Ok(output) => {
            //println!("Transmuted text: {}", output);
            Ok(vec![input.to_string(), output])
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            Err(err)
        }
    }
}

pub fn await_input() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input = input.trim().to_string();
    Ok(input)
}

fn handle_input(operation: &Operation) -> Result<Vec<String>, Box<dyn Error>> {
    print!("Enter text to transmute: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    let result = match &operation {
        Operation::Csv => {
            let csv_output = handle_csv()?;
            Ok(vec![
                format!("{:?}", csv_output.input),
                format!("\n{}", csv_output),
            ])
        }
        Operation::NoCommand => {
            match_operation(operation, &input)
        }
        _ => {
            input = await_input()?;
            match_operation(operation, &input)
        }
    };

    match &result {
        Ok(output) => {
            println!(
                "Success - operation '{:?}' transmuted input '{}' to '{}'",
                operation, &output[0], output[1]
            );
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    };
    return result
}


pub fn handle_str_input(input: String) -> Result<Vec<String>, Box<dyn Error>> {
    let operation = Operation::from(input.as_str());
    handle_input(&operation)
}
