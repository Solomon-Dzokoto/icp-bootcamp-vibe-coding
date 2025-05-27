use std::num::ParseIntError;
use std::fmt;

/// Part 1: Configuration Parser
/// This module demonstrates how to handle configuration parsing with custom errors
/// and proper error propagation using the Result type.

#[derive(Debug)]
struct Config {
    username: String,
    timeout: u32,
    max_retries: u32,
}

/// Custom error type for configuration parsing
/// Provides specific error variants for different types of configuration failures
#[derive(Debug)]
enum ConfigError {
    MissingField(String),    // When a required field is not provided
    InvalidTimeout(String),   // When timeout value cannot be parsed
    InvalidRetryCount(String) // When retry count value cannot be parsed
}

// Display implementation for ConfigError - just needs message content
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::MissingField(field) => write!(f, "Missing required field: {}", field),
            ConfigError::InvalidTimeout(val) => write!(f, "Invalid timeout value: {}", val),
            ConfigError::InvalidRetryCount(val) => write!(f, "Invalid retry count: {}", val),
        }
    }
}

/// Parse configuration string function - implementation needed
fn parse_config(config_str: &str) -> Result<Config, ConfigError> {
    let mut username = None;
    let mut timeout = None;
    let mut max_retries = None;

    // Split the configuration string by commas and process each key-value pair
    for pair in config_str.split(',') {
        let parts: Vec<&str> = pair.split('=').collect();
        if parts.len() != 2 {
            continue; // Skip invalid pairs
        }

        let key = parts[0].trim();
        let value = parts[1].trim();

        match key {
            "username" => username = Some(value.to_string()),
            "timeout" => {
                match value.parse::<u32>() {
                    Ok(val) => timeout = Some(val),
                    Err(_) => return Err(ConfigError::InvalidTimeout(value.to_string())),
                }
            },
            "max_retries" => {
                match value.parse::<u32>() {
                    Ok(val) => max_retries = Some(val),
                    Err(_) => return Err(ConfigError::InvalidRetryCount(value.to_string())),
                }
            },
            _ => {} // Ignore unknown keys
        }
    }

    let username = username.ok_or(ConfigError::MissingField("username".to_string()))?;
    let timeout = timeout.ok_or(ConfigError::MissingField("timeout".to_string()))?;
    let max_retries = max_retries.ok_or(ConfigError::MissingField("max_retries".to_string()))?;

    Ok(Config {
        username,
        timeout,
        max_retries,
    })
}

/// Part 2: Safe String to Integer Conversion
/// This module provides a function to safely convert a string to an integer
/// with error handling for invalid inputs.

/// Convert a string slice to an optional i32
/// Returns Some(i32) if successful, None if parsing fails
fn parse_number(s: &str) -> Option<i32> {
    match s.parse::<i32>() {
        Ok(number) => Some(number),
        Err(_) => None,
    }
}

/// Part 3: Data Validation with Custom Errors
/// This module implements a user data validator with detailed error reporting
/// for common data issues like invalid ID, short name, or underage users.

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    age: u32,
}

/// ValidationError enum - already defined with variants
#[derive(Debug)]
enum ValidationError {
    InvalidId,
    NameTooShort,
    InvalidAge,
}

// Display implementation for ValidationError
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::InvalidId => write!(f, "User ID cannot be zero - please provide a valid ID"),
            ValidationError::NameTooShort => write!(f, "Name is too short - must be at least 2 characters long"),
            ValidationError::InvalidAge => write!(f, "User must be at least 18 years old to register"),
        }
    }
}

/// Validate user function - with proper error handling
fn validate_user(user: &User) -> Result<(), ValidationError> {
    // Check ID validity
    if user.id == 0 {
        return Err(ValidationError::InvalidId);
    }
    
    if user.name.len() < 2 {
        return Err(ValidationError::NameTooShort);
    }
    
    if user.age < 18 {
        return Err(ValidationError::InvalidAge);
    }
    
    Ok(())
}

/// Part 4: Error Propagation Chain
/// This module demonstrates error propagation using the ? operator in a series of functions
/// that process configuration and user data.

#[derive(Debug)]
enum ProcessError {
    ConfigError(ConfigError),
    ParseError(ParseIntError),
    ValidationError(ValidationError),
}

// From implementations for automatic conversions
impl From<ConfigError> for ProcessError {
    fn from(err: ConfigError) -> Self {
        ProcessError::ConfigError(err)
    }
}

