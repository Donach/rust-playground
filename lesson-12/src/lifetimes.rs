fn main() {
    hello(String::new());
}

// Lifetimes
/*
- Rust programmer's daily bread
- ownership ruels are enforce through lifteimes
Definition:
- a lifetime is a named region of code that a reference must be valid for
    - regions correspond to paths of execution in the program
- Types that contain references (or pretend to) may also be tagged with a lifetime
    - We have seen these when looking at generics
- Mostly, lifetimes coincide with 'scopes'
Syntax:
- Apostrophe + name -> 'a, 'b, 'c typically
- 'static has special meaning -> valid for program runtime
In function bodies, we cannot actually label scopes with lifetimes
 - we can only refer to generic lifetimes on function parameters, struct members,...
    */

fn hello<T>(_: T) 
where T: 'static{

}



/*
fn pseudo() {
    let x = 0;
    let y = &x;
    let z = &y;
    // NOTE: 'a: { is not valid syntax!
    'a: {
        let x = 0;
        'b: {
            // lifetime used is 'b because that's good enough
            let y: &'b i32 = &'b x;
            'c: {
                // ditto on 'c
                let z: &'c &'b i32 = &'c y;
                // 'a reference to a reference to an i32
                // (with lifetimes annotated)
            }
        }
    }
}
*/

fn main2() {
    let x = 0;
    let z;
    let y = &x;
    // y should die before z if lifetime would not be larger
    z = y;

    /*pseudo:
    'a: {
        let x = 0;
        'b: {
            let z: &'b i32;
            'c: {
                // Must use 'b here because the reference to x is  being passed to the scope 'b
                let y: &'b i32 = &'b x;
                z = y;
            }
        }
    } */
}

/*
fn bad_as_str<'a>(data: &'a u32) -> &'a str {
    'b: {
        let s = format!("{}", data); // s was defined in scope 'b
        return &'a s; // so 'b: 'a must hold -> impossible!
    }
} */

/*vec bad example
fn bad_vector() {
    'a: {
        let mut data: Vec<i32> = vec![1, 2, 3];
        'b: {
            // 'b is as big as we need this borrow to be
            // (just need to get to `println!`)
            let x: &'b i32 = Index::index::<'b>(&'b data, 0);
            'c: {
                // Temporary scope because we don't need the
                // &mut to last any longer.
                Vec::push(&'c mut data, 4);
            }
            println!("{}", x);
        }
    }
} */

/*
We want Rust to reject previous example
    - We have an existing reference 'x' to a descendant of data
        - -> 'aliased' mutable references
        - Violates the second rule of borrowing
That is not however, how Rust sees the situations
    - It sees that 'x' hs to live for 'b
    - 'Index' demands that 'data' has to survive for 'b
    - When we try to call '.push()' we try to make &'c mut data
        - Rust sees that 'c is inside 'b
        - And rejects the code because &'b data must therefore still be alive
        
A reference (aka a borrow) is alive from the place it was created to its last use
    - A borrowed value only needs to outlive borrows that are alive
Not so simple due to non-lexical lifetimes
    - Historically, Rust kept borrows alive until the end of scope - these were called
    'lexical lifetimes', since you could discover them from the syntax alone*/

/*
let mut data = vec![1, 2, 3];
let x = &data[0];
println!("{}", x);
// This is OK, x is no longer needed
data.push(4);
 */

/* this example is valid, because 'x' is no longer needed
    - it is not 'used'
However, a similar example will never work if the value has a destructor
    - That means 'T: Drop'
    - The reason is taht a destructor is run at the end of scope
        - Running a destructor is considered a use because it has access to the data
        - And obviously, it will always be the last use*/

// This will not compile
/*
#[derive(Debug)]
struct X<'a>(&'a i32);

impl Drop for X<'_> {
	fn drop(&mut self) {}
}
fn wontwork() {
    let mut data = vec![1, 2, 3];
    let x = X(&data[0]);
    println!("{:?}", x);
    data.push(4);
}
 */
