use std::io;

fn main() {
    // Part 1: FizzBuzz Implementation
    // This section implements the FizzBuzz challenge.
    // It iterates from 1 to 20 (inclusive).
    // For multiples of 3, it prints "Fizz".
    // For multiples of 5, it prints "Buzz".
    // For multiples of both 3 and 5, it prints "FizzBuzz".
    // Otherwise, it prints the number.
    println!("=== FizzBuzz Challenge ===");
    
    for i in 1..=20 { // Loop from 1 through 20
        if i % 15 == 0 { // Check for divisibility by 15 (both 3 and 5)
            println!("FizzBuzz");
        } else if i % 3 == 0 { // Check for divisibility by 3
            println!("Fizz");
        } else if i % 5 == 0 { // Check for divisibility by 5
            println!("Buzz");
        } else { // If none of the above, print the number
            println!("{}", i);
        }
    }
    
    // Part 2: Menu-driven Calculator
    // This section implements a simple calculator that presents a menu to the user.
    // The user can choose an operation, input two numbers, and see the result.
    // The calculator continues to run until the user chooses to exit.
    println!("\n=== Calculator ===");
    
    let mut running = true; // Flag to control the main loop of the calculator
    
    while running {
        // Display the menu options to the user
        println!("\nCalculator Menu:");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Exit");
        print!("Enter your choice (1-5): "); // Using print! to keep input on the same line

        // Flush stdout to ensure the prompt is shown before input is read
        io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout.");

        // Read the user's menu choice
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read choice from stdin.");
        
        let choice = choice.trim(); // Remove leading/trailing whitespace

        // Process the user's choice
        match choice {
            "1" | "2" | "3" | "4" => { // Valid operations
                // Get the first number
                print!("Enter first number: ");
                io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout.");
                let mut num1_str = String::new();
                io::stdin()
                    .read_line(&mut num1_str)
                    .expect("Failed to read first number.");

                let num1: f64 = match num1_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input for the first number. Please enter a valid number.");
                        continue; // Skip to the next iteration of the loop
                    }
                };

                // Get the second number
                print!("Enter second number: ");
                io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout.");
                let mut num2_str = String::new();
                io::stdin()
                    .read_line(&mut num2_str)
                    .expect("Failed to read second number.");

                let num2: f64 = match num2_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input for the second number. Please enter a valid number.");
                        continue; // Skip to the next iteration of the loop
                    }
                };

                // Perform the calculation based on the choice
                match choice {
                    "1" => println!("Result: {} + {} = {}", num1, num2, num1 + num2),
                    "2" => println!("Result: {} - {} = {}", num1, num2, num1 - num2),
                    "3" => println!("Result: {} * {} = {}", num1, num2, num1 * num2),
                    "4" => {
                        if num2 == 0.0 {
                            println!("Error: Cannot divide by zero.");
                        } else {
                            println!("Result: {} / {} = {}", num1, num2, num1 / num2);
                        }
                    }
                    _ => unreachable!(), // Should not happen due to outer match
                }
            }
            "5" => { // Exit option
                println!("Exiting calculator. Goodbye!");
                running = false; // Set running to false to terminate the loop
                continue; // End this iteration
            }
            _ => { // Invalid menu choice
                println!("Invalid choice! Please enter a number between 1 and 5.");
                // No need to ask for continuation here, the loop will naturally restart
                continue;
            }
        }

        // If the user didn't choose to exit, ask if they want to continue
        if running {
            print!("Do you want to perform another calculation? (y/n): ");
            io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout.");
            let mut continue_choice = String::new();
            io::stdin()
                .read_line(&mut continue_choice)
                .expect("Failed to read continue choice.");

            if continue_choice.trim().to_lowercase() != "y" {
                println!("Exiting calculator. Goodbye!");
                running = false; // Set running to false to exit the loop
            }
        }
    }
}