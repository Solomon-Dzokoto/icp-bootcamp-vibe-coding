# Final Challenge: Rust Task Manager

## Challenge Overview

Now that you've completed the core modules of the Rust bootcamp, it's time to put your skills to the test with a comprehensive challenge. In this project, you'll build a command-line task management application that incorporates concepts from all the core modules.

## Learning Objectives

- Apply Rust fundamentals in a practical project
- Combine multiple concepts including ownership, structs, enums, modules, and collections
- Gain experience with building a complete, usable application
- Practice error handling and user input processing

## Requirements

### Core Functionality

Your task manager application should be able to:

1. **Add tasks** with a title, description, and optional due date
2. **List all tasks** with their details and status
3. **Mark tasks as complete**
4. **Delete tasks**
5. **Save tasks to a file** and **load tasks from a file**
6. **Filter tasks** by status (pending/completed) and due date

### Technical Requirements

1. **Data Modeling (Module 3)**

   - Define appropriate structs and enums for tasks and their properties
   - Implement methods for task manipulation
   - Use enums for task status and command types

2. **Memory Management (Module 2)**

   - Apply appropriate ownership and borrowing patterns
   - Avoid unnecessary cloning and copies
   - Clean up resources properly

3. **Code Organization (Module 4)**

   - Organize code into modules (e.g., task management, file I/O, user interface)
   - Apply visibility rules appropriately
   - Create a clean API between modules

4. **Collections (Module 5)**

   - Store tasks in appropriate collection types
   - Filter and sort tasks efficiently
   - Handle string manipulation for user input and output

5. **Error Handling**
   - Use Result and Option types for error handling
   - Provide meaningful error messages to users
   - Handle file I/O errors gracefully

## Starter Code

```rust
// This is a minimal starter structure - feel free to modify as needed!

// Task status enum
enum TaskStatus {
    Pending,
    Completed,
}

// Task struct to store task information
struct Task {
    id: u32,
    title: String,
    description: String,
    due_date: Option<String>, // Consider using a proper date type in your implementation
    status: TaskStatus,
}

// TaskManager to handle operations on tasks
struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskManager {
    // Create a new TaskManager
    fn new() -> TaskManager {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    // Add the rest of the implementation...
}

// Command enum to represent user commands
enum Command {
    Add { title: String, description: String, due_date: Option<String> },
    List,
    Complete { id: u32 },
    Delete { id: u32 },
    Save { filename: String },
    Load { filename: String },
    Quit,
    Unknown,
}

fn main() {
    // Initialize task manager
    let mut task_manager = TaskManager::new();

    // Main application loop
    loop {
        // Get user command
        // Process command
        // Exit if command is Quit
    }
}
```

## Extension Ideas

Once you have the basic functionality working, consider adding these features:

1. **Task Categories or Tags**: Allow users to categorize tasks and filter by category
2. **Priority Levels**: Add priority levels to tasks and sort by priority
3. **Recurring Tasks**: Implement support for tasks that repeat at regular intervals
4. **Better Date Handling**: Use a proper date/time crate like `chrono` for due dates
5. **Interactive UI**: Improve the command-line interface with colored text or a TUI library

## Submission Requirements

Your final project should include:

1. Well-organized, commented Rust code
2. A README.md explaining how to build and use your application
3. Example input and output showing your application in action

## Tips

- Start with a simple implementation and add features incrementally
- Test your code frequently as you develop
- Consider edge cases like empty task lists and invalid user input
- Focus on creating a good user experience with clear instructions and feedback

Good luck, and have fun applying your Rust knowledge to create a useful application!
