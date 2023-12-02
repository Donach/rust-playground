
use async_stream::stream;
use futures::stream::{self, StreamExt};
use futures::executor::block_on;

async fn run() {
	let nums = stream::iter(vec![1, 2, 3, 4, 5]);
	let nums2 = stream! {
        for i in 1..=5 {
            // Simulate aync work with a simple delay
            yield i; // sender.send(i).await;
        }
    };

	nums.for_each(|num| {
    	println!("Got: {}", num);
    	async {}
	})
	.await;

    nums2.for_each(|num| {
    	println!("Got2: {}", num);
    	async {}
	})
	.await;
}

fn main() {
	block_on(run());
}
