# Memory Concepts in Rust

## Stack vs. Heap

### Stack

- Fast, fixed-size memory allocation
- LIFO (Last In, First Out) access pattern
- Data with known, fixed size at compile time
- Automatically deallocated when variables go out of scope

```rust
fn main() {
    // Stack allocated: fixed size known at compile time
    let x = 42;        // Integer (i32)
    let y = true;      // Boolean
    let z = 3.14;      // Float (f64)
}
// x, y, z automatically deallocated here
```

### Heap

- Dynamic size memory allocation
- Access via pointers
- Explicit allocation (though Rust manages this)
- Lives until explicitly deallocated

```rust
fn main() {
    // Heap allocated: size determined at runtime
    let s = String::from("hello"); // Data stored on heap, pointer on stack

    // Heap memory for "hello" is freed automatically when s goes out of scope
}
```

## Memory Layout

### Stack Variables

```rust
fn main() {
    let a = 5;     // 4 bytes directly on stack
    let b = 10;    // 4 bytes directly on stack

    // Stack frame contains 8 bytes total
}
```

### Heap Variables

```rust
fn main() {
    let s1 = String::from("Hello"); // Stack contains pointer, length, capacity
                                     // Heap contains the actual string data

    // s1 on stack: [pointer_addr | 5 | 5]
    //      pointer_addr points to heap memory containing "Hello"
}
```

## Memory Allocation

Rust automatically handles memory allocation and deallocation through its ownership system:

```rust
fn main() {
    // Allocation happens automatically
    {
        let s = String::from("hello"); // Memory allocated
        // s can be used here
    } // s goes out of scope, memory automatically deallocated
}
```

## Memory Safety Challenges

In addition to protect from memory leaks, Rust prevents common memory problems:

### 1. Double Free

```rust
// Not possible in safe Rust:
let s1 = String::from("hello");
let s2 = s1; // s1 is moved, no longer valid
// drop(s1); // Error: use of moved value
```

### 2. Dangling References

```rust
// Compiler prevents this:
fn dangling() -> &String {
    let s = String::from("hello");
    &s // Error: returns reference to data owned by current function
}
```
