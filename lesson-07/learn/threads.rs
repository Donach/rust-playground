/*
std::threads module
Real OS threads
can be managed
communicate via statics (not recomm) or channels
Threads can panic independently
*/


use std::thread;
use std::panic::panic_any;
fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from spawned thread!");
    });

    handle.join().unwrap();

    println!("Hello from main thread!");

    // Moving data into closure
    let message = "Hello";
    let message_copy: &str = message.clone();
    let handle = thread::spawn(move || {
        println!("Got message: {}", message_copy);
        21 // Return value to result below
    });
    

    let result = handle.join().unwrap();
    println!("Result: {}", result);

    // Panics
    let handle = thread::spawn(|| {
        panic!("Thread panic!");
    });

    match handle.join() {
        Ok(_) => println!("Thread finished successfully"),
        Err(err) => println!("Thread panicked with message: {:?}", err.downcast::<&str>().unwrap()),
    }

}



use std::thread;
// Multiple Threads
fn main() {
    let mut handles = vec![];

    for i in 0..5 {
        let handle = thread::spawn(move || {
            println!("Thread {} started", i);
            i
        });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.join().unwrap();
        println!("Thread {} finished ", result);
    }

    main3();
    main4();
}

// Builder 
fn main2() {
    let t = thread::Builder::new().name("Thread-1".to_string()).spawn(move || {
        println!("Thread-1 started");
    });
}

// Thread local storage
thread_local!{
    static COUNTER: std::cell::RefCell<i32> = std::cell::RefCell::new(0);
}

fn main3() {
    // INcrement the counter in the main thread
    COUNTER.with(|c| {
        *c.borrow_mut() += 1;
        println!("Main thread: {}", c.borrow());
    });

    //Spawn a new thread and increment the counter in that thread
    let handle = thread::spawn(move || {
        COUNTER.with(|c| {
            *c.borrow_mut() += 2;
            println!("Spawned thread: {}", c.borrow());
        });
    });

    handle.join().unwrap();

    // Check counter in main thread
    COUNTER.with(|c| {
        println!("Main thread: {}", c.borrow());
    });
}


// Channel
use std::sync::mpsc; // mpsc = multiple producer, single consumer
// Flume - mpmc channel
// crossbeam-channel - highly configurable (used in Tokio)

fn main4() {
    //let (tx, rx) = mpsc::channel();
    let (tx, rx) = flume::unbounded();

    thread::spawn(move || {
        let message = "Hello from spawned thread!";
        tx.send(message).unwrap();
        println!("Message sent.");
    });

    let received_message = rx.recv().unwrap();
    println!("Received: {}", received_message);

}




// Send and Sync
/*
Marker traits
Implemented automatically for all types satisfying conditions
Tracks thread-safety
What is thread-unsafe:
 - raw pointers
 - Interior mutability without atomic counters
 - Locks (Mutex)
Send: &T is threadsafe
 - Ownership of a value can be transferred between threads
Sync: T -> can be sent to another thread
 - A value of this type can be safely shared between threads immutably
Implementing it manually is unsafe
Unimplementing it manually is safe
Given type T is Sync if (and only if) &T is Send



 */ 

 use std::thread;
 use std::ptr;
 
 #[derive(Debug)]
 struct MyType(*mut i32);
 
 unsafe impl Send for MyType{}
 
 fn main() {
     let (tx, rx) = flume::unbounded();
 
     thread::spawn(move || {
         tx.send(MyType(ptr::null::<i32> as *mut i32)).unwrap();
         println!("Message sent.")
     });
 
 
 
 }
 
 
 #[feature(negative_impls)]
 // I have some magic semantics for some synchronization primitive
 struct SpecialThreadToken(u8);
 
 impl !Send for SpecialThreadToken{}
 impl !Sync for SpecialThreadToken{}
 
 
 
 
 /*
 Send and Sync
 opt-out in safe 
 Allows for custom type to inherit Send/Sync trait without wasting memory*/
 use std::marker::PhantomData;
 
 type PhantomUnsync = PhantomData<*mut ()>;
 
 struct UnsyncType {
     something: String,
     __phantom: PhantomUnsync, // size = 0
 }