# Structs in Rust

Structs are custom data types that let you group related values together and name them.

## Defining Structs

```rust
// Basic struct with named fields
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

## Creating Instances

```rust
fn main() {
    // Create a new instance
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Create a mutable instance
    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: false,
        sign_in_count: 3,
    };

    // Modify fields of a mutable struct
    user2.email = String::from("newemail@example.com");
}
```

## Field Init Shorthand

When variables and fields have the same name:

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,    // same as email: email
        username, // same as username: username
        active: true,
        sign_in_count: 1,
    }
}
```

## Struct Update Syntax

Create a new instance from an existing one:

```rust
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Create a new user with some values from user1
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // copy the rest from user1
    };
}
```

## Tuple Structs

Structs without named fields, just types:

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Access fields with index
    println!("Black: ({}, {}, {})", black.0, black.1, black.2);
}
```

## Unit-Like Structs

Structs without any fields:

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
    // Useful when implementing traits but don't need to store data
}
```
