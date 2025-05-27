use std::collections::HashMap;
use std::fmt;

// Define the Book struct
#[derive(Debug, Clone)]
struct Book {
    title: String,
    author: String,
    year: u32,
    isbn: String,
}

impl Book {
    fn new(title: &str, author: &str, year: u32, isbn: &str) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            year,
            isbn: isbn.to_string(),
        }
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\", by {} ({})", self.title, self.author, self.year)
    }
}

// Define a BookStatus enum to track availability
#[derive(Debug, Clone, PartialEq)]
enum BookStatus {
    Available,
    Borrowed { since: String },
}

impl fmt::Display for BookStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BookStatus::Available => write!(f, "Available"),
            BookStatus::Borrowed { since } => write!(f, "Borrowed since {}", since),
        }
    }
}

// Define a Library struct to manage books
struct Library {
    books: HashMap<String, (Book, BookStatus)>,  // ISBN -> (Book, Status)
}

// TODO: Implement methods for the Library struct
impl Library {
    // Create a new, empty library
    fn new() -> Library {
        Library {
            books: HashMap::new(),
        }
    }

    // Add a book to the library
    fn add_book(&mut self, book: Book) {
        self.books.insert(book.isbn.clone(), (book, BookStatus::Available));
    }

    // Borrow a book from the library
    fn borrow_book(&mut self, isbn: &str) -> Result<&Book, &str> {
        match self.books.get_mut(isbn) {
            Some((book, status)) => {
                if *status == BookStatus::Available {
                    *status = BookStatus::Borrowed { 
                        since: chrono::Local::now().format("%Y-%m-%d").to_string() 
                    };
                    Ok(book)
                } else {
                    Err("Book is already borrowed")
                }
            }
            None => Err("Book not found"),
        }
    }

    // Return a borrowed book to the library
    fn return_book(&mut self, isbn: &str) -> Result<&Book, &str> {
        match self.books.get_mut(isbn) {
            Some((book, status)) => {
                if let BookStatus::Borrowed { .. } = status {
                    *status = BookStatus::Available;
                    Ok(book)
                } else {
                    Err("Book is not borrowed")
                }
            }
            None => Err("Book not found"),
        }
    }

    // List all books in the library with their status
    fn list_books(&self) {
        println!("\nLibrary Catalog:");
        println!("---------------");
        if self.books.is_empty() {
            println!("No books in the library.");
            return;
        }

        let mut sorted_books: Vec<_> = self.books.iter().collect();
        sorted_books.sort_by(|a, b| a.1.0.title.cmp(&b.1.0.title));

        for (_, (book, status)) in sorted_books {
            println!("{}", "-".repeat(50));
            println!("Title: {}", book.title);
            println!("Author: {}", book.author);
            println!("Year: {}", book.year);
            println!("ISBN: {}", book.isbn);
            println!("Status: {}", status);
        }
        println!("{}", "-".repeat(50));
    }
}

fn main() {
    // Create a new library
    let mut library = Library::new();
    
    // Add several books to the library
    library.add_book(Book::new(
        "The Rust Programming Language",
        "Steve Klabnik and Carol Nichols",
        2018,
        "9781718500440"
    ));
    
    library.add_book(Book::new(
        "Design Patterns",
        "Erich Gamma et al.",
        1994,
        "9780201633610"
    ));
    
    library.add_book(Book::new(
        "Clean Code",
        "Robert C. Martin",
        2008,
        "9780132350884"
    ));
    
    // List all books
    library.list_books();
    
    // Borrow a book
    println!("Borrowing \"Clean Code\"...");
    match library.borrow_book("9780132350884") {
        Ok(_) => println!("Book borrowed successfully!"),
        Err(e) => println!("Error: {}", e),
    }
    println!();
    
    // List all books again to see the updated status
    library.list_books();
    
    // Return the book
    println!("Returning \"Clean Code\"...");
    match library.return_book("9780132350884") {
        Ok(_) => println!("Book returned successfully!"),
        Err(e) => println!("Error: {}", e),
    }
    println!();
    
    // List all books one more time
    library.list_books();
}