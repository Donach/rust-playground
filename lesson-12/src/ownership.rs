fn main() {
    println!("Hello, world!");
    let a = i32_as_str(&10);
}


// Ownership
/*
Tool for memory management in Rust
Motivation
- Garbage collection is useful and a great tool for safety
- not always desirable -> performance cost
- manual memory management is hard to do unchecked
    - 70% of reported CVEs are due to memory safety issues
    - Eg. the Heartbleed exploit would not happen in Rust
        - OpenSSL buffer over-read
- We need to not have to worry about things going away too soon
    - GC does that
    - So do ownership (lifetimes and borrowing)*/
// Example:
fn i32_as_str(number: &i32) -> &str {
    let s = format!("{}", number);
    &s
    // We are trying to return reference to something that exists
    // only in i32_as_str() = bad
    // Technical term would be "returning a dangling pointer" and doing a "use after free"
}

/*
The example is caught in Rust, but not necessarily in C
- who knows what happens with freed memory in C?
    - Might get segfault immediately
    - data might just happen to randomly stay valie
    - data might become corrupted
- There are many other examples of unsound code not caught in C
- in Rust, the approach is the other way around
    - It is "your task to prove to the compilter that your code is sound"
        - You do this by writing code that Rust will accept
        - Following borrowing rules
        - Writing out lifetime bounds where ambiguity exists*/
// Another bad example
fn data_push() {
    // vecs are backed by a heap allocation
    // if  oyu know the realloc() C function ,you know it might move data
    let mut data = vec![1, 2, 3];
    // Get internal reference
    let x = &data[0];

    // 'push' causes the backing storage of 'data' to be reallocated
    // Dangling pointer! Use after free!
    // (will not compile in Rust)
    data.push(4);

    data[0] = 12;

    println!("x = {}", x); // 1 not 12
}

/*
- This example will be caught in Rust
    - Who knows what happens when we enlarge the vector and then try to work with &data[0]?
        - vector is just resized - Everything might work perfectly
        - The vector does not fit into its original place and is moved by the resize
            a) everything seems to work, but the pointer to "&data[0] is outdated"
            b) immediate segfault
        - Data might become corrupted
- Naive scope analysis not enough -> we need to worry about mutable and immutable reference separately
- Rust requires references to freeze the referent and its owners
 */