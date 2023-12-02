/*
trait Hello {
    async fn greet(); // fn greet() -> imple Future<Output=()>  // -> problem!
}
*/

// ->

use async_trait::async_trait;
#[async_trait]
pub trait AsyncGreet {
    async fn greet(&self, name: &str) -> String;
}

use std::pin::Pin;


// --> Gets rewritten to:
/*
pub trait AsyncGreet {
	fn greet<'life0, 'async_trait>(
    	&'life0 self,
    	name: &'life0 str
	) -> Pin<Box<dyn Future<Output = String> + Send + 'async_trait>>
	where
    	Self: 'async_trait,
    	'life0: 'async_trait;
}
 */

pub struct Greeter;

#[async_trait]
impl AsyncGreet for Greeter {
    async fn greet(&self, name: &str) -> String {
        format!("Hello, {}!", name)
    }
}


fn main() {
    
}