impl From<ParseIntError> for ProcessError {
    fn from(err: ParseIntError) -> Self {
        ProcessError::ParseError(err)
    }
}

impl From<ValidationError> for ProcessError {
    fn from(err: ValidationError) -> Self {
        ProcessError::ValidationError(err)
    }
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProcessError::ConfigError(e) => write!(f, "Configuration error: {}", e),
            ProcessError::ParseError(e) => write!(f, "Parse error: {}", e),
            ProcessError::ValidationError(e) => write!(f, "Validation error: {}", e),
        }
    }
}

/// Process data function - needs implementation with ? operator
fn process_data(config_str: &str, user_id: &str, user_name: &str, user_age: &str) -> Result<(), ProcessError> {
    let config = parse_config(config_str)?;
    
    let id: u32 = user_id.parse()?;
    let age: u32 = user_age.parse()?;
    
    let user = User {
        id,
        name: user_name.to_string(),
        age,
    };
    
    validate_user(&user)?;
    
    Ok(())
}

fn main() {
    println!("\x1b[1;36m=== Error Handling Workshop ===\x1b[0m\n");

    // Part 1: Test the configuration parser
    println!("\x1b[1;33m--- Part 1: Configuration Parser ---\x1b[0m");
    let config_str = "username=alice,timeout=30,max_retries=5";
    println!("Testing valid config: {}", config_str);
    match parse_config(config_str) {
        Ok(config) => println!("\x1b[1;32m✓ Config parsed successfully: {:?}\x1b[0m", config),
        Err(e) => println!("\x1b[1;31m✗ Failed to parse config: {}\x1b[0m", e),
    }
    
    let invalid_config = "username=bob,timeout=invalid,max_retries=5";
    println!("\nTesting invalid config: {}", invalid_config);
    match parse_config(invalid_config) {
        Ok(config) => println!("\x1b[1;32m✓ Config parsed successfully: {:?}\x1b[0m", config),
        Err(e) => println!("\x1b[1;31m✗ Failed to parse config: {}\x1b[0m", e),
    }
    
    // Part 2: Test the string to integer conversion
    println!("\n\x1b[1;33m--- Part 2: String to Integer Conversion ---\x1b[0m");
    for test_case in [("42", "valid"), ("four", "invalid")] {
        println!("\nTesting {} number: '{}'", test_case.1, test_case.0);
        match parse_number(test_case.0) {
            Some(num) => println!("\x1b[1;32m✓ Successfully parsed to {}\x1b[0m", num),
            None => println!("\x1b[1;31m✗ Failed to parse number\x1b[0m"),
        }
    }
    
    // Part 3: Test the user validation
    println!("\n\x1b[1;33m--- Part 3: User Validation ---\x1b[0m");
    let test_users = [
        User { id: 1001, name: String::from("Charlie"), age: 25 },
        User { id: 1002, name: String::from("D"), age: 30 },
        User { id: 1003, name: String::from("Eve"), age: 17 },
    ];
    
    for user in &test_users {
        println!("\nValidating user: {:?}", user);
        match validate_user(user) {
            Ok(()) => println!("\x1b[1;32m✓ User is valid\x1b[0m"),
            Err(e) => println!("\x1b[1;31m✗ Validation error: {}\x1b[0m", e),
        }
    }
    
    // Part 4: Test the error propagation chain
    println!("\n\x1b[1;33m--- Part 4: Error Propagation ---\x1b[0m");
    let test_cases = [
        ("username=charlie,timeout=30,max_retries=5", "1001", "Charlie", "25"),
        ("username=diana,timeout=invalid,max_retries=5", "1002", "Diana", "30"),
        ("username=eve,timeout=30,max_retries=5", "invalid_id", "Eve", "22"),
        ("username=frank,timeout=30,max_retries=5", "1004", "F", "17"),
    ];
    
    for (config, id, name, age) in test_cases.iter() {
        println!("\nProcessing case:");
        println!("  Config: {}", config);
        println!("  ID: {}, Name: {}, Age: {}", id, name, age);
        match process_data(config, id, name, age) {
            Ok(()) => println!("\x1b[1;32m✓ Success: Data processed successfully\x1b[0m"),
            Err(e) => println!("\x1b[1;31m✗ Error: {}\x1b[0m", e),
        }
    }
    
    println!("\n\x1b[1;36m=== Workshop Complete ===\x1b[0m");
}