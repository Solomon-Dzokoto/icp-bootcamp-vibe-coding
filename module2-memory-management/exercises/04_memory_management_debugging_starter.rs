// Uncomment each section one at a time and fix the issues

// Problem 1: Fix ownership errors
fn problem1() {
    // 1.1: Fix the double-move error
    let data = vec![1, 2, 3];
    let x = data.clone();  // Clone instead of move
    let y = data;  // Now we can use data here
    println!("{:?} {:?}", x, y);

    // 1.2: Fix the ownership issue with the function
    let name = String::from("Rust");
    print_data(&name);  // Pass reference instead of ownership
    println!("My name is {}", name);  // Now we can still use name here
}

fn print_data(data: &String) {  // Take reference instead of ownership
    println!("Data: {}", data);
}

// Problem 2: Fix borrowing conflicts
fn problem2() {
    // 2.1: Fix the mutable/immutable borrow conflict
    let mut numbers = vec![1, 2, 3];
    let first = numbers[0];  // Copy the value instead of borrowing
    numbers.push(4);
    println!("First element is: {}", first);

    // 2.2: Fix the multiple mutable borrows
    let mut data = String::from("Hello");
    {
        let ref1 = &mut data;
        *ref1 = String::from("Hello, ");
    }  // ref1's scope ends here
    {
        let ref2 = &mut data;
        *ref2 = ref2.to_string() + "Rust!";
    }  // ref2's scope ends here
    println!("Data: {}", data);
}

// Problem 3: Fix dangling references
fn problem3() {
    // 3.1: Fix the dangling reference returned by the function
    let result = get_string();
    println!("Result: {}", result);

    // 3.2: Fix the issue with references outliving the data
    let data = vec![1, 2, 3];  // Move the data outside the block
    let reference = &data;
    println!("Reference: {:?}", reference);
}

fn get_string() -> String {  // Return owned String instead of reference
    String::from("I am no longer a dangling reference")
}

// Problem 4: Fix lifetime problems
fn problem4() {
    // 4.1: Fix the function signature to properly handle lifetimes
    let string1 = String::from("long string is long");
    let string2 = String::from("short");  // Move string2 outside the block
    let result = longest(string1.as_str(), string2.as_str());
    println!("Longest string: {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {  // Add lifetime annotation
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Problem 5: Optimize unnecessary cloning
fn problem5() {
    // 5.1: Remove unnecessary clones while keeping the code functional
    let original = String::from("Rust Programming");
    let len = calculate_length(&original);  // Pass reference instead of clone
    
    let mut names = Vec::new();
    names.push(String::from("Alice"));
    names.push(String::from("Bob"));
    
    for name in &names {  // Iterate over references
        print_string(name);  // Pass reference
    }
    
    println!("Original is still: {}", original);
    println!("Length was: {}", len);
    println!("Names: {:?}", names);
}

fn calculate_length(s: &String) -> usize {  // Take reference instead of ownership
    s.len()
}

fn print_string(s: &String) {  // Take reference instead of ownership
    println!("{}", s);
}

fn main() {
    println!("Uncomment and fix each problem section one by one.");
    println!("Once fixed, you can run each problem function from main.");
    
    // All problems are fixed, uncomment to test:
    problem1();
    problem2();
    problem3();
    problem4();
    problem5();
}