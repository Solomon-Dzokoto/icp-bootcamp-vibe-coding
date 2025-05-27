use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use chrono::{NaiveDateTime, Local};
use serde::{Serialize, Deserialize};
use colored::*;

// Task status enum
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Archived,
}

impl TaskStatus {
    fn to_colored_string(&self) -> ColoredString {
        match self {
            TaskStatus::Pending => "PENDING".yellow(),
            TaskStatus::InProgress => "IN PROGRESS".blue(),
            TaskStatus::Completed => "COMPLETED".green(),
            TaskStatus::Archived => "ARCHIVED".red(),
        }
    }
}

// Task struct to store task information
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    id: u32,
    title: String,
    description: String,
    due_date: Option<String>, // Consider using a proper date type in your implementation
    status: TaskStatus,
    created_at: String,
    tags: Vec<String>,
    priority: Priority,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    fn to_colored_string(&self) -> ColoredString {
        match self {
            Priority::Low => "LOW".green(),
            Priority::Medium => "MEDIUM".yellow(),
            Priority::High => "HIGH".red(),
        }
    }
}

// TaskManager to handle operations on tasks
struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
    tags: HashMap<String, Vec<u32>>,  // Tag -> Task IDs
}

impl TaskManager {
    // Create a new TaskManager
    fn new() -> TaskManager {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
            tags: HashMap::new(),
        }
    }
    
    // Add a new task to the task manager
    fn add_task(&mut self, title: String, description: String, due_date: Option<String>, 
                tags: Vec<String>, priority: Priority) -> &Task {
        let task = Task {
            id: self.next_id,
            title,
            description,
            due_date,
            status: TaskStatus::Pending,
            created_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            tags: tags.clone(),
            priority,
        };
        
        // Update tags index
        for tag in tags {
            self.tags.entry(tag)
                .or_insert_with(Vec::new)
                .push(self.next_id);
        }
        
        self.next_id += 1;
        self.tasks.push(task);
        self.tasks.last().unwrap()
    }

    fn list_tasks(&self, filter: Option<TaskFilter>) {
        if self.tasks.is_empty() {
            println!("{}", "No tasks found.".yellow());
            return;
        }

        let filtered_tasks: Vec<&Task> = match filter {
            Some(TaskFilter::Status(status)) => {
                self.tasks.iter().filter(|t| t.status == status).collect()
            }
            Some(TaskFilter::Priority(priority)) => {
                self.tasks.iter().filter(|t| t.priority == priority).collect()
            }
            Some(TaskFilter::Tag(tag)) => {
                if let Some(task_ids) = self.tags.get(&tag) {
                    self.tasks.iter()
                        .filter(|t| task_ids.contains(&t.id))
                        .collect()
                } else {
                    Vec::new()
                }
            }
            None => self.tasks.iter().collect(),
        };

        if filtered_tasks.is_empty() {
            println!("{}", "No tasks match the filter.".yellow());
            return;
        }

        println!("\n📋 Task List");
        println!("{}", "=".repeat(80));
        for task in filtered_tasks {
            self.print_task(task);
        }
        println!("{}", "=".repeat(80));
    }

    fn print_task(&self, task: &Task) {
        println!("\n{} {}", "ID:".bold(), task.id);
        println!("{} {}", "Title:".bold(), task.title);
        println!("{} {}", "Status:".bold(), task.status.to_colored_string());
        println!("{} {}", "Priority:".bold(), task.priority.to_colored_string());
        if !task.description.is_empty() {
            println!("{} {}", "Description:".bold(), task.description);
        }
        if let Some(date) = &task.due_date {
            println!("{} {}", "Due Date:".bold(), date);
        }
        if !task.tags.is_empty() {
            println!("{} {}", "Tags:".bold(), task.tags.join(", ").blue());
        }
        println!("{} {}", "Created:".bold(), task.created_at);
    }

    fn complete_task(&mut self, id: u32) -> Result<(), String> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.status = TaskStatus::Completed;
            Ok(())
        } else {
            Err("Task not found".to_string())
        }
    }

    fn delete_task(&mut self, id: u32) -> Result<(), String> {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            let task = self.tasks.remove(pos);
            // Remove task ID from tags
            for tag in task.tags {
                if let Some(task_ids) = self.tags.get_mut(&tag) {
                    task_ids.retain(|&x| x != id);
                }
            }
            Ok(())
        } else {
            Err("Task not found".to_string())
        }
    }

    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self.tasks)?;
        fs::write(filename, json)?;
        Ok(())
    }

    fn load_from_file(&mut self, filename: &str) -> io::Result<()> {
        let content = fs::read_to_string(filename)?;
        self.tasks = serde_json::from_str(&content)?;
        // Rebuild tags index
        self.tags.clear();
        for task in &self.tasks {
            for tag in &task.tags {
                self.tags.entry(tag.clone())
                    .or_insert_with(Vec::new)
                    .push(task.id);
            }
        }
        if let Some(max_id) = self.tasks.iter().map(|t| t.id).max() {
            self.next_id = max_id + 1;
        }
        Ok(())
    }
}

