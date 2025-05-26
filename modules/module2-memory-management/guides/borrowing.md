# Borrowing in Rust

## References and Borrowing

References allow you to access data without taking ownership:

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // & creates a reference

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // s goes out of scope, but doesn't drop what it refers to
```

## Mutable References

References are immutable by default. Use `&mut` for mutable references:

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s); // Prints "hello, world"
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

## Reference Rules

1. At any time, you can have either:

   - One mutable reference, OR
   - Any number of immutable references

2. References must always be valid

### Multiple Immutable References

```rust
fn main() {
    let s = String::from("hello");

    let r1 = &s; // No problem
    let r2 = &s; // No problem

    println!("{} and {}", r1, r2);
}
```

### Restriction on Mutable References

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // No problem
    let r2 = &s; // No problem
    // let r3 = &mut s; // ERROR - cannot borrow as mutable when already borrowed as immutable

    println!("{} and {}", r1, r2);

    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // OK - r1 and r2 are no longer used
    println!("{}", r3);
}
```

### Preventing Dangling References

Rust ensures references don't outlive the data they refer to:

```rust
fn main() {
    // let reference_to_nothing = dangle(); // Error!
}

fn dangle() -> &String { // Returns a reference to a String
    let s = String::from("hello"); // s is created inside
    &s // We return a reference to s
} // s goes out of scope and is dropped, reference would be invalid!
```

Correct approach:

```rust
fn main() {
    let string = no_dangle();
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s // Return the String directly, ownership moves
}
```

## Reference Lifetimes

Rust tracks how long references are valid:

```rust
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
```
