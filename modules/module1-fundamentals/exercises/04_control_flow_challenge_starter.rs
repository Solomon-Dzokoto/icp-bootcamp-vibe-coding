use std::io;

fn main() {
    // Part 1: FizzBuzz Implementation
    println!("=== FizzBuzz Challenge ===");
    
    for i in 1..=20 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
    
    // Part 2: Menu-driven Calculator
    println!("\n=== Calculator ===");
    
    let mut running = true;
    
    while running {
        println!("\nCalculator Menu:");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Exit");
        println!("Enter your choice (1-5): ");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read choice");
        
        let choice = choice.trim();

        if choice == "5" {
            println!("Goodbye!");
            running = false;
            continue;
        }

        println!("Enter first number: ");
        let mut num1 = String::new();
        io::stdin()
            .read_line(&mut num1)
            .expect("Failed to read first number");
        
        println!("Enter second number: ");
        let mut num2 = String::new();
        io::stdin()
            .read_line(&mut num2)
            .expect("Failed to read second number");

        let num1: f64 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid first number!");
                continue;
            }
        };

        let num2: f64 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid second number!");
                continue;
            }
        };

        match choice {
            "1" => println!("{} + {} = {}", num1, num2, num1 + num2),
            "2" => println!("{} - {} = {}", num1, num2, num1 - num2),
            "3" => println!("{} * {} = {}", num1, num2, num1 * num2),
            "4" => {
                if num2 == 0.0 {
                    println!("Error: Cannot divide by zero!");
                } else {
                    println!("{} / {} = {}", num1, num2, num1 / num2);
                }
            }
            _ => println!("Invalid choice! Please enter a number between 1 and 5."),
        }
    }
}