// Here, the destructor is run and therefore this'll fail to compile.
// The only way to convince the compiler that ‘x’ is no longer valid is to drop it explicitly with mem::drop()



// Lifetime can have pause
fn pause() {
    let mut data = vec![1, 2, 3];
    // This mut allows us to change where the reference points to
    let mut x = &data[0];

    println!("{}", x); // Last use of this borrow
    data.push(4); // this is valid because we will no longer refer to &data[0] via x
    x = &data[3]; // We start a new borrow here
    println!("{}", x);
}


// Lifetime elision
/*
For the sake of ergonomics, lifetimes can be elidede in function signatures
    - his is just one of the possible lifetime positions
    - These are anywhere, where you can write:
        &’a T
        &’a mut T
        T<’a>
Lifetime positions can be either input or output
    - fn definitions, fn types, Fn/FnMut/FnOnce traits, input refers to types of the formal arguments, output to return type
    - Impl headers - all considered input position 

Example:
fn run_fn(x: fn() -> &'a str) {
    let mut s = String::new();
    let x = |x: i32| ();
    let y = || s.clear();
    let z = move || s;
}
    
Rules:
Each elided lifetime in input pos. becomes a distinct lifetime parameter (‘a, ‘b, ‘c)
If there is 'exactly one' input lifetime (elided or not), it is assigned to elided output lifetimes
If there are multiple lifetime positions, but one of them is '&self' or '&mut self', the lifetime of 'self' is assigned to elided output lifetimes
Any other case would an error to elide
-> lifetime annotation required compile error
*/

fn print_two_nums(x: &i32, y: &i32) {
    println!("{} {}", x, y);
}

fn cut_string_short<'a>(input: &'a str, num: usize) -> &'a str {
    &input[num..]
}
 // This has lifetime specifier missing on the fn output
fn cut_string_short2<'a, 'b>(input: &'a str, other_input: &str) -> & str {
    &input
}

// Examples
fn print(s: &str);                                  	// elided
fn print<'a>(s: &'a str);                           	// expanded

fn debug(lvl: usize, s: &str);                      	// elided
fn debug<'a>(lvl: usize, s: &'a str);               	// expanded

fn substr(s: &str, until: usize) -> &str;           	// elided
fn substr<'a>(s: &'a str, until: usize) -> &'a str; 	// expanded

fn get_str() -> &str;                               	// ILLEGAL

fn frob(s: &str, t: &str) -> &str;                  	// ILLEGAL

fn get_mut(&mut self) -> &mut T;                    	// elided
fn get_mut<'a>(&'a mut self) -> &'a mut T;          	// expanded

fn args<T: ToCStr>(&mut self, args: &[T]) -> &mut Command ;             	// elided
fn args<'a, 'b, T: ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command; // expanded

fn new(buf: &mut [u8]) -> BufWriter;                	// elided
fn new(buf: &mut [u8]) -> BufWriter<'_>;            	// elided
                                            // ↑ (with `rust_2018_idioms`)
fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a>;      	// expanded



// Unbounded lifetime
/*
Unsafe code can magically conjure lifetimes or references out completely out of thin air
    - These are called unbounded lifetimes
    - (not bounded to any owned value)
An unbounded lifetime becomes as big as the context demands
For most cases, we can think of an unbounded lifetime to be as ‘static
    - But it can typecheck in some cases where ‘static won’t
Origins:
    - Reborrowing raw pointers (e.g. coming from C code)
    - mem::transmute and mem::transmute_copy()
In other words: Any output lifetimes that doesn’t derive from inputs are unbounded */

fn get_str2<'a>(s: *const String) -> &'a str {
	unsafe { &*s }
}

fn main3() {
	let soon_dropped = String::from("hello");
	let dangling = get_str2(&soon_dropped);
	drop(soon_dropped);
	println!("Invalid str: {}", dangling); // Invalid str: gӚ_`
}


