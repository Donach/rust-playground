#[tokio::main]
async fn main() {
    println!("Tokio executor is running!");
}


#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Tokio executor is running!");
}


use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(future: async {
        println!("Tokio executor is running!");
    });
}


// Tokio tasks
#[tokio::main]
async fn main() {
    let task = tokio::spawn(async {
        println!("Tokio executor is running!");
    });

    //task.await.unwrap(); // no need with tokio
}

// Sleep/wait
use tokio::time::{sleep, timeout, Duration};

#[tokio::main]
async fn main() {

    sleep(Duration::from_secs(2)).await;
    timeout(Duration::from_secs(3), async { println!("timeout is over");}).await;
    println!("Done waiting");
}

