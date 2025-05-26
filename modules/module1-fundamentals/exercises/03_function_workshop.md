# Exercise 3: Function Workshop

## Problem Statement

Write a Rust program that:

1. Defines and calls a function that takes two integers and returns their sum
2. Creates a function that calculates the area of a rectangle (width × height)
3. Implements a function that checks if a number is prime
4. Writes a function that converts temperatures between Fahrenheit and Celsius
5. Uses all these functions in the `main` function and displays the results

## Learning Objectives

- Practice defining and calling functions in Rust
- Learn about function parameters and return types
- Understand different ways to return values from functions
- Apply control flow within functions

## Starter Code

```rust
// TODO: 1. Define a function that adds two integers and returns the result

// TODO: 2. Define a function that calculates the area of a rectangle

// TODO: 3. Define a function that checks if a number is prime

// TODO: 4. Define a function that converts Fahrenheit to Celsius
// Formula: C = (F - 32) * 5/9

fn main() {
    // TODO: Call the addition function with different values and print the results

    // TODO: Calculate and print the area of rectangles with different dimensions

    // TODO: Test your prime number checker with several numbers

    // TODO: Convert and print some temperatures from Fahrenheit to Celsius
}
```

## How to Run Your Code

1. First, modify the starter code in `03_function_workshop_starter.rs` according to the requirements
2. Run your code from the bootcamp root directory with:
   ```
   cargo run --bin module1_03
   ```

## Expected Output

Your output should show the results of calling all your functions with different inputs:

```
Sum of 10 and 25 is: 35
Area of rectangle with width 5 and height 10 is: 50 square units
Is 7 a prime number? true
Is 12 a prime number? false
98.6°F is equivalent to 37.0°C
32.0°F is equivalent to 0.0°C
```

## Tips

- Use explicit return type annotations: `fn function_name(parameter: type) -> return_type`
- The last expression in a function becomes the return value (no semicolon)
- You can also use the `return` keyword explicitly
- For the prime number checker, a number is prime if it's greater than 1 and only divisible by 1 and itself
- Use appropriate control flow (loops, conditionals) in your prime number function
- Format your outputs clearly using the `println!` macro
