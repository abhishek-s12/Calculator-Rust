use std::{io::{self, Write}};

fn main() {

    println!("Welcome to the Rust Calculator!");

    println!("------------------------");
    println!("    Rust Calculator");
    println!("------------------------");

    println!("1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");
    println!("5. Exit");

    print!("Please enter your choice (1-5): ");
    io::stdout().flush().unwrap();
    
    let mut choice = String::new();
  
    
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");



    let choice= choice  
        .trim()
        .parse::<u32>()
        .expect("Invalid input");

        if choice == 5 {
            println!("Goodbye!");
            return;
        }
        
    
    
    

    match choice {
        1 => {
            let a = read_number("Enter first number: ");
            let b = read_number("Enter second number: ");
            println!("{} + {} = {}", a, b, add(a, b));
        }
        2 => {
            let a = read_number("Enter first number: ");
            let b = read_number("Enter second number: ");
            println!("{} - {} = {}", a, b, subtract(a, b));
        }           
            
        3 => {
            let a = read_number("Enter first number: ");
            let b = read_number("Enter second number: ");
            println!("{} * {} =     {}", a, b, multiply(a, b));
        }
        4 => {
            let a = read_number("Enter first number: ");
            let b = read_number("Enter second number: ");
             println!("{} / {} = {}", a, b, divide(a, b));
        }
        
        _ => {
            println!("Invalid choice! Please select a valid option.");
        }

    }



   
}

fn add (a: f64, b: f64) -> f64 {
        a + b
    } 
    fn subtract(a: f64, b: f64) -> f64 {
        a - b
    }
    fn multiply(a: f64, b: f64) -> f64 {
        a * b
    }
    fn divide(a: f64, b: f64) -> f64 {
        if b == 0.0 {
            println!("Error: Division by zero is not allowed.");
            std::f64::NAN
        } else {
            a / b
        }
    }

// fn read_numers() -> f64

fn read_number(prompt: &str) -> f64 {
    print!("{}", prompt);
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().parse::<f64>().expect("Invalid number")

}