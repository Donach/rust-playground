// Lifetimes - subtyping and variance

/*
A naive implementation of lifetimes would be either too restrictive, or permit undefined behavior
    - We need to able to safely convert one lifetime to another,  where it is valid
To allow flexible usage of lifetimes, but preventing misuses, Rust utilizes:
    - Subtyping
    - Variance

Generic definition: Using one type (A) in place of another (B)
    - Such that A is a subtype of B
We have seen it with traits
    - I can use Trait1, such that Trait1: Trait2, in places that expect Trait2
We can consider the same with lifetimes
    - Let’s define a lifetime such that ‘a defines a region of code
    - Lifetime ‘long is a subtype of ‘short such iff ‘long defines a region of code that completely contains ‘short
        - It is fine if ‘long is even larger than ‘short from either side
Subtyping is in places where we need to shorten a lifetime in the eyes of a function


 */

fn debug<'a>(a: &'a str, b: &'a str) {
    println!("a = {a:?} b = {b:?}");
}

fn main() {
	let hello: &'static str = "hello";
	{
    	let world = String::from("world");
    	let world = &world; // 'world has a shorter lifetime than 'static
    	debug(hello, world); // hello silently downgrades from `&'static str` into `&'world str`
	}
}

/*
In the previous example, we have kind of omitted the fact that ‘static being a subtype of ‘b also implies that &’static T is subtype of &’b T
Variance is the transitiveness of subtyping
    - Three options, generic definition
        1. Covariant - if type T is subtype of U, then F<T> is subtype of F<U>
        2. Contravariant - if type T is a subtype of U, then F<U> is a subtype of <T>
        3. Invariant - no relation can be derived */


// Bad example
fn assign<T>(input: &mut T, val: T) {
	*input = val;
}

fn main2() {
	let mut hello: &'static str = "hello";
	{
    	let world = String::from("world");
    	assign(&mut hello, &world);
	}
	println!("{hello}"); // use after free :(((
}

/*
The previous code cannot be valid
    - We are setting the hello reference to point to world
    - However, world goes out of scope
    - In the println!() hello would be invalid after the assign() call
The problem is that we cannot assume that &mut &’static str and &mut &’b str are compatible
    - Even if ‘static is subtype of ‘b
    - This means that &mut &’static str cannot be a subtype of &mut &’b str
        - Invariant
    - But we could do &’a T as subtype of &’b T if ‘a is subtype of ‘b
        - Covariant */