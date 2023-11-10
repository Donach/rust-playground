
use std::sync::Mutex;
use once_cell::sync::Lazy;

// const fn new(closure) -> Self{}
static COUNTER: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));


fn main() {
    println!("Hello, world!");
    {
        // .lock(&self) where Self: Mutex<i32>
        let mut counter = COUNTER.lock().unwrap();
        *counter += 1;
    }

    main2();
    main4();
}


use std::cell::Cell;
use std::cell::RefCell;

fn main2() {
    let x = Cell::new(42);
    println!("x: {}", x.get());

    x.set(21);
    println!("x changed: {}", x.get());

    let y = RefCell::new(vec![1, 2, 3]);
    {
        let mut y_ref = y.borrow_mut();
        y_ref.push(4);
    }
    {
        let y_ref = y.borrow();
        println!("y: {:?}", y_ref);
    }
}

//use std::sync::Mutex; --> Faster; only one party to access it; each access is mutable
fn main3() {
    let counter = Mutex::new(0); 

    {
        let mut num = counter.lock().unwrap();
        *num += 1;
    }
    println!("counter: {}", *counter.lock().unwrap()); // 1
}

use std::sync::RwLock; // --> Bit slower; More flexible; can choose to read or write
fn main4() {
    let lock = RwLock::new(String::from("Hello, "));

    {
        let read_guard1 = lock.read().unwrap();
        let read_guard2 = lock.read().unwrap();
        println!("read_guard1: {}; read_guard2: {}", read_guard1, read_guard2);
    }

    {
        let mut write_guard = lock.write().unwrap();
        write_guard.push_str("world!");

    }
    {
        let read_guard = lock.read().unwrap();
        println!("after modification: {}", read_guard);
    }
}


// parking_lot --> https://crates.io/crates/parking_lot 

// LAZY INITIALIZATION
// Std: OnceLock; not really used
// Lazy_Static -> most used
// Once-cell - more flexible, no maro
// Static-init - nice syntax (just attribute); can do eager init; claims to be fastest; ; less platform support 
#[macro_use] extern crate lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref FRUITS: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(1, "apple");
        m.insert(2, "banana");
        m
    }
}

static FRUITS: Lazy<HashMap<u32, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(1, "apple");
    m.insert(2, "banana");
    m
});


// Lazy - once_cell
static DATA: Lazy<String> = Lazy::new(|| {
    String::from("Hello, world!")
});
fn main5() {
    println!("{}", *DATA);
}

// Lazy - static_init
use static_init::dynamic;
#[dynamic]
static L1: Vec<i32> = vec![1, 2, 3];

fn main6() {
    println!("{:?}", L1);
}



