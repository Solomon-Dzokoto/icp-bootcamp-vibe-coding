# Control flow in rust

### If expressions

In Rust, `if` expressions are used to branch code depending on conditions. The basic syntax is:

```rust
fn main() {
    if condition {
        // code block for satisfied condition
    } else {
        // code block for unsatisfied condition
    }
}
```

### Using if in a let statement

You can also use `if` expressions on the right side of a `let` statement to assign the outcome to a variable:

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}
```

### Repetition with loops

Rust provides three types of loops: `loop`, `while`, and `for`. Let's explore each:

#### Repeating code with loop

The `loop` keyword tells Rust to execute a block of code repeatedly until you explicitly tell it to stop.

```rust
fn main() {
    loop {
        println!("again!");
    }
}
```

#### Conditional loops with while

A `while` loop runs as long as the condition is true:

```rust
fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

#### Looping through a collection with for

You can use `for` to loop over the elements of a collection, such as an array:

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}
```

This is more concise and safer than using a `while` loop with an index:

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    // Using for with a range and indexing
    for i in 0..a.len() {
        println!("the value is: {}", a[i]);
    }
}
```
