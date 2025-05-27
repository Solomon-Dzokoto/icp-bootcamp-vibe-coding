use std::collections::HashMap;

fn main() {
    // Sample text for analysis
    let text = "Rust is a multi-paradigm, general-purpose programming language. \
                Rust emphasizes performance, type safety, and concurrency. \
                Rust enforces memory safety—that is, that all references point \
                to valid memory—without requiring the use of a garbage collector \
                or reference counting present in other memory-safe languages.";

    // 1. Split text into words and clean them
    let words: Vec<String> = text
        .split_whitespace()
        .map(|word| word.trim_matches(|c: char| !c.is_alphabetic()).to_lowercase())
        .filter(|word| !word.is_empty())
        .collect();

    // 2. Count word frequencies
    let mut word_counts: HashMap<String, usize> = HashMap::new();
    for word in &words {
        *word_counts.entry(word.clone()).or_insert(0) += 1;
    }

    // 3. Find the longest word
    let longest_word = words
        .iter()
        .max_by_key(|word| word.len())
        .unwrap_or(&String::from(""));

    // 4. Convert to uppercase
    let uppercase_words: Vec<String> = words
        .iter()
        .map(|word| word.to_uppercase())
        .collect();

    // 5. Filter short words
    let filtered_words: Vec<String> = words
        .into_iter()
        .filter(|word| word.len() >= 4)
        .collect();

    // Print results with nice formatting
    println!("\n📊 Text Analysis Results");
    println!("{}", "=".repeat(50));
    
    // Print word frequencies in a sorted manner
    println!("\n📝 Word Frequencies:");
    let mut sorted_counts: Vec<_> = word_counts.iter().collect();
    sorted_counts.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    for (word, count) in sorted_counts {
        println!("  • {:15} : {}", word, count);
    }

    // Print longest word with its length
    println!("\n📏 Longest Word:");
    println!("  • {} ({} characters)", longest_word, longest_word.len());

    // Print uppercase words in a more readable format
    println!("\n⬆️  First 5 Uppercase Words:");
    for word in uppercase_words.iter().take(5) {
        println!("  • {}", word);
    }
    if uppercase_words.len() > 5 {
        println!("  ... and {} more", uppercase_words.len() - 5);
    }

    // Print filtered words statistics
    println!("\n🔍 Words with 4+ Characters:");
    println!("  • Count: {}", filtered_words.len());
    println!("  • Examples: {}", filtered_words.iter().take(5).collect::<Vec<_>>().join(", "));
    if filtered_words.len() > 5 {
        println!("  ... and {} more", filtered_words.len() - 5);
    }

    println!("\n{}", "=".repeat(50));
}