# Functions in Rust

Functions are a fundamental part of Rust, allowing you to organize your code into reusable blocks.

## Defining Functions

In Rust, functions are defined using the `fn` keyword, followed by the function name, parameters in parentheses, and the function body enclosed in curly braces:

```rust
fn main() {
    println!("Hello from the main function!");
    another_function();
}

fn another_function() {
    println!("Hello from another function!");
}
```

## Parameters

Functions can take parameters, which are special variables that are part of the function's signature:

```rust
fn main() {
    greet("Alice");
    greet("Bob");
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```

You must declare the type of each parameter:

```rust
fn print_details(name: &str, age: u32) {
    println!("{} is {} years old.", name, age);
}
```

## Return Values

Functions can return values. In Rust, the return type is specified after an arrow (`->`):

```rust
fn main() {
    let x = five();
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}
```

The return value is the value of the final expression in the function body. You can also return early using the `return` keyword:

```rust
fn add_or_subtract(x: i32, y: i32, should_add: bool) -> i32 {
    if should_add {
        return x + y;
    }
    x - y
}
```

## Statements and Expressions

Rust is an expression-based language:

- **Statements** are instructions that perform some action but don't return a value.
- **Expressions** evaluate to a resulting value.

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1  // Note the lack of semicolon - this makes it an expression
    };

    println!("The value of y is: {}", y);
}
```

In the above code, the block `{ let x = 3; x + 1 }` is an expression that returns `4`.

## Functions with Multiple Return Values

Rust doesn't directly support multiple return values, but you can return a tuple:

```rust
fn calculate_statistics(numbers: &[i32]) -> (i32, i32, i32) {
    let sum = numbers.iter().sum();
    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();

    (sum, min, max)
}

fn main() {
    let numbers = [5, 10, 15, 20, 25];
    let (sum, min, max) = calculate_statistics(&numbers);

    println!("Sum: {}, Min: {}, Max: {}", sum, min, max);
}
```

## Function Pointers

Functions can be passed as arguments to other functions:

```rust
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer); // 12
}
```
