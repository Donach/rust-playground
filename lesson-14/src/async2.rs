use futures::executor::block_on;

async fn fetch_data() -> String {
    "Data".to_string()
}

async fn process_data() -> String {
    let data = fetch_data().await;
    data.to_uppercase()
}
fn main() {
    let future = process_data(); // Returns a future
    let result = block_on(async {
        process_data().await
    });

    println!("{}", result);
}