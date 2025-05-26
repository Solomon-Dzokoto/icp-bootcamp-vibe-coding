# Data Types in Rust

Rust is a statically typed language, which means it must know the types of all variables at compile time. This guide covers the various data types available in Rust.

## Scalar Types

Scalar types represent a single value. Rust has four primary scalar types:

### Integers

Integers are numbers without fractional components. Rust provides a variety of integer types with different sizes:

| Length  | Signed | Unsigned |
| ------- | ------ | -------- |
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

The `isize` and `usize` types depend on the architecture of the computer your program is running on:

- 64 bits on 64-bit architecture
- 32 bits on 32-bit architecture

Integer literals can be written in various formats:

```rust
fn main() {
    let a = 98_222;    // Decimal
    let b = 0xff;      // Hex
    let c = 0o77;      // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A';      // Byte (u8 only)
}
```

The underscore (`_`) can be used as a visual separator for readability.

### Floating-Point Numbers

Rust has two floating-point types: `f32` and `f64`. The default is `f64` because on modern CPUs, it's roughly the same speed as `f32` but offers more precision.

```rust
fn main() {
    let x = 2.0;      // f64 (default)
    let y: f32 = 3.0; // f32

    // Basic operations
    let sum = 5.0 + 10.0;     // addition
    let difference = 95.5 - 4.3;  // subtraction
    let product = 4.0 * 30.0;     // multiplication
    let quotient = 56.7 / 32.2;   // division
    let remainder = 43.0 % 5.0;   // remainder
}
```

### Booleans

A boolean type has two possible values: `true` and `false`. Booleans are one byte in size and are often used in conditional expressions.

```rust
fn main() {
    let t = true;
    let f: bool = false;

    // Booleans are often used in conditionals
    if t {
        println!("This will be printed");
    }

    if f {
        println!("This will not be printed");
    }
}
```

### Characters

Rust's `char` type is the language's most primitive alphabetic type. It represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ';         // Unicode character
    let heart_eyed_cat = '😻'; // Unicode emoji

    println!("Character size: {} bytes", std::mem::size_of::<char>());  // 4 bytes
}
```

Note that `char` literals use single quotes, while string literals use double quotes.

## Compound Types

Compound types can group multiple values into one type. Rust has several compound types:

### Tuples

A tuple is a general way of grouping together a number of values with a variety of types into one compound type with a fixed length.

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructuring
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    // Access via index
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
}
```

The tuple without any values, `()`, is a special type called the unit type. This value and its corresponding type are both written `()` and represent an empty value or an empty return type.

### Arrays

Arrays in Rust have a fixed length and every element must have the same type:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    // Arrays with explicit type and size
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // Creating an array with the same value repeated
    let a = [3; 5]; // Equivalent to [3, 3, 3, 3, 3]

    // Accessing array elements
    let first = a[0];
    let second = a[1];

    // Arrays have bounds checking at runtime
    // let invalid = a[10]; // This would cause a runtime panic
}
```

Unlike some low-level languages, Rust checks for array bounds at runtime, which helps prevent buffer overflow vulnerabilities.

### Slices

Slices are similar to arrays but their size is not known at compile time. A slice is a reference to a contiguous sequence of elements in a collection:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3]; // This is a slice containing elements 2 and 3

    println!("The first element of the slice is: {}", slice[0]); // 2
}
```

Slices are a view into the original data and don't take ownership of the data they reference.

### Strings

Rust has two string types:

- `String`: a growable, heap-allocated data structure
- `&str`: a string slice, often used as a view into a String

```rust
fn main() {
    // String type (owned)
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // String slice (borrowed)
    let hello = "Hello, world!";
    let slice = &hello[0..5]; // "Hello"
    println!("{}", slice);
}
```

## Type Conversion

Rust requires explicit type conversion (casting) between numeric types:

```rust
fn main() {
    let x = 5;
    let y = 3.0;

    // This won't compile:
    // let sum = x + y;

    // Instead, do this:
    let sum = x as f64 + y;

    // Or convert the float to an integer (truncates the decimal part)
    let truncated = y as i32 + x;
}
```

For more complex conversions between types, Rust provides the `From` and `Into` traits:

```rust
fn main() {
    let s = String::from("hello"); // Convert &str to String

    let num_str = String::from("42");
    let num: i32 = num_str.parse().expect("Not a number!"); // Convert String to i32
}
```

## Custom Types

Rust allows you to define your own types using structs and enums, which we'll cover in other guides.
