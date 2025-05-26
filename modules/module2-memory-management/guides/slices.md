# Slices in Rust

Slices are references to a contiguous sequence of elements in a collection.

## String Slices

String slices (&str) are references to parts of a String:

```rust
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];  // "hello"
    let world = &s[6..11]; // "world"

    // Alternative syntax
    let hello = &s[..5];   // "hello", start from 0
    let world = &s[6..];   // "world", go to the end
    let entire = &s[..];   // entire string slice

    println!("{} {}", hello, world);
}
```

## Finding Words with Slices

```rust
fn main() {
    let s = String::from("hello world");

    let first_word = first_word(&s);

    println!("First word: {}", first_word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

## String Literals as Slices

String literals are already slices:

```rust
fn main() {
    let s = "Hello, world!"; // s is &str, not String

    let first = first_word(s); // Works because s is &str

    println!("First word: {}", first);
}
```

## Function Parameters as Slices

Best practice to use &str instead of &String when possible:

```rust
fn first_word(s: &str) -> &str {
    // Implementation
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    // Works with both String and &str
    let my_string = String::from("hello world");
    let word1 = first_word(&my_string[..]);

    let my_literal = "hello world";
    let word2 = first_word(my_literal);

    println!("{} {}", word1, word2);
}
```

## Array Slices

Slices work with collections types such as vectors or arrays too:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    println!("{:?}", slice); // [2, 3]

    // Use slices in functions
    print_slice(slice);
}

fn print_slice(slice: &[i32]) {
    for element in slice {
        println!("{}", element);
    }
}
```
