use std::collections::HashMap;

// 1. Calculates the sum of even numbers in a slice of i32.
fn sum_of_even(numbers: &[i32]) -> i32 {
    // Iterate over the slice, filter out odd numbers, and sum the remaining even numbers.
    numbers.iter().filter(|&&n| n % 2 == 0).sum()
}

// 2. Counts the frequency of each word in a given string slice.
// Words are converted to lowercase and stripped of non-alphanumeric trailing characters.
fn word_frequency_counter(text: &str) -> HashMap<String, u32> {
    let mut frequencies = HashMap::new();
    // Split the text into words by whitespace.
    for word_str in text.split_whitespace() {
        // Clean the word: trim non-alphanumeric chars from ends, convert to lowercase.
        // Example: "Hello," -> "hello", " Rust." -> "rust"
        let cleaned_word = word_str
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_lowercase();

        // Only count non-empty words.
        if !cleaned_word.is_empty() {
            // Increment the count for the cleaned word.
            // .entry() gets an Entry enum, .or_insert(0) inserts 0 if not present,
            // then we dereference to get the value and increment it.
            *frequencies.entry(cleaned_word).or_insert(0) += 1;
        }
    }
    frequencies
}

// 3. Removes duplicate numbers from a vector and sorts it in place.
fn deduplicate_and_sort(numbers: &mut Vec<i32>) {
    // Sort the vector first. `sort_unstable` is generally faster if order of equal elements doesn't matter.
    numbers.sort_unstable();
    // `dedup` removes consecutive duplicate elements. Since the vector is sorted,
    // all identical elements will be consecutive.
    numbers.dedup();
}

fn main() {
    println!("--- Collection Manipulation Exercises ---");

    // --- Demonstrate sum_of_even ---
    println!("\n--- Sum of Even Numbers ---");
    let number_slice1 = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sum1 = sum_of_even(number_slice1);
    println!("For slice {:?}, sum of even numbers is: {}", number_slice1, sum1); // Expected: 2 + 4 + 6 + 8 + 10 = 30

    let number_slice2 = &[11, 23, 31, 45, 57]; // No even numbers
    let sum2 = sum_of_even(number_slice2);
    println!("For slice {:?}, sum of even numbers is: {}", number_slice2, sum2); // Expected: 0
    
    let number_slice3: &[i32] = &[]; // Empty slice
    let sum3 = sum_of_even(number_slice3);
    println!("For slice {:?}, sum of even numbers is: {}", number_slice3, sum3); // Expected: 0

    // --- Demonstrate word_frequency_counter ---
    println!("\n--- Word Frequency Counter ---");
    let text1 = "Hello world, hello Rust! Rust is fun, world.";
    let frequencies1 = word_frequency_counter(text1);
    println!("Text: \"{}\"", text1);
    println!("Word frequencies: {:?}", frequencies1);
    // Expected: {"hello": 2, "world": 2, "rust": 2, "is": 1, "fun": 1} (order may vary)

    let text2 = "  Test test  TEST!  "; // Test with leading/trailing spaces and punctuation
    let frequencies2 = word_frequency_counter(text2);
    println!("Text: \"{}\"", text2);
    println!("Word frequencies: {:?}", frequencies2);
    // Expected: {"test": 3}

    let text3 = ""; // Empty text
    let frequencies3 = word_frequency_counter(text3);
    println!("Text: \"{}\"", text3);
    println!("Word frequencies: {:?}", frequencies3);
    // Expected: {}

    // --- Demonstrate deduplicate_and_sort ---
    println!("\n--- Deduplicate and Sort Vec<i32> ---");
    let mut vec1 = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    println!("Original vector 1: {:?}", vec1);
    deduplicate_and_sort(&mut vec1);
    println!("Processed vector 1: {:?}", vec1); // Expected: [1, 2, 3, 4, 5, 6, 9]

    let mut vec2 = vec![7, 7, 7, 7, 7];
    println!("Original vector 2: {:?}", vec2);
    deduplicate_and_sort(&mut vec2);
    println!("Processed vector 2: {:?}", vec2); // Expected: [7]

    let mut vec3: Vec<i32> = vec![];
    println!("Original vector 3: {:?}", vec3);
    deduplicate_and_sort(&mut vec3);
    println!("Processed vector 3: {:?}", vec3); // Expected: []

    let mut vec4 = vec![5, 4, 3, 2, 1];
    println!("Original vector 4: {:?}", vec4);
    deduplicate_and_sort(&mut vec4);
    println!("Processed vector 4: {:?}", vec4); // Expected: [1, 2, 3, 4, 5]
}