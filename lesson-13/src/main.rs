

fn main() {
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
