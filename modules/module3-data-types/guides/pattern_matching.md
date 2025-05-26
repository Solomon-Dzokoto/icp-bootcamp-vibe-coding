# Pattern Matching in Rust

Pattern matching is a powerful feature for checking and destructuring values.

## The match Control Flow Operator

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

## Patterns with Values

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... other states
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

## Matching with Option<T>

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five: {:?}, six: {:?}, none: {:?}", five, six, none);
}
```

## Match is Exhaustive

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        // Error! Pattern `None` not covered
        // Must cover all possibilities
    }
}
```

## Catch-all Patterns

### The \_ Placeholder

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (), // Do nothing for all other values
}

fn add_fancy_hat() { /* ... */ }
fn remove_fancy_hat() { /* ... */ }
```

## if let Syntax

Shorter way to handle one pattern and ignore the rest:

```rust
let some_value = Some(3);

// Using match
match some_value {
    Some(3) => println!("three"),
    _ => (),
}

// Using if let (cleaner)
if let Some(3) = some_value {
    println!("three");
}
```

## if let with else

```rust
let coin = Coin::Quarter(UsState::Alaska);

if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    println!("Not a quarter!");
}
```
