// Tokio has its own channels

use tokio::sync::mpsc; // you will encounter these a lot, but consider using flume instead
// Cannot easily wait for synchronous function -> flume can

// broadcast -> many senders, many receivers
// watch -> one sender, many receivers
// oneshot - single message (not async)

// tokio Mutex -> much safer but slower; Parking Lot - faster (not async though)

#[tokio::main]
async fn main() {
	let (tx, mut rx) = mpsc::channel(32);

	tokio::spawn(async move {
    	tx.send("hello").await.unwrap();
	});

	while let Some(message) = rx.recv().await {
    	println!("Received: {}", message);
	}
}