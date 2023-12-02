
use std::future::Future;
use futures::executor::block_on;

struct MyStruct<Fut>
where
    Fut: Future<Output = i32>,
{
    future: Fut,
}

impl<Fut> MyStruct<Fut>
where
    Fut: Future<Output = i32>,
{
    fn new(future: Fut) -> MyStruct<Fut> {
        MyStruct { future }
    }

    async fn run(self) -> i32 {
        self.future.await
    }
}

fn main() {
    block_on(async {
        let my_s = MyStruct::new(async { println!("Hello from async"); 21 });
        my_s.run().await;
    })
}



// Desugaring


async fn basic_async_function() {
    // Your async code here
    println!("This is an async function.");
}

// ==
use std::future::Future;

fn basic_async_function() -> impl Future<Output=()> {
	// Your async code here
	async {
        println!("This is an async function.");
    }
}
// == 
use std::future::Future;
use std::task::{Context, Poll};
use std::pin::Pin;

struct MyFuture;

impl Future for MyFuture {
	type Output = ();

	fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
    	println!("This is an async function.");
    	Poll::Ready(())
	}
}
