// asynchronous file operations using Tokio.

use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
	let mut file = File::create("foo.txt").await?;
	file.write_all(b"Hello, world!").await?;
    
	let mut file = File::open("foo.txt").await?;
	let mut contents = vec![];
	file.read_to_end(&mut contents).await?;
    
	println!("File contents: {:?}", String::from_utf8(contents).unwrap());
	Ok(())
}