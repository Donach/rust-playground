use tokio::select;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
	let future1 = sleep(Duration::from_secs(5));
	let future2 = sleep(Duration::from_secs(10));

	select! {
    	_ = future1 => println!("Future 1 completed first"),
    	_ = future2 => println!("Future 2 completed first"),
	}
}


// Example demonstrating conditional operation based on external factors
// using `select!`.
use tokio::select;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
	let (tx, mut rx) = mpsc::channel(32);
	let timeout_duration = Duration::from_secs(5);

	// Simulate an external event sending a message
	tokio::spawn(async move {
    	sleep(Duration::from_secs(2)).await;
    	tx.send("Message from external event").await.unwrap();
	});

	select! {
    	Some(message) = rx.recv() => {
        	// Handle the message received from the channel
        	println!("Received message: {}", message);
    	}
    	_ = sleep(timeout_duration) => {
        	// Handle timeout
        	println!(
"No message received within {} seconds; operation timed out.",
timeout_duration.as_secs()
);
    	}
	}
}