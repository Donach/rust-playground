use futures::stream::{self, StreamExt};
use futures::executor::block_on;

async fn run() {
	let nums = stream::iter(vec![1, 2, 3, 4, 5]);

	nums.filter(|&num| async move { num % 2 == 0 })
    	.for_each(|num| {
        	println!("Even number: {}", num);
        	async {}
    	})
    	.await;
}

fn main() {
	block_on(run());
}