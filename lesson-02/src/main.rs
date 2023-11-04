use slug::slugify; 
use std::env;
use std::io::stdin;

fn main() {
    let args: Vec<String> = env::args().collect();


    // Get the user input.
    let mut input = String::new();
    println!("Enter text to trasmute: ");
    stdin().read_line(&mut input).unwrap();

    for arg in args {
        println!("Found args: {}", arg);
        if arg == "lowercase" {
            input = input.to_lowercase();
        } else if arg == "uppercase" {
            input = input.to_uppercase();
        } else if arg == "no-spaces" {
            input = input.replace(" ", "");
        } else if arg == "slugify" {
            input = slugify(input);
        }
    }
    println!("Transmuted text: {}", input);

}