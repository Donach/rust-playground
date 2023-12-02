use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
	let mut file = tokio::fs::File::open("some_file.txt").await.unwrap();
	let mut contents = Vec::new();
	file.read_to_end(&mut contents).await.unwrap();
	// Other async operations can run while waiting for the file to be read.
}


use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
	let mut file = tokio::fs::File::create("output.txt").await.unwrap();
	file.write_all(b"Hello, world!").await.unwrap();
	// The file write operation is non-blocking.
}