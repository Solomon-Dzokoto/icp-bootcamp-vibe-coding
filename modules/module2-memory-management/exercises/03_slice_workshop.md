# Exercise 3: Slice Workshop

## Problem Statement

Write a Rust program that demonstrates effective use of slices by:

1. Creating a function that takes a string slice and returns the first word
2. Creating a function that takes an array slice and returns the sum of its elements
3. Implementing a function that finds the middle element(s) of a slice
4. Creating a function that extracts a specific part of a slice based on a condition

## Learning Objectives

- Understand what slices are and how they differ from full collections
- Learn to use string slices effectively
- Practice working with array and vector slices
- Understand the relationship between slices and references

## Starter Code

```rust
// 1. Find the first word in a string
fn first_word(s: /* TODO: Add parameter type for a string slice */) -> /* TODO: Return type */ {
    // TODO: Return the first word in the string (up to the first space or the entire string if no spaces)
    ""
}

// 2. Calculate the sum of elements in an array slice
fn sum_slice(numbers: /* TODO: Add parameter type for a slice of integers */) -> i32 {
    // TODO: Calculate and return the sum of all elements in the slice
    0
}

// 3. Find the middle element(s) of a slice
fn middle_elements(slice: /* TODO: Add parameter type for a generic slice */) -> /* TODO: Return type */ {
    // TODO: Return the middle element if length is odd, or the two middle elements if length is even
    // Hint: For a generic implementation, you'll need to handle both cases
    if slice.len() % 2 == 1 {
        // Odd length - return a slice containing just the middle element
    } else {
        // Even length - return a slice containing the two middle elements
    }

    &slice[0..0] // Placeholder empty slice - replace this
}

// 4. Extract a subslice based on a condition (e.g., all positive numbers)
fn extract_positive(numbers: /* TODO: Add parameter type for a slice of integers */) -> /* TODO: Return type */ {
    // TODO: Find the first continuous run of positive numbers in the slice and return it as a slice
    // If the slice starts with a positive number, return from start until the first non-positive
    // If the slice starts with a non-positive, find the first positive and return from there until the next non-positive
    // If no positives are found, return an empty slice

    &numbers[0..0] // Placeholder empty slice - replace this
}

fn main() {
    // 1. Test first_word function
    let sentence = String::from("Hello Rust slices world");
    let first = first_word(/* TODO: Pass the string as a slice */);
    println!("First word: {}", first);

    let empty_str = String::from("");
    let first_empty = first_word(/* TODO: Pass the empty string as a slice */);
    println!("First word in empty string: '{}'", first_empty);

    // 2. Test sum_slice function
    let numbers = [1, 2, 3, 4, 5];
    let sum = sum_slice(/* TODO: Pass the array as a slice */);
    println!("Sum of all elements: {}", sum);

    let partial_sum = sum_slice(/* TODO: Pass a slice of part of the array */);
    println!("Sum of first three elements: {}", partial_sum);

    // 3. Test middle_elements function
    let vec1 = vec![1, 2, 3, 4, 5]; // Odd length
    let mid1 = middle_elements(/* TODO: Pass the vector as a slice */);
    println!("Middle element(s) of odd-length vector: {:?}", mid1);

    let vec2 = vec![1, 2, 3, 4]; // Even length
    let mid2 = middle_elements(/* TODO: Pass the vector as a slice */);
    println!("Middle element(s) of even-length vector: {:?}", mid2);

    // 4. Test extract_positive function
    let mixed_numbers = [3, 5, 2, -1, -5, 8, 10, -3];
    let positive_run = extract_positive(/* TODO: Pass the array as a slice */);
    println!("First run of positive numbers: {:?}", positive_run);

    let negative_start = [-2, -5, 3, 4, 5, -1, 7];
    let positive_run2 = extract_positive(/* TODO: Pass the array as a slice */);
    println!("First run of positive numbers (starting negative): {:?}", positive_run2);
}
```

## How to Run Your Code

1. First, modify the starter code in `03_slice_workshop_starter.rs` according to the requirements
2. Run your code from the bootcamp root directory with:
   ```
   cargo run --bin module2_03
   ```

## Expected Output

```
First word: Hello
First word in empty string: ''
Sum of all elements: 15
Sum of first three elements: 6
Middle element(s) of odd-length vector: [3]
Middle element(s) of even-length vector: [2, 3]
First run of positive numbers: [3, 5, 2]
First run of positive numbers (starting negative): [3, 4, 5]
```

## Tips

- String slices are written as `&str`
- Array/vector slices are written as `&[T]` where T is the element type
- Use indexing with ranges to create slices: `&s[0..5]`
- Remember that string indices must be at valid UTF-8 character boundaries
- For the first_word function, use `.bytes()` and `.enumerate()` to find a space character
- For generic slices, you can use `&[T]` where T is a generic type parameter
- Use `.iter()` to iterate over slice elements, or index directly with `slice[i]`
