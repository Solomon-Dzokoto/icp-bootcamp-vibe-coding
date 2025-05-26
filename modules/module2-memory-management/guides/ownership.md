# Ownership in Rust

## Ownership Rules

1. Each value has a single owner
2. Only one owner at a time
3. When owner goes out of scope, value is dropped

```rust
fn main() {
    {
        let s = String::from("hello"); // s is valid here
        // Operations with s
    } // s is no longer valid - automatically dropped
}
```

## Move Semantics

When you assign a variable to another variable, the ownership moves:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid

    // println!("{}", s1); // Error: value borrowed after move
    println!("{}", s2); // Works fine
}
```

This also applies with function calls:

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s); // s's value moves into the function
    // s is no longer valid here

    let x = 5;
    makes_copy(x); // i32 datatype implements Copy trait, so x is still valid after
    println!("{}", x); // Works fine
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope and is dropped

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer goes out of scope, nothing special happens
```

## Getting Ownership Back

Functions can also return ownership:

```rust
fn main() {
    let s1 = gives_ownership();         // takes ownership from function

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved, s3 takes ownership

    // s2 is no longer valid here, but s1 and s3 are
}

fn gives_ownership() -> String {
    let s = String::from("hello");
    s // returned and ownership moves to calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // returned and ownership moves to calling function
}
```

## Clone and Copy Traits

### Clone

Make a deep copy of heap data:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // Deep copy, both s1 and s2 are valid

    println!("s1 = {}, s2 = {}", s1, s2); // Works fine
}
```

### Copy

Types with Copy trait are duplicated rather than moved:

```rust
fn main() {
    let x = 5;
    let y = x; // x is copied, not moved

    println!("x = {}, y = {}", x, y); // Both valid
}
```

Types that implement Copy:

- All integer types (u32, i32, etc.)
- Boolean type (bool)
- Floating point types (f32, f64)
- Character type (char)
- Tuples that only contain Copy types, e.g., (i32, i32)
