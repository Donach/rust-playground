use futures::executor::block_on;
use futures::join;

async fn async_task_1() -> i32 {
	// simulate some async work
	10
}

async fn async_task_2() -> String {
	// simulate some other async work
	"task 2 complete".to_string()
}

fn main() {
	let result = block_on(async {
    	let (result1, result2) = join!(async_task_1(), async_task_2());
    	format!("Results: {}, {}", result1, result2)
	});

	println!("{}", result);
}