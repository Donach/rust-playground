
fn main4 () {

}

/*
fn duplucate<T: Copy>(item:t) -> [T; 2] {
    [item, item]
}
// T, U, V, W ... A B
// K, V - hashmap
struct Point<T, U> {
    x: T,
    y: U,
}
impl<T> Point<T> where T: Clone {

}

impl<T> Point<T> where T: Animal {

}

fn main() {
    let int_point = Point {x: 5, y: 10};
    let flt_point = Point {x: 1.0, y: 10.5};
    let flt_point = Point {x: 1, y: 10.5};
}

fn merge<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

enum Option<T> {
    Some(T), 
    None,
}

fn compare<T, U>(first: T, second: U)
where
    T: PartialOrd + Copy,
    U: PartialOrd + Copy,
    {
        if first < second {
            println!("first is less than second");
        } else {
            println!("first is not less than second");
        }
    }


struct Array<T, const N: usize> {
    elements: [T; N],
}

fn arr() {
    let integers: Array<i32, 5> = Array { elements: [1, 2, 3, 4, 5] };
    let floats: Array<f64, 5> = Array { elements: [1.0, 2.0, 3.0, 4.0, 5.0] };
}
 */
// Generics + Lifetimes
struct RefHolder<'a, T> {
    ref_to_t: &'a T,
}
/*
fn lifetimes() -> RefHolder<'a, String>{
    let string1 = String::from("Rust");
    let result;
    {
        let string2 = String::from("C++");
        let holder = RefHolder { ref_to_t: &string1 };

        result = holder.ref_to_t;
        // 'result' lives as long as 'string1'
        holder // not possible
    }
}
 */

// Traits
/*
Backbone of rust generics
define shared behaviour
Similar to interfaces in other langs
Opposite of C++ templates */

trait MyTrait {
    type Error;
    fn my_function();

    fn methods(&self) -> Result<bool, Self::Error>;
}

struct MyType;

impl MyType2 {
    fn my_function() {
        
    }
}

// MyType: !MyTrait
// std::ops::Add example

use std::ops::{Add, Deref};

struct MyType2;

impl Add<u32> for MyType2 {
    type Output = f32;

    fn add(self, rhs: u32) -> Self::Output {
        todo!()
    }
}

trait Describable {
    const SOME_CONST: usize = 10usize;
    type MyType;
    fn describe(&self) -> String; // Has to be always implemented

    fn get_const() -> usize {
        Self::SOME_CONST
    }

}

struct Person {
    name: String,
    age: u8,
}

impl Describable for Person {
    fn describe(&self) -> String {
        format!("{} is {}", self.name, self.age)
    }

    type MyType = Person;
}
/* 
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    fn count(&mut self) -> usize {
        let mut length = 0;

        while let Some(_) = self.next() {
            length += 1;
        }
        length
    }
} */
/* 
fn some_func() -> impl MyTrait { // Type will be derived from fn body (Existential type)

}
*/
fn my_func() -> impl Fn() -> i32 {
    || 12 
}

// Static dispatch
fn print_static(item: &impl Describable) {
    println!("{}", item.describe())
}

// compiler does Monomophization
// print will be copied for each different type T used

// More flexible
/*
fn print_static<T: Describable>(item: &T) {
    println!("{}", item.describe())
}

// Traits - dynamic dispatch
fn print_dynamic(item: &dyn Describable) {
    println!("{}", item.describe())
} */
// &dyn Describable is called a trait object
// has to be behind a pointer
// compiler builds a vtable with methods, chich is used at runtime - no monomorphization
// slower, but smaller binaries, and more flexible
// can mix and match different concrete types - suitable for heterogeneous collections

// not all traits can be made into objects


// Crate "num" for conversions

// Traits - Object safte
/*
Only object-safe traits can be made into trait objects
Trait cannot require Self: Sized
- but methods can! and should if you need to do something not object safe
Methods must take self parameter
Methods cannot have generic paarms (w/ static dispatch) */
/*
trait ObjectUnsafe {
    fn new_instance(&self) -> Self
    where
        Self: Sized
    {

    }
}

fn try_trait_object(x: &dyn ObjectUnsafe) {
    x.new_instance();
}
 */

// Object-safe
trait Speak {
    fn say_hello(&self);
}

struct Human;

impl Speak for Human {
    fn say_hello(&self) {
        println!("Hello");
    }
}
fn _test_human() {
    let human = Box::new(Human);
    human.say_hello();
}

// Not object-safe
trait Compute {
    fn compute<T>(&self, data: T); // not object-safe; method has a generic parameter
}

struct Calculator;

impl Compute for Calculator {
    fn compute<T>(&self, data: T) {
        todo!()
    }
}

/*
this will not compile

fn main() {
    let computer: Box<dyn Compute> = Box::new(Calculator);
    computer.compute(21); // Error: trait 'Compute' is not object-safe
} */




// Supertraits
/*
traits can depends on other traits
enforce trait relationships
the trait that depends on other trait is called a subtrait, the trait(s) it depends on is/are the supertraits(s)
supertraits can be abused to create something like OOP inheritance
 - discouraged */

// ToString is a supertrait of Display
trait Display: ToString {
   fn display(&self){
        println!("{}", self.to_string());
   }
}

// dyn Dsiplay -> dyn ToString
// T: Display -> T: ToString

struct Product {
   id: u32,
   name: String,
}

impl ToString for Product {
    fn to_string(&self) -> String {
        format!("{} - {}", self.id, self.name)
    }
}

