use std::collections::HashMap;

// Represents a student with a name and a list of grades.
#[derive(Debug, Clone)] // Clone can be useful for some operations, though not strictly required by spec.
struct Student {
    name: String,
    grades: Vec<u32>,
}

impl Student {
    // Creates a new student with an empty list of grades.
    fn new(name: &str) -> Self {
        Student {
            name: name.to_string(),
            grades: Vec::new(),
        }
    }

    // Calculates the average of the student's grades.
    // Returns None if the student has no grades.
    fn average_grade(&self) -> Option<f64> {
        if self.grades.is_empty() {
            None
        } else {
            let sum: u32 = self.grades.iter().sum();
            Some(sum as f64 / self.grades.len() as f64)
        }
    }
}

// Manages a collection of students and their grades.
#[derive(Debug)]
struct Gradebook {
    students: HashMap<String, Student>,
}

impl Gradebook {
    // Creates a new, empty gradebook.
    fn new() -> Self {
        Gradebook {
            students: HashMap::new(),
        }
    }

    // Adds a new student to the gradebook.
    // If student with the same name already exists, prints a message and does nothing.
    fn add_student(&mut self, name: &str) {
        if self.students.contains_key(name) {
            println!("Student '{}' already exists.", name);
        } else {
            self.students.insert(name.to_string(), Student::new(name));
            println!("Student '{}' added.", name);
        }
    }

    // Adds a grade to a specific student.
    // If the student does not exist, prints an error message.
    // Grade should be between 0 and 100.
    fn add_grade(&mut self, name: &str, grade: u32) {
        if !(0..=100).contains(&grade) {
            println!("Error: Grade {} for '{}' is invalid. Must be between 0 and 100.", grade, name);
            return;
        }
        match self.students.get_mut(name) {
            Some(student) => {
                student.grades.push(grade);
                println!("Grade {} added for student '{}'.", grade, name);
            }
            None => {
                println!("Error: Student '{}' not found. Cannot add grade.", name);
            }
        }
    }

    // Retrieves the average grade for a specific student.
    // Returns None if the student is not found or has no grades.
    fn get_student_average(&self, name: &str) -> Option<f64> {
        self.students.get(name).and_then(|student| student.average_grade())
    }

    // Calculates the average grade for all students in the gradebook.
    // Returns None if there are no students or no students with grades.
    fn get_class_average(&self) -> Option<f64> {
        let mut total_average_sum = 0.0;
        let mut students_with_grades_count = 0;

        for student in self.students.values() {
            if let Some(avg) = student.average_grade() {
                total_average_sum += avg;
                students_with_grades_count += 1;
            }
        }

        if students_with_grades_count == 0 {
            None
        } else {
            Some(total_average_sum / students_with_grades_count as f64)
        }
    }

    // Retrieves a list of students whose average grade falls within a specified range (inclusive).
    fn get_students_by_grade_range(&self, min: u32, max: u32) -> Vec<&Student> {
        let mut result = Vec::new();
        for student in self.students.values() {
            if let Some(avg) = student.average_grade() {
                // Compare f64 average with u32 min/max by casting avg or min/max.
                // For simplicity, we round avg and cast, or cast min/max to f64.
                // Let's cast min/max to f64 for more precise comparison.
                if avg >= min as f64 && avg <= max as f64 {
                    result.push(student);
                }
            }
        }
        result
    }
}

fn main() {
    println!("--- Student Gradebook System ---");

    // Create a new gradebook
    let mut gradebook = Gradebook::new();

    // Add students
    println!("\n--- Adding Students ---");
    gradebook.add_student("Alice");
    gradebook.add_student("Bob");
    gradebook.add_student("Charlie");
    gradebook.add_student("Alice"); // Try adding duplicate

    // Add grades
    println!("\n--- Adding Grades ---");
    gradebook.add_grade("Alice", 90);
    gradebook.add_grade("Alice", 85);
    gradebook.add_grade("Alice", 105); // Invalid grade
    gradebook.add_grade("Bob", 78);
    gradebook.add_grade("Bob", 82);
    gradebook.add_grade("Charlie", 95);
    gradebook.add_grade("Charlie", 88);
    gradebook.add_grade("David", 70); // Student not found

    // Get student averages
    println!("\n--- Student Averages ---");
    for name in ["Alice", "Bob", "Charlie", "David"].iter() {
        match gradebook.get_student_average(name) {
            Some(avg) => println!("Average grade for {}: {:.2}", name, avg),
            None => println!("Could not calculate average for {} (not found or no grades).", name),
        }
    }
     // Alice: (90+85)/2 = 87.5
     // Bob: (78+82)/2 = 80.0
     // Charlie: (95+88)/2 = 91.5

    // Get class average
    println!("\n--- Class Average ---");
    match gradebook.get_class_average() {
        Some(avg) => println!("Class average grade: {:.2}", avg), // (87.5 + 80.0 + 91.5) / 3 = 86.33
        None => println!("Could not calculate class average (no students or no grades)."),
    }

    // Get students by grade range
    println!("\n--- Students by Grade Range (80-90) ---");
    let students_in_range = gradebook.get_students_by_grade_range(80, 90);
    if students_in_range.is_empty() {
        println!("No students found in the 80-90 average grade range.");
    } else {
        println!("Students with average grade between 80 and 90:");
        for student in students_in_range {
            println!("- {} (Average: {:.2})", student.name, student.average_grade().unwrap_or(0.0));
        }
    }
    // Expected: Alice (87.5), Bob (80.0)

    println!("\n--- Students by Grade Range (90-100) ---");
    let students_high_range = gradebook.get_students_by_grade_range(90, 100);
    if students_high_range.is_empty() {
        println!("No students found in the 90-100 average grade range.");
    } else {
        println!("Students with average grade between 90 and 100:");
        for student in students_high_range {
             println!("- {} (Average: {:.2})", student.name, student.average_grade().unwrap_or(0.0));
        }
    }
    // Expected: Charlie (91.5)

    println!("\n--- Gradebook operations complete ---");
}