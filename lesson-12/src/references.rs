fn main() {

}
// References
/* 
- two kinds of a reference
    - Shared (read-only) -> & 
    - mutable -> &mut
    
- Two rules for working with them
    1. Reference cannot outlive its referent
    2. Mutable referenced cannot be 'aliased'

References - Aliasing
Def:
    - Variables and pointers 'alias' if they refer to overlapping regions of memory
    - eg. Two &borrows of the same variable 'are aliasing'
    - borrow of a vector and one of its elements 'are aliasing'
- Sometimes, aliasing is okay, it is, however, a loaded gun, when mutable data is involved
*/

// Why aliasing matters - optimizations


// note that compute takes two pointers/borrows
fn compute(input: &u32, output: &mut u32) {
    if *input > 10 {
        *output = 1;
    }
    if *input > 5 {
        *output *=2;
    }
    // remember that 'output' will be '2' if 'input > 10'
}

// traversing pointers costs CPU time, so we would like to avoid it

// how we want the compile to optimize compute()
fn compute2(input: &u32, output: &mut u32) {
    let cached_input = *input; // keep '*input' in a CPU register - very fast!!!
    if cached_input > 10 {
        /* If input is greater than 10, the previous code would set 
         * output to 1, and then double it, resulting in an output of 2
         * Here, we avoid the double assignmet and just set it directly to 2
         */
        *output = 2;
    }
    else if cached_input > 5 {
        *output *=2;
    }
    // remember that 'output' will be '2' if 'input > 10'
}

/*
Besides the previous optimization, here are some general cases
    - Keeping values in registers by proving no pointers access its memory
    - Eliminating reads by proving some memory hasn't been written to since last read
    - Eliminating writed by proving some memory is never read before the next, write to it
    - Moving or reordering reads and writes by proving they do not depend on each other
The above optimizations also serve as gateway optimizations for some other optimizations 
- lopp vectorizations
- constant propagation
- dead code eliminations*/
