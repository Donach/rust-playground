
// Storing a future without static dispatch

use std::future::Future;
use std::pin::Pin;
use futures::executor::block_on;

// Define struct hat holds a boxed future
struct MyStruct {
    future: Pin<Box<dyn Future<Output = i32> + Send>>,
}

impl MyStruct {
    fn new<F>(future: F) -> MyStruct 
    where
        F: Future<Output = i32> + Send + 'static,
    {
        MyStruct {
            future: Box::pin(future),
        }
    }

    async fn run(&mut self) -> i32 {
        self.future.as_mut().await
    }
}

fn main() {
    block_on(MyStruct::new(async { println!("Hello"); 21}).run());
}