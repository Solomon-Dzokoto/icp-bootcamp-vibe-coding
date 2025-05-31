// 1. Get the first element of an integer slice
// Takes a slice of integers and returns an Option<&i32>.
// Returns Some(&slice[0]) if the slice is not empty, otherwise None.
fn get_first_element(slice: &[i32]) -> Option<&i32> {
    // The .get() method on slices is preferred as it gracefully handles empty slices
    // by returning None, preventing panics.
    slice.get(0)
}

// 2. Calculate the sum of elements in an integer slice
// Takes a slice of integers and returns their sum.
fn get_slice_sum(numbers: &[i32]) -> i32 {
    // .iter() creates an iterator over the slice elements.
    // .sum() consumes the iterator and sums the elements.
    // This is a concise and idiomatic way to sum elements in a slice.
    numbers.iter().sum()
}

// 3. Find the first word in a string slice
// Takes a string slice and returns a slice representing the first word.
// A word is defined as a sequence of characters separated by a space.
// If no space is found, the entire string slice is considered the first word.
fn find_first_word(s: &str) -> &str {
    // Convert the string slice to a byte slice to iterate over bytes.
    // This is useful for finding ASCII space characters.
    // For full Unicode word boundary detection, a more complex approach would be needed.
    let bytes = s.as_bytes();

    // Iterate over the bytes along with their indices.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // If a space byte (b' ') is found,
            return &s[0..i]; // return the slice from the beginning of the string to the space.
        }
    }

    // If no space is found after iterating through all bytes,
    // the entire string slice is considered the first word.
    s
}

fn main() {
    // --- Test get_first_element function ---
    println!("--- Testing get_first_element ---");
    let numbers1 = [10, 20, 30, 40, 50];
    match get_first_element(&numbers1) {
        Some(first) => println!("First element of {:?}: {}", numbers1, first), // Expected: 10
        None => println!("Slice {:?} is empty.", numbers1),
    }

    let numbers2: [i32; 0] = []; // Empty slice
    match get_first_element(&numbers2) {
        Some(first) => println!("First element of {:?}: {}", numbers2, first),
        None => println!("Slice {:?} is empty.", numbers2), // Expected: Slice [] is empty.
    }

    // --- Test get_slice_sum function ---
    println!("\n--- Testing get_slice_sum ---");
    let arr1 = [1, 2, 3, 4, 5];
    let sum1 = get_slice_sum(&arr1);
    println!("Sum of elements in {:?}: {}", arr1, sum1); // Expected: 15

    let arr2 = [10, -2, 7, 0, 5];
    let sum2 = get_slice_sum(&arr2);
    println!("Sum of elements in {:?}: {}", arr2, sum2); // Expected: 20

    let arr3: [i32; 0] = []; // Empty slice
    let sum3 = get_slice_sum(&arr3);
    println!("Sum of elements in {:?}: {}", arr3, sum3); // Expected: 0

    // Test with a sub-slice
    let partial_sum = get_slice_sum(&arr1[0..3]);
    println!("Sum of first three elements in {:?} (sub-slice {:?}): {}", arr1, &arr1[0..3], partial_sum); // Expected: 6

    // --- Test find_first_word function ---
    println!("\n--- Testing find_first_word ---");
    let sentence1 = String::from("Hello Rust slices world");
    let first1 = find_first_word(&sentence1);
    println!("First word in '{}': '{}'", sentence1, first1); // Expected: Hello

    let sentence2 = String::from("onlyone"); // String with no spaces
    let first2 = find_first_word(&sentence2);
    println!("First word in '{}': '{}'", sentence2, first2); // Expected: onlyone
    
    let sentence3 = String::from(""); // Empty string
    let first3 = find_first_word(&sentence3);
    println!("First word in '{}': '{}'", sentence3, first3); // Expected: ''

    let sentence4 = String::from(" leading space"); // String with a leading space
    let first4 = find_first_word(&sentence4); // The first "word" is empty
    println!("First word in '{}': '{}'", sentence4, first4); // Expected: ''

    let sentence5 = String::from("trailing space "); // String with a trailing space
    let first5 = find_first_word(&sentence5);
    println!("First word in '{}': '{}'", sentence5, first5); // Expected: "trailing"
}