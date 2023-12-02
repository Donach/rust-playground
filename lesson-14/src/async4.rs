
use std::pin::Pin;
use std::marker::PhantomPinned;

struct MyStruct {
    data: i32,
}


impl MyStruct {
    fn new(data: i32) -> MyStruct {
        MyStruct { data }
    }
    fn display(&self) {
        println!("Data {}", self.data);
    }
}

struct MyNonUnpinStruct {
    data: i32,
    _marker: PhantomPinned, // This fields prevents the struct from being Unpin
}

impl MyNonUnpinStruct {
    fn new(data: i32) -> Pin<Box<MyNonUnpinStruct>> {
        let my_struct = MyNonUnpinStruct { data, _marker: PhantomPinned };
        Box::pin(my_struct)
    }
    fn display(&self) {
        println!("Data {}", self.data);
    }
}


fn main() {
    let my_struct = MyStruct::new(21);
    let pinned = Pin::new(&my_struct);
    pinned.display();

    // Unpin
    let my_struct = MyNonUnpinStruct::new(21);
    println!("Data: {}", my_struct.data);
    //let moved_strucrt = Pin::into_inner(my_struct); -> this won't compile!

}

