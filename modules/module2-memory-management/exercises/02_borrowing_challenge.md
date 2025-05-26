# Exercise 2: Borrowing Challenge

## Problem Statement

Write a Rust program that demonstrates proper use of borrowing and references by:

1. Creating a function that takes an immutable reference to a string and returns its length
2. Creating a function that takes a mutable reference to a vector and adds elements to it
3. Creating a function that processes multiple references to different data structures
4. Fixing code that violates Rust's borrowing rules

## Learning Objectives

- Understand the difference between mutable and immutable references
- Learn how to use references to avoid unnecessary copying of data
- Practice working with Rust's borrowing rules
- Identify and fix common borrowing issues

## Starter Code

```rust
// 1. Processing string data with immutable references
fn get_length(/* TODO: Add parameter for an immutable reference to a String */) -> usize {
    // TODO: Return the length of the string
    0 // Replace this placeholder
}

// 2. Modifying vector data with mutable references
fn add_three_elements(/* TODO: Add parameter for a mutable reference to a Vec<i32> */) {
    // TODO: Add three elements (10, 20, and 30) to the vector
}

// 3. Processing multiple data structures
fn calculate_stats(/* TODO: Add parameters for references to needed data structures */) -> (f64, i32) {
    // TODO: Calculate and return the average of the numbers vector and the count of items in the strings vector
    (0.0, 0) // Replace this placeholder
}

// 4. Borrowing rules demonstration
fn fix_borrowing_issues() {
    let mut data = vec![1, 2, 3];

    // TODO: The following code has borrowing issues. Uncomment and fix it.
    // let ref1 = &mut data;
    // let ref2 = &mut data;
    // ref1.push(4);
    // ref2.push(5);
    // println!("Modified data: {:?}", data);

    // TODO: Fix another example of borrowing issue
    // let ref3 = &data;
    // let ref4 = &mut data;
    // println!("Data length: {}", ref3.len());
    // ref4.push(6);
}

fn main() {
    // 1. Test immutable reference function
    let test_string = String::from("Hello, Rust borrowing!");
    let length = get_length(/* TODO: Pass the string as an immutable reference */);
    println!("String length: {}", length);
    // Verify the string is still usable after passing as reference
    println!("Original string: {}", test_string);

    // 2. Test mutable reference function
    let mut my_vec = Vec::new();
    println!("Before function call: {:?}", my_vec);
    add_three_elements(/* TODO: Pass the vector as a mutable reference */);
    println!("After function call: {:?}", my_vec);

    // 3. Test multiple references
    let numbers = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    let words = vec![String::from("apple"), String::from("banana"), String::from("cherry")];
    let (average, count) = calculate_stats(/* TODO: Pass appropriate references */);
    println!("Average of numbers: {:.1}, Count of strings: {}", average, count);

    // 4. Test the fixed borrowing issues
    fix_borrowing_issues();
}
```

## How to Run Your Code

1. First, modify the starter code in `02_borrowing_challenge_starter.rs` according to the requirements
2. Run your code from the bootcamp root directory with:
   ```
   cargo run --bin module2_02
   ```

## Expected Output

```
String length: 22
Original string: Hello, Rust borrowing!
Before function call: []
After function call: [10, 20, 30]
Average of numbers: 30.0, Count of strings: 3
Modified data: [1, 2, 3, 4, 5]
Data length: 5
Modified data: [1, 2, 3, 4, 5, 6]
```

## Tips

- Remember that Rust allows multiple immutable references OR one mutable reference at a time
- Use `&` for immutable references and `&mut` for mutable references
- Pass references to functions using `&variable` or `&mut variable`
- Think about the lifetime of references - they must not outlive the data they reference
- For the borrowing issues section, consider scope and non-overlapping use of references
