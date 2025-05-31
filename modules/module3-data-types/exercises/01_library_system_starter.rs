// --- Struct Definitions ---

// Represents a book in the library.
#[derive(Debug, Clone)] // Added Clone for easier management if needed later.
struct Book {
    title: String,
    author: String,
    isbn: String,
    available: bool,
}

impl Book {
    // Constructor for Book.
    fn new(title: &str, author: &str, isbn: &str) -> Self {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            isbn: isbn.to_string(),
            available: true, // Books are available by default when added.
        }
    }
}

// Represents a library member.
#[derive(Debug)]
struct Member {
    name: String,
    member_id: u32,
    borrowed_books: Vec<String>, // Stores ISBNs of borrowed books.
}

impl Member {
    // Constructor for Member.
    fn new(name: &str, member_id: u32) -> Self {
        Member {
            name: name.to_string(),
            member_id,
            borrowed_books: Vec::new(),
        }
    }
}

// Represents the library, containing books and members.
#[derive(Debug)]
struct Library {
    books: Vec<Book>,
    members: Vec<Member>,
}

impl Library {
    // Creates a new, empty library.
    fn new() -> Self {
        Library {
            books: Vec::new(),
            members: Vec::new(),
        }
    }

    // Adds a new book to the library's collection.
    fn add_book(&mut self, book: Book) {
        // Check if book with same ISBN already exists to prevent duplicates
        if self.books.iter().any(|b| b.isbn == book.isbn) {
            println!("Book with ISBN {} already exists.", book.isbn);
            return;
        }
        self.books.push(book);
    }

    // Registers a new member with the library.
    fn register_member(&mut self, member: Member) {
        // Check if member ID already exists
        if self.members.iter().any(|m| m.member_id == member.member_id) {
            println!("Member with ID {} already registered.", member.member_id);
            return;
        }
        self.members.push(member);
    }

    // Allows a member to borrow a book.
    fn borrow_book(&mut self, member_id: u32, isbn: &str) -> Result<(), String> {
        // Find the member.
        let member = self.members.iter_mut().find(|m| m.member_id == member_id)
            .ok_or_else(|| format!("Member with ID {} not found.", member_id))?;

        // Find the book and check its availability.
        let book = self.books.iter_mut().find(|b| b.isbn == isbn)
            .ok_or_else(|| format!("Book with ISBN {} not found.", isbn))?;

        if !book.available {
            return Err(format!("Book '{}' (ISBN: {}) is not available.", book.title, isbn));
        }

        // Update book status and member's borrowed list.
        book.available = false;
        member.borrowed_books.push(isbn.to_string());

        Ok(())
    }

    // Allows a member to return a book.
    fn return_book(&mut self, member_id: u32, isbn: &str) -> Result<(), String> {
        // Find the member.
        let member = self.members.iter_mut().find(|m| m.member_id == member_id)
            .ok_or_else(|| format!("Member with ID {} not found.", member_id))?;

        // Check if the member actually borrowed this book.
        if let Some(pos) = member.borrowed_books.iter().position(|borrowed_isbn| borrowed_isbn == isbn) {
            member.borrowed_books.remove(pos); // Remove from member's list.
        } else {
            return Err(format!("Member {} did not borrow book with ISBN {}.", member.name, isbn));
        }

        // Find the book and update its availability.
        // This assumes the book exists if the member borrowed it.
        // A more robust system might re-verify book existence or handle inconsistencies.
        if let Some(book) = self.books.iter_mut().find(|b| b.isbn == isbn) {
            book.available = true;
        } else {
            // This case should ideally not be reached if data is consistent.
            // It means a member returned a book that's no longer in the library catalog.
            return Err(format!("Book with ISBN {} was borrowed but is now missing from catalog. Setting as returned for member.", isbn));
        }

        Ok(())
    }

    // Helper function to display book details (optional, for easier debugging in main)
    fn print_book_status(&self, isbn: &str) {
        match self.books.iter().find(|b| b.isbn == isbn) {
            Some(book) => println!("Status for '{}' (ISBN: {}): Available = {}", book.title, book.isbn, book.available),
            None => println!("Book with ISBN {} not found in library.", isbn),
        }
    }