#[derive(Debug)]
enum TaskFilter {
    Status(TaskStatus),
    Priority(Priority),
    Tag(String),
}

// Command enum to represent user commands
enum Command {
    Add {
        title: String,
        description: String,
        due_date: Option<String>,
        tags: Vec<String>,
        priority: Priority,
    },
    List(Option<TaskFilter>),
    Complete { id: u32 },
    Delete { id: u32 },
    Save { filename: String },
    Load { filename: String },
    Help,
    Quit,
    Unknown,
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn parse_command(input: &str) -> Command {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.is_empty() {
        return Command::Unknown;
    }

    match parts[0].to_lowercase().as_str() {
        "add" => {
            println!("\n📝 Adding New Task");
            let title = get_input("Title: ");
            let description = get_input("Description: ");
            let due_date = get_input("Due Date (YYYY-MM-DD, or press Enter to skip): ");
            let due_date = if due_date.is_empty() { None } else { Some(due_date) };
            let tags_input = get_input("Tags (comma-separated): ");
            let tags: Vec<String> = tags_input
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            
            println!("Priority (1: Low, 2: Medium, 3: High): ");
            let priority = match get_input("").as_str() {
                "3" => Priority::High,
                "2" => Priority::Medium,
                _ => Priority::Low,
            };

            Command::Add {
                title,
                description,
                due_date,
                tags,
                priority,
            }
        }
        "list" => {
            if parts.len() > 1 {
                match parts[1] {
                    "pending" => Command::List(Some(TaskFilter::Status(TaskStatus::Pending))),
                    "completed" => Command::List(Some(TaskFilter::Status(TaskStatus::Completed))),
                    "high" => Command::List(Some(TaskFilter::Priority(Priority::High))),
                    _ => Command::List(Some(TaskFilter::Tag(parts[1].to_string()))),
                }
            } else {
                Command::List(None)
            }
        }
        "complete" => {
            if parts.len() > 1 {
                if let Ok(id) = parts[1].parse() {
                    Command::Complete { id }
                } else {
                    Command::Unknown
                }
            } else {
                Command::Unknown
            }
        }
        "delete" => {
            if parts.len() > 1 {
                if let Ok(id) = parts[1].parse() {
                    Command::Delete { id }
                } else {
                    Command::Unknown
                }
            } else {
                Command::Unknown
            }
        }
        "save" => Command::Save {
            filename: parts.get(1).unwrap_or(&"tasks.json").to_string(),
        },
        "load" => Command::Load {
            filename: parts.get(1).unwrap_or(&"tasks.json").to_string(),
        },
        "help" => Command::Help,
        "quit" | "exit" => Command::Quit,
        _ => Command::Unknown,
    }
}

fn print_help() {
    println!("\n🔍 Available Commands:");
    println!("{}", "=".repeat(50));
    println!("add         - Add a new task");
    println!("list        - List all tasks");
    println!("list <filter> - List tasks (pending/completed/high/tag)");
    println!("complete <id> - Mark a task as completed");
    println!("delete <id>   - Delete a task");
    println!("save [file]   - Save tasks to file (default: tasks.json)");
    println!("load [file]   - Load tasks from file (default: tasks.json)");
    println!("help         - Show this help message");
    println!("quit         - Exit the program");
    println!("{}", "=".repeat(50));
}

fn main() {
    println!("\n🎯 Welcome to Rust Task Manager!");
    println!("Type 'help' for available commands.");
    
    let mut task_manager = TaskManager::new();
    
    loop {
        print!("\n📌 Enter command: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = parse_command(input.trim());
        
        match command {
            Command::Add { title, description, due_date, tags, priority } => {
                let task = task_manager.add_task(title, description, due_date, tags, priority);
                println!("\n✅ Task added successfully!");
                task_manager.print_task(task);
            }
            Command::List(filter) => task_manager.list_tasks(filter),
            Command::Complete { id } => {
                match task_manager.complete_task(id) {
                    Ok(_) => println!("\n✅ Task {} marked as completed!", id),
                    Err(e) => println!("\n❌ Error: {}", e),
                }
            }
            Command::Delete { id } => {
                match task_manager.delete_task(id) {
                    Ok(_) => println!("\n🗑️  Task {} deleted!", id),
                    Err(e) => println!("\n❌ Error: {}", e),
                }
            }
            Command::Save { filename } => {
                match task_manager.save_to_file(&filename) {
                    Ok(_) => println!("\n💾 Tasks saved to {}", filename),
                    Err(e) => println!("\n❌ Error saving tasks: {}", e),
                }
            }
            Command::Load { filename } => {
                match task_manager.load_from_file(&filename) {
                    Ok(_) => println!("\n📂 Tasks loaded from {}", filename),
                    Err(e) => println!("\n❌ Error loading tasks: {}", e),
                }
            }
            Command::Help => print_help(),
            Command::Quit => {
                println!("\n👋 Goodbye!");
                break;
            }
            Command::Unknown => {
                println!("\n❌ Unknown command. Type 'help' for available commands.");
            }
        }
    }
}