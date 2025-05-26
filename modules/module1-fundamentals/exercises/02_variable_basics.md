# Exercise 2: Variable Basics

## Problem Statement

Write a Rust program that:

1. Declares variables of different types (integer, float, boolean, and character)
2. Demonstrates mutability by modifying some variables
3. Uses type annotations and type inference
4. Performs basic arithmetic operations between variables
5. Displays the values and results using `println!`

## Learning Objectives

- Practice declaring variables in Rust
- Understand Rust's type system and how to annotate types
- Learn about mutability and the difference between `let` and `let mut`
- Get comfortable with basic operations on different data types

## Starter Code

```rust
fn main() {
    // TODO: 1. Declare an immutable integer variable

    // TODO: 2. Declare a mutable float variable and modify it later

    // TODO: 3. Declare a boolean variable using type inference

    // TODO: 4. Declare a character variable with explicit type annotation

    // TODO: 5. Perform arithmetic operations with the numeric variables

    // TODO: 6. Print all variables and calculation results with appropriate labels
}
```

## How to Run Your Code

1. First, modify the starter code in `02_variable_basics_starter.rs` according to the requirements
2. Run your code from the bootcamp root directory with:
   ```
   cargo run --bin module1_02
   ```

## Expected Output

Your output should show the values of all your variables and calculation results with clear labels, like:

```
Integer value: 42
Original float value: 3.14
Modified float value: 6.28
Boolean value: true
Character value: R
Addition result: 45.14
Multiplication result: 131.88
```

## Tips

- Remember that by default, variables in Rust are immutable
- To make a variable mutable, use the `let mut` syntax
- You can explicitly specify types using the syntax: `let variable_name: type = value;`
- Or let Rust infer the type: `let variable_name = value;`
- Use proper formatting with `println!` to make your output readable
