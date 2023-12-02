use futures::{select, future, FutureExt};
use futures::executor::block_on;

async fn task_one() -> &'static str {
    // Simulate some work
    "Task one completed"
}

async fn task_two() -> &'static str {
    // Simulate some work
    "Task two completed"
}


fn main() {
    block_on(async {
        let mut future2 = Box::pin(task_two().fuse());
        let mut future1 = Box::pin(task_one().fuse());

        // or select_biased! - will always poll from top of the list
        select! {
            result = future1 => {
                println!("First to finish was {}", result);
            },
            result = future2 => {
                println!("First to finish was {}", result);
            },
        };
    });
}