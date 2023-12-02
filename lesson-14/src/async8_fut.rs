use futures::sink::SinkExt;
use futures::stream::{self, StreamExt};
use futures::executor::block_on;

async fn sink_to_vec() {
    let data  = vec![1, 2, 3, 4, 5];
    let stream = stream::iter(data);

    let mut sink = Vec::new();

    stream.map(Ok).forward(&mut sink).await.unwrap();

    println!("Sink: {:?}", sink);
}

fn main() {
    block_on(sink_to_vec());
}