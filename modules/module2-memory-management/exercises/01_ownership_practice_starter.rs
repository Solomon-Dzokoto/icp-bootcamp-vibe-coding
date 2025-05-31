// Example 1: String ownership
fn example1() {
    let s1 = String::from("hello");
    // The assignment `let s2 = s1;` moves ownership of the String data from s1 to s2.
    // After the move, s1 is no longer valid and cannot be used.
    // To fix this, so that s1 can still be used for the println!,
    // we need to create a deep copy of s1's data for s2 using clone().
    let s2 = s1.clone();

    // Now, s1 is still valid because s2 holds a separate copy of the data.
    // The original code `println!("{}, world!", s1);` will now work.
    // Expected output: "hello, world!"
    println!("{}, world!", s1);
}

// Example 2: Function ownership
fn example2() {
    let s = String::from("hello");

    // When `s` is passed to `takes_ownership`, ownership of `s` is moved into the function.
    // Thus, `s` cannot be used after the function call.
    // To fix this and allow `s` to be used in `println!`, we can pass a clone of `s`.
    // This way, `takes_ownership` receives a copy, and `s` in `example2` retains ownership.
    takes_ownership(s.clone());

    // s is still valid here because only a clone was moved.
    // Expected output: "After function call: hello"
    println!("After function call: {}", s);
}

// This function takes ownership of the String passed to it.
fn takes_ownership(some_string: String) {
    // Expected output: "Inside function: hello"
    println!("Inside function: {}", some_string);
    // `some_string` goes out of scope here and its memory is freed.
}

// Example 3: Vector ownership
fn example3() {
    let v = vec![1, 2, 3, 4, 5];

    // Iterating over `v` directly using `for i in v` moves ownership of `v` into the loop.
    // This means `v` cannot be used afterwards (e.g., for `v.iter().sum()`).
    // To fix this, we iterate over a reference to `v` (`&v`).
    // This allows the loop to borrow `v` without taking ownership.
    println!("Elements of vector v:"); // Added for clarity, matches expected output style
    for i in &v { // Iterate by reference, so v is borrowed, not moved.
        println!("{}", i); // i is of type &i32. println! handles dereferencing.
    }

    // `v` is still valid here because the loop only borrowed it.
    // `v.iter()` also borrows `v`, which is allowed.
    // Expected output: "Sum: 15"
    let sum: i32 = v.iter().sum();
    println!("Sum: {}", sum);
}

fn main() {
    println!("--- Running Example 1 ---");
    example1();
    
    println!("\n--- Running Example 2 ---");
    example2();
    
    println!("\n--- Running Example 3 ---");
    example3();
}