    // Helper function to display member details (optional)
    fn print_member_status(&self, member_id: u32) {
         match self.members.iter().find(|m| m.member_id == member_id) {
            Some(member) => println!("Member '{}' (ID: {}): Borrowed books (ISBNs): {:?}", member.name, member.member_id, member.borrowed_books),
            None => println!("Member with ID {} not found in library.", member_id),
        }
    }
}

fn main() {
    let mut library = Library::new();

    // Add books
    println!("--- Adding Books ---");
    let book1 = Book::new("The Rust Programming Language", "Klabnik & Nichols", "978-1718500440");
    let book2 = Book::new("Clean Code", "Robert C. Martin", "978-0132350884");
    let book3 = Book::new("Design Patterns", "Gamma et al.", "978-0201633610");
    
    library.add_book(book1.clone()); // Use clone if you want to use book1 variable later
    library.add_book(book2.clone());
    library.add_book(book3.clone());
    library.print_book_status("978-1718500440");
    library.print_book_status("978-0132350884");
    println!();

    // Register members
    println!("--- Registering Members ---");
    let member1 = Member::new("Alice Smith", 101);
    let member2 = Member::new("Bob Johnson", 102);
    library.register_member(member1);
    library.register_member(member2);
    library.print_member_status(101);
    library.print_member_status(102);
    println!();

    // Borrowing books
    println!("--- Borrowing Books ---");
    // Alice borrows "Clean Code"
    match library.borrow_book(101, "978-0132350884") {
        Ok(()) => println!("Alice successfully borrowed 'Clean Code'."),
        Err(e) => println!("Error borrowing for Alice: {}", e),
    }
    library.print_book_status("978-0132350884");
    library.print_member_status(101);
    println!();

    // Bob tries to borrow "Clean Code" (should fail as it's already borrowed)
    println!("Bob tries to borrow 'Clean Code' (already borrowed by Alice)...");
    match library.borrow_book(102, "978-0132350884") {
        Ok(()) => println!("Bob successfully borrowed 'Clean Code'."),
        Err(e) => println!("Error borrowing for Bob: {}", e),
    }
    println!();

    // Alice tries to borrow "The Rust Programming Language"
    match library.borrow_book(101, "978-1718500440") {
        Ok(()) => println!("Alice successfully borrowed 'The Rust Programming Language'."),
        Err(e) => println!("Error borrowing for Alice: {}", e),
    }
    library.print_book_status("978-1718500440");
    library.print_member_status(101);
    println!();

    // Returning books
    println!("--- Returning Books ---");
    // Alice returns "Clean Code"
    match library.return_book(101, "978-0132350884") {
        Ok(()) => println!("Alice successfully returned 'Clean Code'."),
        Err(e) => println!("Error returning for Alice: {}", e),
    }
    library.print_book_status("978-0132350884");
    library.print_member_status(101);
    println!();

    // Bob tries to return "Design Patterns" (which he never borrowed)
    println!("Bob tries to return 'Design Patterns' (not borrowed by him)...");
    match library.return_book(102, "978-0201633610") {
        Ok(()) => println!("Bob successfully returned 'Design Patterns'."),
        Err(e) => println!("Error returning for Bob: {}", e),
    }
    library.print_member_status(102);
    println!();
    
    // Attempt to borrow a non-existent book
    println!("Alice tries to borrow non-existent ISBN '000-0000000000'...");
    match library.borrow_book(101, "000-0000000000") {
        Ok(()) => println!("Book borrowed successfully."),
        Err(e) => println!("Error: {}", e),
    }
    println!();

    // Attempt to borrow with a non-existent member ID
    println!("Non-existent member 999 tries to borrow a book...");
    match library.borrow_book(999, "978-1718500440") {
        Ok(()) => println!("Book borrowed successfully."),
        Err(e) => println!("Error: {}", e),
    }
    println!();
}