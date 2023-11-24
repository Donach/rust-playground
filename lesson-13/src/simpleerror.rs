
/*
Fallibility in Rust

Provided by Option<T> and Result<T, E>
    - If you want a generic type with two outcomes - use Either<L, R>
Error types should implement the Error trait
 */



 // Simple Error
 use std::error::Error;
 use std::fmt;
 
 // Define a custom error type.
 #[derive(Debug)]
 struct MyError {
     message: String,
 }
 
 // Implement the Display trait for MyError.
 impl fmt::Display for MyError {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "MyError: {}", self.message)
     }
 }
 
 // Implement the Error trait for MyError.
 impl Error for MyError {}
 
 
 
 
 // Simple Error - Borrowed T
 // Error type with a borrowed string slice.
 #[derive(Debug)]
 struct MyBorrowedError<'a> {
     message: &'a str,
 }
 
 // Implement Display and Error for MyBorrowedError.
 impl<'a> fmt::Display for MyBorrowedError<'a> {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "MyBorrowedError: {}", self.message)
     }
 }
 
 impl<'a> Error for MyBorrowedError<'a> {}
 
 
 
 
 
 
 // Simple Error - Source and Backtrace
 use std::error::Error;
 use std::fmt;
 use std::io;
 
 #[derive(Debug)]
 struct MyError {
     message: String,
     inner_error: Option<io::Error>,
 }
 
 impl fmt::Display for MyError {
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
             write!(f, "MyError: {}", self.message)
     }
 }
 
 impl Error for MyError {
     fn source(&self) -> Option<&(dyn Error + 'static)> {
             self.inner_error.as_ref().map(|e| e as &dyn Error)
     }
 
     fn backtrace(&self) -> Option<&std::backtrace::Backtrace> {
             // Logic to return a backtrace if available
     }
 }
 
 
 
 // Simple Error - Capturing Backtrace
 fn my_function() -> Result<(), MyError> {
     let result = std::fs::read_to_string("some_file.txt")
         .map_err(|e| MyError {
             message: "Failed to read file".to_string(),
             inner_error: Some(e),
            // we need to extend MyError so that it can contain backtrace
             backtrace: Backtrace::capture(), // Capture the backtrace here
             backtrace: Backtrace::force_capture(), // ALWAYS Capture the backtrace here
         })?;
 
     println!("File content: {}", result);
 
     Ok(())
 }
 
 // RUST_LIB_BACKTRACE to disable capturing if set to 0
 // RUST_BACKTRACE enable capturing
 
 // use Backtrace::force_capture to ignore env vars
 
 
 