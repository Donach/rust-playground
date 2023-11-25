

fn main() {
}


use anyhow::{Result, Context};

fn read_file(file_path: &str) -> Result<String> {
	std::fs::read_to_string(file_path)
    	.with_context(|| format!("Failed to read file at {}", file_path))
}

fn main() -> Result<()> {
	let content = read_file("test.txt")?;
	println!("File content: {}", content);
	Ok(())
}



use anyhow::{Result, anyhow};

fn task1() -> Result<()> {
	Err(anyhow!("Task 1 failed"))
}

fn task2() -> Result<()> {
	task1().with_context(|| "Task 2 failed while executing task 1")
}

fn main() {
	match task2() {
    	Ok(_) => println!("Success"),
    	Err(err) => {
        	eprintln!("Error: {:?}", err);
        	for cause in err.chain() {
            	eprintln!("Caused by: {:?}", cause);
        	}
    	}
	}
}


use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataProcessingError {
	#[error("Data not found: {0}")]
	NotFound(String),
	#[error("Invalid data format")]
	InvalidFormat,
	#[error("IO error")]
	Io(#[from] std::io::Error),
}

fn process_data(file_path: &str) -> Result<(), DataProcessingError> {
	if file_path.is_empty() {
    	return Err(DataProcessingError::NotFound(file_path.to_string()));
	}

	let data = std::fs::read_to_string(file_path)?;
	if data.is_empty() {
    	return Err(DataProcessingError::InvalidFormat);
	}

	println!("Data processed: {}", data);
	Ok(())
}
