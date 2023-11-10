fn main() {
    println!("Hello, world!");

    for i in 4..1 {
        println!("{}", i);
    }

    (1..4)
        .map(|i| i * 2)
        .for_each(|i| println!("Number: {}", i));

    let mut fruits = vec!["apple", "banana", "cherry"];

    for fruit in &mut fruits {
        *fruit = "hello";
    }

    println!("fruits: {:?}", fruits);
}
