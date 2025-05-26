# Exercise 1: Hello, Rust!

## Problem Statement

Write a Rust program that:

1. Asks the user for their name
2. Prints a personalized greeting including their name
3. Prints the current date

## Learning Objectives

- Practice using the `println!` macro
- Learn how to get user input using `std::io`
- Format strings using Rust's formatting capabilities

## Starter Code

```rust
use std::io;

fn main() {
    // TODO: 1. Prompt the user for their name

    // TODO: 2. Read the user's input
    let mut name = String::new();

    // TODO: 3. Print a personalized greeting

    // BONUS: Print the current date
    // Hint: You can use the chrono crate for this
}
```

## How to Run Your Code

1. First, modify the starter code in `01_hello_rust_starter.rs` according to the requirements
2. Run your code from the bootcamp root directory with:
   ```
   cargo run --bin module1_01
   ```

## Expected Output

```
What is your name? Alice
Hello, Alice! Welcome to the Rust Bootcamp!
Today is [current date].
```

## Tips

- Use `io::stdin().read_line(&mut name)` to read user input
- Remember to handle the `Result` returned by `read_line`
- To trim whitespace (like newlines) from the input, use `name.trim()`
- For the bonus, you'll need to add the chrono crate to your dependencies in Cargo.toml
