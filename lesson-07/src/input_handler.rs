use std::{
    error::Error,
    io::{self, Write},
};

use slug::slugify;

use crate::csv_wrapper::handle_file as handle_csv_file;
use crate::csv_wrapper::handle_input as handle_csv_input;

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
    CsvFile,
    NoCommand,
    Invalid,
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
            "csvfile" => {
                println!("Operation: Csv-File");
                Operation::CsvFile
            }
            "" => {
                println!("Empty command - exiting program");
                Operation::NoCommand
            }
            _ => {
                eprintln!(
                    "Invalid operation: '{}', valid operations are: {:?}",
                    value, OPERATIONS
                );
                Operation::Invalid
            }
        };
    }
}

fn handle_lowercase(input: &str) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(vec![input.to_string(), input.to_lowercase()])
}

fn handle_uppercase(input: &str) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(vec![input.to_string(), input.to_uppercase()])
}

fn handle_no_spaces(input: &str) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(vec![input.to_string(), input.replace(' ', "")])
}

fn handle_slugify(input: &str) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(vec![input.to_string(), slugify(input.trim())])
}

fn handle_operation(operation: &Operation, input: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match operation {
        Operation::Uppercase => handle_uppercase(input),
        Operation::Lowercase => handle_lowercase(input),
        Operation::NoSpaces => handle_no_spaces(input),
        Operation::Slugify => handle_slugify(input),
        Operation::Csv | Operation::CsvFile => {
            let csv_output: Result<crate::csv_wrapper::Csv, Box<dyn Error>> = match operation {
                Operation::CsvFile => handle_csv_file(input.to_string()),
                _ => handle_csv_input(vec![input.to_string()]),
            };
            match csv_output {
                Ok(csv_output) => Ok(vec![
                    format!("{:?}", csv_output.input),
                    format!("\n{}", csv_output),
                ]),
                Err(err) => Err(err),
            }
        }
        Operation::NoCommand => Err("Empty commands are invalid!".into()),
        _ => panic!("Invalid operation!"),
    }
}

fn match_operation(operation: &Operation, input: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match handle_operation(operation, input) {
        Ok(output) => {
            //println!("Transmuted text: {}", output);
            Ok(output)
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            Err(err)
        }
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

fn handle_input(operation: &Operation, input: &String) -> Result<Vec<String>, Box<dyn Error>> {
    io::stdout().flush().unwrap();

    let result = match &operation {
        Operation::NoCommand | Operation::Csv | Operation::CsvFile => {
            match_operation(operation, input)
        }
        _ => {
            let input = if input.is_empty() {
                await_input()?
            } else {
                input.to_string()
            };
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
    result
}

pub fn handle_vec_input(input: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
    let operation = Operation::from(input[0].as_str());
    handle_input(&operation, &input[1])
}
