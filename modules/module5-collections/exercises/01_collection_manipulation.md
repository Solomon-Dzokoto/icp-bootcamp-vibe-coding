# Exercise 1: Collection Manipulation

## Problem Statement

Implement a program that demonstrates effective use of Rust's common collection types (vectors, strings, and hash maps) to process and analyze text data.

## Learning Objectives

- Practice working with vectors, strings, and hash maps
- Understand how to efficiently manipulate and transform collections
- Apply iterators and collection methods to solve problems
- Handle string data properly in Rust

## Starter Code

```rust
use std::collections::HashMap;

fn main() {
    // Sample text for analysis
    let text = "Rust is a multi-paradigm, general-purpose programming language. \
                Rust emphasizes performance, type safety, and concurrency. \
                Rust enforces memory safety—that is, that all references point \
                to valid memory—without requiring the use of a garbage collector \
                or reference counting present in other memory-safe languages.";

    // TODO: 1. Split the text into words and store them in a vector
    let words = vec![];

    // TODO: 2. Count the frequency of each word and store in a HashMap
    let word_counts = HashMap::new();

    // TODO: 3. Find the longest word in the text
    let longest_word = "";

    // TODO: 4. Convert all words to uppercase and store in a new vector
    let uppercase_words = vec![];

    // TODO: 5. Filter out words shorter than 4 characters
    let filtered_words = vec![];

    // Print results
    println!("Word counts: {:?}", word_counts);
    println!("Longest word: {}", longest_word);
    println!("Uppercase words: {:?}", uppercase_words);
    println!("Words longer than 3 characters: {:?}", filtered_words);
}
```

## How to Run Your Code

1. First, modify the starter code in `01_library_system_starter.rs` according to the requirements
2. Run your code from the bootcamp root directory with:
   ```
   cargo run --bin module3_01
   ```

## Expected Output

Your output should be similar to (word counts might vary depending on how you split words):

```
Word counts: {"Rust": 3, "a": 2, "memory-safe": 1, "performance": 1, ...}
Longest word: "multi-paradigm,"
Uppercase words: ["RUST", "IS", "A", "MULTI-PARADIGM,", ...]
Words longer than 3 characters: ["Rust", "multi-paradigm", "general-purpose", ...]
```

## Tips

- Use string methods like `split_whitespace()` or `split()` to separate words
- Remember to handle punctuation appropriately
- Consider using `to_string()` or `String::from()` to convert string slices to owned strings
- Utilize collection methods like `iter()`, `map()`, `filter()`, and `collect()`
- Use the `entry()` API for efficient insertion into hash maps
