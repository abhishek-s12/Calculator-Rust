use std::io;

fn main() {

    println!("Welcome to the Rust Calculator!");
    println!("------------------------");

    println!("Rust Calculator");

    println!("------------------------");

    println!("1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");
    println!("5. Exit");

    let mut choice = String::new();
        io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    println!("You selected: {}", choice.trim());

}