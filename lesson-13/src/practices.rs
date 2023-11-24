// Handling different errors without conversions
#[cfg(debug_assertions)]
use color_eyre::eyre;
#[cfg(not(debug_assertions))]
use ::anyhow as eyre;

use eyre::{Result, anyhow, Context};

use std::io;
use std::num::ParseIntError;

enum MyError {
	Io(io::Error),
	Parse(ParseIntError),
}

// impl Error...

fn process_file(path: &str) -> Result<i32, MyError> {
    // io::Error  -> MyError::Io(..)
	let contents = std::fs::read_to_string(path).map_err(MyError::Io)?;
	let number = contents.trim().parse::<i32>().map_err(MyError::Parse)?;
	Ok(number)
}



// Downcasting errors
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyCustomError;

impl fmt::Display for MyCustomError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    	write!(f, "My custom error")
	}
}

impl Error for MyCustomError {}

fn get_error() -> Box<dyn Error> {
	Box::new(MyCustomError)
}

// Usage
fn main () {
    let error = get_error();
    if let Some(specific_error) = error.downcast_ref::<MyCustomError>() {
        println!("Caught specific error: {}", specific_error);
    }
}





// Result combinators
fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
	s.parse::<i32>()
}

fn process_number(s: &str) -> Result<String, String> {
	parse_number(s)
    	.map_err(|e| e.to_string()) // Converts ParseIntError to String
    	.and_then(|n| Ok(format!("Number is: {}", n))) // Process number on success
}

// Usage
match process_number("42") {
	Ok(msg) => println!("{}", msg),
	Err(e) => println!("Error: {}", e),
}



// Transpose
fn might_fail(i: i32) -> Result<i32, &'static str> {
	if i % 2 == 0 {
    	Ok(i)
	} else {
    	Err("Odd number")
	}
}

fn optional_operation(flag: bool) -> Option<Result<i32, &'static str>> {
	if flag {
    	Some(might_fail(2))
	} else {
    	None
	}
}

// Usage
let result: Result<Option<i32>, &'static str> = optional_operation(true).transpose();
println!("{:?}", result); // Ok(Some(2))