use std::io;
use chrono::Local;

fn main() {
    // TODO: 1. Prompt the user for their name
    println!("Please enter your name: ");

    // TODO: 2. Read the user's input
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim();

    // TODO: 3. Print a personalized greeting
    println!("Hello, {}! Nice to meet you.", name);

    // BONUS: Print the current date
    let current_date = Local::now();
    println!("Today's date is: {}", current_date.format("%B %d, %Y"));
}