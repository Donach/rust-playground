use slug::slugify;
use std::env;
use std::error::Error;
use std::io::{self, Write};

mod csv_wrapper;
use csv_wrapper::csv_wrapper::handle_input as handle_csv;

const OPERATIONS: [&str; 5] = ["uppercase", "lowercase", "no-spaces", "slugify", "csv"];
enum Operation {
    Uppercase,
    Lowercase,
    NoSpaces,
    Slugify,
    Csv,
}
fn handle_lowercase(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.to_lowercase())
}
fn handle_uppercase(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.to_uppercase())
}
fn handle_no_spaces(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.replace(" ", ""))
}
fn handle_slugify(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(slugify(input.trim().to_string()))
}

fn handle_operation(operation: &Operation, input: &str) -> Result<String, Box<dyn Error>> {
    match operation {
        Operation::Uppercase => handle_uppercase(&input),
        Operation::Lowercase => handle_lowercase(&input),
        Operation::NoSpaces => handle_no_spaces(&input),
        Operation::Slugify => handle_slugify(&input),
        _ => panic!("Unhandled operation!"),
    }
}

fn handle_input(operation: &Operation) -> Result<Vec<String>, Box<dyn Error>> {
    print!("Enter text to transmute: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    match &operation {
        Operation::Csv => {
            let csv_output = handle_csv()?;
            Ok(vec![format!("{:?}", csv_output.input), format!("\n{}", csv_output)])
        }
        _ => {
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");
            input = input.trim().to_string();
            match handle_operation(&operation, &input) {
                Ok(output) => {
                    //println!("Transmuted text: {}", output);
                    Ok(vec![input, output])
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                    Err(err)
                }
            }
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    // Evaluate args
    if args.len() != 2 {
        eprintln!(
            "Entered args: {:?}, valid arguments are: {:?}",
            args, OPERATIONS
        );
        return;
    }
    let operation = match args[1].as_str() {
        "uppercase" => {
            format!("Operation: Uppercase");
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
        _ => {
            eprintln!("Invalid operation: '{}', valid operations are: {:?}", args[1], OPERATIONS);
            return;
        }
    };
    let result: Result<Vec<String>, Box<dyn Error>> = handle_input(&operation);

    match result {
        Ok(output) => {
            println!(
                "Success - operation '{}' transmuted input '{}' to '{}'",
                args[1], &output[0], output[1]
            );
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    };
}
