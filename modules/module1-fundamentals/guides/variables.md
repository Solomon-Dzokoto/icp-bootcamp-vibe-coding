# Variables in Rust

## Variable Declaration

In Rust, variables are immutable by default. You declare a variable using the `let` keyword:

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    // This would cause an error
    // x = 6; // cannot assign twice to immutable variable
}
```

### Mutability

To make a variable mutable, use the `mut` keyword:

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6; // This is valid because x is mutable
    println!("The value of x is now: {}", x);
}
```

### Constants

Constants are values that are bound to a name and cannot change. They're declared using the `const` keyword:

```rust
fn main() {
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum points is: {}", MAX_POINTS);
}
```

Constants differ from immutable variables in that:

- They must be annotated with a type
- They can be declared in any scope, including the global scope
- They can only be set to a constant expression, not the result of a function call or any other value computed at runtime

### Shadowing

You can declare a new variable with the same name as a previous variable, effectively "shadowing" the previous variable:

```rust
fn main() {
    let x = 5;

    let x = x + 1; // This creates a new variable x that shadows the old one

    {
        let x = x * 2; // This shadows x within this scope
        println!("The value of x in the inner scope is: {}", x); // 12
    }

    println!("The value of x is: {}", x); // 6
}
```

Shadowing allows you to change a variable's type while reusing the same name.

### Type Annotations

When you want to explicitly specify a variable's type, you can use a type annotation:

```rust
fn main() {
    // Here the guess variable is expected to be of u32 type
    let guess: u32 = "42".parse().expect("Not a number!");
}
```

## Working with Types

While a detailed guide on data types is available in a separate document, it's important to understand how variables interact with types in Rust:

### Type Inference

Rust can often infer what type you want to use based on the value and how you use it:

```rust
fn main() {
    let x = 5;        // Rust infers this as i32
    let y = 3.0;      // Rust infers this as f64
    let active = true; // Rust infers this as bool
}
```

### Using Type Annotations for Clarity

Even when Rust can infer the type, you might want to add type annotations for clarity:

```rust
fn main() {
    let x: i32 = 5;
    let y: f64 = 3.0;
    let active: bool = true;
}
```

### Type Annotations for Disambiguation

Sometimes Rust needs your help to know which specific type to use:

```rust
fn main() {
    // Without a type annotation, Rust doesn't know which numeric type to use
    let guess = "42".parse().expect("Not a number!");  // Error!

    // With a type annotation, Rust knows exactly what to do
    let guess: u32 = "42".parse().expect("Not a number!");  // Works!
}
```

For more information on the various data types available in Rust, please refer to the [Data Types](data_types.md) guide.
