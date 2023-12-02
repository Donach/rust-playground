async fn basic_async_fn () {
    println!("basic_async_fn");

    let my_future = async { println!("Hello from the future.");};
    my_future.await;
}

fn main() {
    let future = basic_async_fn(); // Returns a future

}