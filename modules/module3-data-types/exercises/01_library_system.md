# Exercise 1: Library Management System

## Problem Statement

Create a library management system in Rust that uses structs and methods to track books and their borrowing status. The system should be able to:

1. Add new books to the library
2. Allow users to borrow and return books
3. Display information about books and their availability

## Learning Objectives

- Practice defining and working with structs
- Implement methods for structs
- Use enums to represent different states
- Apply ownership principles to struct fields

## Starter Code

```rust
// Define the Book struct
struct Book {
    // TODO: Add fields for book properties (title, author, year, isbn)
}

// Define a BookStatus enum to track availability
enum BookStatus {
    // TODO: Add variants for different states (Available, Borrowed)
}

// Define a Library struct to manage books
struct Library {
    // TODO: Add fields to store books and their status
}

// TODO: Implement methods for the Library struct
impl Library {
    // Create a new, empty library
    fn new() -> Library {
        // TODO: Implement this function
    }

    // Add a book to the library
    fn add_book(&mut self, book: Book) {
        // TODO: Implement this function
    }

    // Borrow a book from the library
    fn borrow_book(&mut self, isbn: &str) -> Result<&Book, &str> {
        // TODO: Implement this function
    }

    // Return a borrowed book to the library
    fn return_book(&mut self, isbn: &str) -> Result<&Book, &str> {
        // TODO: Implement this function
    }

    // List all books in the library with their status
    fn list_books(&self) {
        // TODO: Implement this function
    }
}

fn main() {
    // TODO: Create a new library
    // TODO: Add several books to the library
    // TODO: Borrow and return books
    // TODO: List the books and their status
}
```

## How to Run Your Code

1. First, modify the starter code in `01_library_system_starter.rs` to fix the ownership issues
2. Run your code from the bootcamp root directory with:
   ```
   cargo run --bin module3_01
   ```

## Expected Output

When implemented correctly, running your program should produce output similar to:

```
Library Book Inventory:
"The Rust Programming Language" by Steve Klabnik and Carol Nichols (2018) - Available
"Design Patterns" by Erich Gamma et al. (1994) - Available
"Clean Code" by Robert C. Martin (2008) - Available

Borrowing "Clean Code"...
Book borrowed successfully!

Library Book Inventory:
"The Rust Programming Language" by Steve Klabnik and Carol Nichols (2018) - Available
"Design Patterns" by Erich Gamma et al. (1994) - Available
"Clean Code" by Robert C. Martin (2008) - Borrowed

Returning "Clean Code"...
Book returned successfully!

Library Book Inventory:
"The Rust Programming Language" by Steve Klabnik and Carol Nichols (2018) - Available
"Design Patterns" by Erich Gamma et al. (1994) - Available
"Clean Code" by Robert C. Martin (2008) - Available
```

## Tips

- Use appropriate data structures to store multiple books and their status
- Think about how to represent the relationship between books and their status
- Remember to handle errors for cases like borrowing unavailable books or returning books that weren't borrowed
- Consider using the `Hash` trait for efficient lookups