impl Display for Product {
    fn display(&self) {
        println!("{}", self.to_string());
    }
}

fn main2() {
    let product = Product { id: 1, name: "Apple".to_string() };
    product.display();
}

// Higher-Ranked Trait bounds
/*
sometimes we need to be able to cover a lot of lifetimes in far too many combinations
- typically generics over closures
also sometimes used with iterators */

fn apply_to_all<'a, F>(f: F, items: &[&'a str])
where
    F: Fn(&'a str) -> &'a str
{
    for &item in items {
        f(item);
    }
}

fn main3() {
    let echo = |x: &str| println!("{}", x);
    // apply_to_all(echo, &["Helloworld", "Hello", "World"]);
}

/*
Marker traits
- no methods
- Send, Sync, Copy, Sized 

Conversions
- TryFrom / From, FromStr
- TryInto / Into
    - From/TryFrom is reflexive and implies Into/TryInto
    - impl<T> Into<U> for T where U: From<T> 
    
Operators
- Add, Sub, Mul,...

Display, Debug, Clone

Comparison and equality
- PartialEq, PartialOrd
- Eq, Ord
    - Marker traits signifying order or equality is precise
Pointer/Borrow conversions
 - AsRef, AsMut

Dereference helpers
 - Deref, DerefMut

Borrow, BorrowMut, ToOwned
- semantical trait that T can be borrowed U canonically
- -> Eq, Ord, Hash must match

// T: From<U> 
// ---> U: Into<T>*/

struct StringContainer(String);

impl From<String> for StringContainer {
    fn from(value: String) -> Self {
        Self(value)
    }
}
// my_sc.0.len()

impl Deref for StringContainer {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// --> my_sc.len() 
// let sc: StringContainer = StringContainer::from(String::new());
// let sc: StringContainer = String::new().into(); // Automatically avail





// Fibonacci
// We can implement an iterator, that generates the fibonacci sequence
// Iterator trait

struct Fib {
    current: u64,
    next: u64,
}

impl Fib {
    fn new() -> Self {
        Fib { current: 0, next: 1 }
    }
}

impl Iterator for Fib {
    type Item = u64;

    // Iterator method
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.current.checked_add(self.next)?;
        self.current = self.next;
        self.next = new_next;

        Some(self.current)
    }
}

fn main6() {
    let fib = Fib::new();
    for num in fib.take(10) {
        println!("{}", num);
    }
}

// Improving it to be generic

use num_traits::{Zero, One, CheckedAdd};

struct Fib2<T> {
    current: T,
    next: T,
}

impl<T> Fib2<T>
where
    T: Zero + One
{
    fn new() -> Self {
        Fib2 { current: Zero::zero(), next: One::one() }
    }
}

impl<T> Iterator for Fib2<T>
where
    T: CheckedAdd + Clone
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.current.checked_add(&self.next)?;
        let new_current: T = self.next.clone();
        self.current = new_current;
        self.next = new_next;
        Some(self.current.clone())
    }
}

fn main7() {
    let fib = Fib2::<u64>::new();
    for num in fib.take(10) {
        println!("{}", num);
    }
}


// Heterogenous Collections
/*
imagine we wnat to write something vaguely graphics-adjacent
must store everything that is drawable in one collection
 - Trait objects
 - Downcasting */

trait Drawable{
    fn draw(&self);
}

struct Circle {
    x: i32,
    y: i32,
    radius: i32,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Circle: x: {}, y: {}, radius: {}", self.x, self.y, self.radius);
    }
}

struct Square {
    x: i32,
    y: i32,
    size: i32,
}

impl Drawable for Square {
    fn draw(&self) {
        println!("Square: x: {}, y: {}, size: {}", self.x, self.y, self.size);
    }
}

fn main8() {
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { x: 0, y: 0, radius: 5 }),
        Box::new(Square { x: 0, y: 0, size: 5 }),
    ];

    for shape in shapes {
        shape.draw();
    }
}


// Downcasting
use std::any::Any;

fn main8() {
    let shapes: Vec<Box<dyn Any>> = vec![
        Box::new(Circle { x: 0, y: 0, radius: 5 }),
        Box::new(Square { x: 0, y: 0, size: 5 }),
    ];

    for shape in shapes {
        if let Some(circle) = shape.downcast_ref::<Circle>() {
            println!("Circle with radius {}", circle.radius);
        } else if let Some(square) = shape.downcast_ref::<Square>() {
            println!("Square with size {}", square.size);
        }
    }
}


// impl dyn Trait
trait Drawable2{
    fn draw(&self);
}

// additional methods for the trait object
impl dyn Drawable2 {
    fn describe(&self) {
        println!("This is a drawable2 object.");
    }
}

impl Drawable2 for Circle {
    fn draw(&self) {
        println!("Circle: x: {}, y: {}, radius: {}", self.x, self.y, self.radius);
    }
}



// Opting out of Sized implicind bound
use std::fmt::Debug;

fn print_info<T: Debug + ?Sized>(value: &T) {
    println!("{:?}", value);
}

#[derive(Debug)]
struct Person2 {
    name: String,
    age: u8,
}

fn main() {
    let person = Person2 { name: "John".to_string(), age: 30 };
    let person_ref = &person;
    print_info(person_ref); // Works with '&Person' which is Sized

    // Now with a trait object, which is not Sized.
    let debuggable: &dyn std::fmt::Debug = person_ref;
    print_info(debuggable); // Works with '&dyn std::fmt::Debug' which is not Sized

}