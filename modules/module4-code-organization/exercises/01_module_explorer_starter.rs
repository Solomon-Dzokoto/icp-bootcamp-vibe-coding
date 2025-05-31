// Module for utility functions
mod utils {
    // Formats a string with a given text label and a floating-point value, rounded to 2 decimal places.
    pub fn format_output(text: &str, value: f64) -> String {
        format!("{}: {:.2}", text, value)
    }
}

// Module for geometric shapes
mod shapes {
    // Bring the utility function into the scope of the shapes module.
    // This allows us to use format_output without prefixing it with utils::
    // For this exercise, we choose to use fully qualified paths or `super::utils` if preferred.
    // use super::utils; // Option 1: use super to access parent's sibling module
    // use crate::utils; // Option 2: use crate if utils is at the crate root (which it is here)

    // Represents a Rectangle with public width and height.
    #[derive(Debug)]
    pub struct Rectangle {
        pub width: f64,
        pub height: f64,
    }

    impl Rectangle {
        // Calculates the area of the rectangle.
        pub fn area(&self) -> f64 {
            self.width * self.height
        }
    }

    // Represents a Circle with a public radius.
    #[derive(Debug)]
    pub struct Circle {
        pub radius: f64,
    }

    impl Circle {
        // Calculates the area of the circle.
        pub fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }

    // Public enum to represent different kinds of shapes.
    #[derive(Debug)]
    pub enum Shape {
        Rect(Rectangle), // Variant for Rectangle
        Circ(Circle),    // Variant for Circle
    }

    // Public function to print information about a shape, including its type and area.
    pub fn print_shape_info(shape: &Shape) {
        match shape {
            Shape::Rect(r) => {
                // Use the utility function from the utils module (sibling to shapes).
                let output = crate::utils::format_output("Rectangle Area", r.area());
                println!("Shape: Rectangle (Width: {}, Height: {}) - {}", r.width, r.height, output);
            }
            Shape::Circ(c) => {
                // Use the utility function.
                let output = crate::utils::format_output("Circle Area", c.area());
                println!("Shape: Circle (Radius: {}) - {}", c.radius, output);
            }
        }
    }
}

// Main function to demonstrate the use of the shapes and utils modules.
fn main() {
    println!("--- Module Explorer Demonstration ---");

    // Create a Rectangle instance using the shapes module.
    let rect = shapes::Rectangle { width: 10.5, height: 5.2 };
    
    // Create a Circle instance using the shapes module.
    let circ = shapes::Circle { radius: 7.0 };

    // Create Shape enum variants from the shapes module.
    let shape_rect = shapes::Shape::Rect(rect);
    let shape_circ = shapes::Shape::Circ(circ);

    // Print information about the shapes using the function from the shapes module.
    println!("\n--- Using shapes::print_shape_info ---");
    shapes::print_shape_info(&shape_rect);
    shapes::print_shape_info(&shape_circ);

    // Demonstrate direct use of the utils::format_output function.
    println!("\n--- Direct use of utils::format_output ---");
    let custom_value = 123.4567;
    // utils is a sibling module to main (since main is at crate root and utils is defined at crate root)
    let formatted_string = utils::format_output("Custom Value", custom_value);
    println!("{}", formatted_string);

    // Example: Accessing fields of Rectangle and Circle directly (since they are public)
    // This is just to show fields are accessible as per subtask "Make fields public"
    // For shape_rect and shape_circ, we need to match to get the inner struct first.
    match shape_rect {
        shapes::Shape::Rect(ref r) => { // Use `ref r` to borrow the Rectangle inside the enum
             println!("\nAccessed Rectangle directly: width = {}, height = {}", r.width, r.height);
        },
        _ => {} // Should not happen in this specific main flow, but good for completeness
    }
     match shape_circ {
        shapes::Shape::Circ(ref c) => { // Use `ref c` to borrow the Circle
             println!("Accessed Circle directly: radius = {}", c.radius);
        },
        _ => {}
    }
}