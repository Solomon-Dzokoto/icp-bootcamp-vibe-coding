// 1. Processing string data with immutable references
// This function takes an immutable reference to a String and returns its length.
// Taking a reference (&String) means the function borrows the String data
// without taking ownership. The original String remains valid after the call.
fn get_length(s: &String) -> usize {
    s.len() // .len() can be called on an immutable reference.
}

// 2. Modifying vector data with mutable references
// This function takes a mutable reference to a Vec<i32> and adds elements to it.
// A mutable reference (&mut Vec<i32>) allows the function to modify the vector.
// The caller must use `&mut` and the vector itself must be declared as `mut`.
fn add_three_elements(vec: &mut Vec<i32>) {
    vec.push(10);
    vec.push(20);
    vec.push(30);
}

// 3. Processing multiple data structures
// This function takes immutable references to a vector of numbers and a vector of strings.
// It calculates the average of the numbers and counts the strings.
fn calculate_stats(numbers: &Vec<f64>, strings: &Vec<String>) -> (f64, i32) {
    // Calculate the sum of numbers. .iter().sum() works on a reference.
    let sum: f64 = numbers.iter().sum();
    // Calculate average, handling empty vector case to avoid division by zero.
    let average = if !numbers.is_empty() {
        sum / numbers.len() as f64
    } else {
        0.0
    };
    // Get the count of strings. .len() works on a reference.
    let count = strings.len() as i32;
    (average, count)
}

// 4. Borrowing rules demonstration
// This function demonstrates fixing common borrowing errors.
fn fix_borrowing_issues() {
    let mut data = vec![1, 2, 3];
    // println!("Initial data: {:?}", data); // Optional: to see initial state

    // Original issue 1: Two mutable references to `data` in the same scope.
    // let ref1 = &mut data;
    // let ref2 = &mut data; // Error: cannot borrow `data` as mutable more than once at a time
    // ref1.push(4);
    // ref2.push(5);

    // Fix 1: Ensure mutable borrows do not overlap in scope.
    // Each mutable borrow is contained within its own block, so their scopes don't overlap.
    {
        let ref1 = &mut data; // First mutable borrow
        ref1.push(4);
    } // ref1 goes out of scope here, its borrow ends.

    {
        let ref2 = &mut data; // Second mutable borrow is now valid as ref1's borrow ended.
        ref2.push(5);
    } // ref2 goes out of scope here.
    // Expected output: Modified data: [1, 2, 3, 4, 5]
    println!("Modified data: {:?}", data); // Shows data after first set of fixes
    
    // Original issue 2: Mutable borrow while an immutable borrow is active.
    // let ref3 = &data;       // Immutable borrow
    // let ref4 = &mut data;   // Error: cannot borrow `data` as mutable because it's already borrowed as immutable
    // println!("Data length: {}", ref3.len()); // Use of immutable borrow
    // ref4.push(6);                          // Use of mutable borrow

    // Fix 2: Ensure the immutable borrow's scope ends before the mutable borrow begins.
    // The immutable borrow (ref3) is scoped so it's dropped before ref4 is created.
    {
        let ref3 = &data; // Immutable borrow
        // Expected output: Data length: 5
        println!("Data length: {}", ref3.len()); // Use the immutable borrow
    } // ref3 goes out of scope here, its immutable borrow ends.

    {
        let ref4 = &mut data; // Mutable borrow is now valid as ref3's borrow ended.
        ref4.push(6); // Use the mutable borrow
    } // ref4 goes out of scope here.

    // This print statement is added to show the final state of `data`
    // to match the cumulative expected output from the markdown.
    // Expected output: Modified data: [1, 2, 3, 4, 5, 6] (implicitly, after "Data length: 5")
    println!("Modified data: {:?}", data); // Shows final state of data
}

fn main() {
    // 1. Test immutable reference function
    println!("--- Testing get_length ---");
    let test_string = String::from("Hello, Rust borrowing!");
    let length = get_length(&test_string); // Pass an immutable reference
    println!("String length: {}", length);
    // Verify the string is still usable after passing as reference
    println!("Original string: '{}'", test_string); // Original string is unchanged and usable
    
    // 2. Test mutable reference function
    println!("\n--- Testing add_three_elements ---");
    let mut my_vec: Vec<i32> = Vec::new(); // Vector must be mutable to be borrowed mutably
    println!("Before function call: {:?}", my_vec);
    add_three_elements(&mut my_vec); // Pass a mutable reference
    println!("After function call: {:?}", my_vec); // Vector is modified
    
    // 3. Test multiple references
    println!("\n--- Testing calculate_stats ---");
    let numbers = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    let words = vec![String::from("apple"), String::from("banana"), String::from("cherry")];
    // Pass immutable references to both vectors
    let (average, count) = calculate_stats(&numbers, &words);
    println!("Average of numbers: {:.1}, Count of strings: {}", average, count);
    
    // 4. Test the fixed borrowing issues
    println!("\n--- Testing fix_borrowing_issues ---");
    fix_borrowing_issues();
}