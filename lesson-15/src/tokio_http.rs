// This example shows how to create a simple HTTP client using Tokio and the hyper crate.

use hyper::{Body, Client, Request};
use hyper::rt::Future;
use hyper::http::Result;

#[tokio::main]
async fn main() -> Result<()> {
	let client = Client::new();
    
	let req = Request::builder()
    	.uri("http://httpbin.org/ip")
    	.body(Body::empty())?;
    
	let res = client.request(req).await?;
    
	println!("Response: {}", res.status());
	Ok(())
}