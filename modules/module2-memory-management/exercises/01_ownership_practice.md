# Exercise 1: Ownership Practice

## Problem Statement

Identify and fix the ownership issues in the following code samples. The goal is to make all examples compile and run correctly without changing their intended functionality.

## Learning Objectives

- Identify common ownership problems
- Apply appropriate solutions (cloning, borrowing, etc.)
- Understand the implications of move semantics

## Starter Code

```rust
// Example 1: String ownership
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1); // Error: s1 has been moved
}

// Example 2: Function ownership
fn main() {
    let s = String::from("hello");
    takes_ownership(s);

    println!("After function call: {}", s); // Error: s has been moved
}

fn takes_ownership(some_string: String) {
    println!("Inside function: {}", some_string);
}

// Example 3: Vector ownership
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    for i in v {
        println!("{}", i);
    }

    // Calculate and print the sum of elements in v
    let sum: i32 = v.iter().sum(); // Error: v has been moved in the for loop
    println!("Sum: {}", sum);
}
```

## How to Run Your Code

1. First, modify the starter code in `01_ownership_practice_starter.rs` to fix the ownership issues
2. Run your code from the bootcamp root directory with:
   ```
   cargo run --bin module2_01
   ```

## Expected Output

After fixing the ownership issues, your code should produce the following outputs:

Example 1:

```
hello, world!
```

Example 2:

```
Inside function: hello
After function call: hello
```

Example 3:

```
1
2
3
4
5
Sum: 15
```

## Tips

- Consider when to use `clone()` to duplicate data
- Look for opportunities to use references (`&`) instead of moving values
- Remember that `for` loops take ownership of collections by default unless you iterate over references
