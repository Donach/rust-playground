// Smart points

/*
Helpss us survive
Rust has a number in the std lib
    - Box<T> 
    - Rc<T>
    - Arc<T>
    - Ref<T> and RefMut<T>
        - From RefCell
Some others available in libraries


*/


// simple Heap allocation
fn main() {
	let boxed_int: Box<i32> = Box::new(5);
	println!("Boxed integer: {}", boxed_int);
}





// Reference-counted (kinda GC) pointer
use std::rc::Rc;

fn main() {
	let five = Rc::new(5);
	let five_clone = Rc::clone(&five);
    // five.clone(), but only if T: !Clone, T::clone()
	println!("Original: {}, Clone: {}", five, five_clone);
}



// Atomic Rc<T>
// Rc cannot be shared between threads
use std::sync::Arc;

fn main() {
	let num = Arc::new(5);
	let num_clone = Arc::clone(&num);
	println!("Original: {}, Clone: {}", num, num_clone);
}


