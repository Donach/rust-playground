use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
	// Bind the server to a local address
	let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
	println!("Server running on 127.0.0.1:8080");

	loop {
    	// Accept incoming connections
    	let (mut socket, addr) = match listener.accept().await {
        	Ok((socket, addr)) => (socket, addr),
        	Err(e) => {
            	eprintln!("Failed to accept connection: {}", e);
            	continue;
        	}
    	};
    	println!("New connection from {}", addr);

    	// Spawn a new task for each connection
    	tokio::spawn(async move {
        	let mut buffer = [0; 1024];

        	// Read data from the socket
        	loop {
            	match socket.read(&mut buffer).await {
                	Ok(0) => {
                    	// Connection was closed
                    	println!("Connection closed by {}", addr);
                    	return;
                	}
                	Ok(n) => {
                    	// Echo the data back to the client
                    	if let Err(e) = socket.write_all(&buffer[..n]).await {
                        	eprintln!("Failed to write to socket: {}", e);
                        	return;
                    	}
                	}
                	Err(e) => {
                    	eprintln!("Failed to read from socket: {}", e);
                    	return;
                	}
            	}
        	}
    	});
	}
}