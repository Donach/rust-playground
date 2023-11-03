use std::io;

fn main() {
    println!("Enter your name:");

    let mut name: String = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read your name!");

    let name: &str = name.trim();
    println!("Hello, {}!", name);
}
