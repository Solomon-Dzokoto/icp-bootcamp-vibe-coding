// --- Function Definitions ---

// 1. Define a function `greet` that takes a name (string slice) and prints a greeting.
fn greet(name: &str) {
    println!("Hello, {}! Welcome to the function workshop.", name);
}

// 2. Define a function `add` that takes two integers and returns their sum.
fn add(a: i32, b: i32) -> i32 {
    // The last expression in a function without a semicolon is implicitly returned.
    a + b
}

// 3. Define a function `multiply_and_log` that takes two floats,
//    multiplies them, prints the result, and returns nothing (implicitly void).
fn multiply_and_log(x: f32, y: f32) {
    let result = x * y;
    println!("The result of multiplying {} by {} is: {}", x, y, result);
    // This function does not return a value (void return type).
}

// --- Main Function: Calling the defined functions ---
fn main() {
    // Call the `greet` function.
    println!("--- Calling greet function ---");
    greet("Alice");
    greet("Bob");

    println!("\n--- Calling add function ---");
    // Call the `add` function and store its result.
    let sum1 = add(10, 5);
    let sum2 = add(-5, 8);
    println!("Sum of 10 and 5 is: {}", sum1);
    println!("Sum of -5 and 8 is: {}", sum2);

    println!("\n--- Calling multiply_and_log function ---");
    // Call the `multiply_and_log` function.
    multiply_and_log(3.5, 2.0);
    multiply_and_log(7.0, 7.0);
}