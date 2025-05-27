// TODO: 1. Define a function that adds two integers and returns the result
fn add(a: i32, b: i32) -> i32 {
    a + b  // In Rust, the last expression is implicitly returned
}

// TODO: 2. Define a function that calculates the area of a rectangle
fn calculate_rectangle_area(width: f64, height: f64) -> f64 {
    width * height
}

// TODO: 3. Define a function that checks if a number is prime
fn is_prime(number: u32) -> bool {
    if number <= 1 {
        return false;
    }
    
    for i in 2..=(number as f64).sqrt() as u32 {
        if number % i == 0 {
            return false;
        }
    }
    true
}

// TODO: 4. Define a function that converts Fahrenheit to Celsius
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn main() {
    // TODO: Call the addition function with different values and print the results
    let sum1 = add(5, 3);
    let sum2 = add(10, 20);
    
    // TODO: Calculate and print the area of rectangles with different dimensions
    let area1 = calculate_rectangle_area(4.0, 3.0);
    let area2 = calculate_rectangle_area(10.0, 5.0);
    
    // TODO: Test your prime number checker with several numbers
    let prime_check1 = is_prime(17);
    let prime_check2 = is_prime(4);
    
    // TODO: Convert and print some temperatures from Fahrenheit to Celsius
    let celsius1 = fahrenheit_to_celsius(98.6);
    let celsius2 = fahrenheit_to_celsius(32.0);
    
    // TODO: Print all results with appropriate labels
    println!("Sum of 5 and 3 is: {}", sum1);
    println!("Sum of 10 and 20 is: {}", sum2);
    println!("Area of rectangle with width 4.0 and height 3.0 is: {} square units", area1);
    println!("Area of rectangle with width 10.0 and height 5.0 is: {} square units", area2);
    println!("Is 17 a prime number? {}", prime_check1);
    println!("Is 4 a prime number? {}", prime_check2);
    println!("98.6°F is equivalent to {}°C", celsius1);
    println!("32.0°F is equivalent to {}°C", celsius2);
}