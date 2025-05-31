fn main() {
    // --- Immutable Variable Demo (x) ---

    // 1. Declare an immutable variable `x` and initialize it.
    let x = 5;
    println!("Immutable variable x (initial value): {}", x);

    // 2. Attempting to reassign `x` (will cause a compiler error).
    // x = 10; // Uncommenting this line will result in a compile-time error.
    // println!("This line will not be reached if the above is uncommented.");
    // Note: For the program to run, we keep the reassignment commented out.
    // This demonstrates that immutable variables cannot be reassigned.

    println!("---");

    // --- Mutable Variable Demo (y) ---

    // 3. Declare a mutable variable `y` and initialize it.
    let mut y = 10;
    println!("Mutable variable y (initial value): {}", y);

    // 4. Reassign `y` to a new value.
    y = 20;
    println!("Mutable variable y (new value): {}", y);

    println!("---");

    // --- Shadowing Demo (z) ---

    // 5. Introduce a variable `z`.
    let z = 30;
    println!("Variable z (before shadowing): {}", z);

    // 6. Shadow `z` with a new variable (can be of a different type too).
    let z = "I am now a string"; // z is shadowed by a new variable, also named z.
    println!("Variable z (after shadowing): {}", z);

    // Another example of shadowing with the same type
    let z_numeric = 40;
    println!("Numeric z (z_numeric, before shadowing): {}", z_numeric);
    {
        let z_numeric = z_numeric * 2; // Inner scope z_numeric
        println!("Numeric z (z_numeric, shadowed in inner scope): {}", z_numeric);
    }
    println!("Numeric z (z_numeric, after inner scope): {}", z_numeric); // Back to original z_numeric
}