# Enums in Rust

Enums (enumerations) allow you to define a type by enumerating its possible variants.

## Basic Enums

```rust
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
}

fn route(ip_kind: IpAddrKind) {
    // Code to handle different kinds of IP addresses
}
```

## Option Enum

Rust's standard library provides the `Option` enum for values that might be something or nothing:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Even though it's in the standard library, you can use it without importing:

```rust
fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    // Using Option forces you to handle the None case
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // This won't compile:
    // let sum = x + y;

    // You must handle the Option
    let sum = x + y.unwrap_or(0);
}
```

## Result Enum

Used for operations that might fail:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let file_result = std::fs::File::open("hello.txt");

    match file_result {
        Ok(file) => println!("File opened successfully"),
        Err(error) => println!("Error opening file: {:?}", error),
    }
}
```

## Concise Error Handling

```rust
fn main() {
    // Using if let
    if let Ok(file) = std::fs::File::open("hello.txt") {
        println!("File opened successfully");
    } else {
        println!("Failed to open file");
    }

    // Using unwrap (will panic if Err)
    let file = std::fs::File::open("hello.txt").unwrap();

    // Using expect (will panic with custom message if Err)
    let file = std::fs::File::open("hello.txt")
        .expect("Failed to open hello.txt");

    // Using ? operator in a function that returns Result
    fn read_username_from_file() -> Result<String, std::io::Error> {
        let mut file = std::fs::File::open("hello.txt")?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        Ok(s)
    }
}
```
