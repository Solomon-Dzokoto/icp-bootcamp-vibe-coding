# Exercise 1: Trait Workshop

## Problem Statement

Design and implement a shape calculation system using traits. The system should be able to calculate area and perimeter for different geometric shapes, including circles, rectangles, and triangles.

## Learning Objectives

- Practice defining and implementing traits
- Use trait bounds to create flexible interfaces
- Implement default trait methods
- Work with trait objects for runtime polymorphism

## Starter Code

```rust
use std::f64::consts::PI;

// Define the Shape trait with methods for area and perimeter calculation
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;

    // Default method to return the shape's name
    fn name(&self) -> &str {
        "Unknown Shape"
    }
}

// Implement Circle
struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Self {
        Self { radius }
    }
}

impl Shape for Circle {
    // TODO: Implement area method for Circle
    fn area(&self) -> f64 {
        // Hint: The area of a circle is PI * r²
        0.0 // Replace with correct implementation
    }

    // TODO: Implement perimeter method for Circle
    fn perimeter(&self) -> f64 {
        // Hint: The perimeter (circumference) of a circle is 2 * PI * r
        0.0 // Replace with correct implementation
    }

    // TODO: Override the name method to return "Circle"
    fn name(&self) -> &str {
        "Unknown Shape" // Replace with correct implementation
    }
}

// Implement Rectangle
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

// TODO: Implement the Shape trait for Rectangle
impl Shape for Rectangle {
    // Implement area method
    fn area(&self) -> f64 {
        0.0 // Replace with correct implementation
    }

    // Implement perimeter method
    fn perimeter(&self) -> f64 {
        0.0 // Replace with correct implementation
    }

    // Override name method
    fn name(&self) -> &str {
        "Unknown Shape" // Replace with correct implementation
    }
}

// Implement Triangle
struct Triangle {
    side_a: f64,
    side_b: f64,
    side_c: f64,
}

impl Triangle {
    // Constructor that validates if the sides can form a valid triangle
    fn new(side_a: f64, side_b: f64, side_c: f64) -> Option<Self> {
        // TODO: Check if the sides can form a valid triangle
        // Hint: In a valid triangle, the sum of any two sides must be greater than the third side

        Some(Self { side_a, side_b, side_c }) // Replace with validation logic
    }
}

// TODO: Implement the Shape trait for Triangle
impl Shape for Triangle {
    // Implement area method
    // Hint: You can use Heron's formula:
    // Let s = (a + b + c) / 2
    // Area = √(s * (s - a) * (s - b) * (s - c))
    fn area(&self) -> f64 {
        0.0 // Replace with correct implementation
    }

    // Implement perimeter method
    fn perimeter(&self) -> f64 {
        0.0 // Replace with correct implementation
    }

    // Override name method
    fn name(&self) -> &str {
        "Unknown Shape" // Replace with correct implementation
    }
}

// Function to print shape information using trait bounds
fn print_shape_info<T: Shape>(shape: &T) {
    // TODO: Implement this function to print the shape's name, area, and perimeter
    println!("Shape: {}", shape.name());
    println!("Area: {:.2}", shape.area());
    println!("Perimeter: {:.2}", shape.perimeter());
    println!();
}

fn main() {
    // Create instances of each shape
    let circle = Circle::new(5.0);
    // TODO: Create a rectangle with width 3.0 and height 5.0

    // TODO: Create a triangle with sides 3.0, 4.0, and 5.0
    // Remember to handle the Option return type

    // TODO: Print information for each shape using print_shape_info
    print_shape_info(&circle);
    // Add calls for rectangle and triangle

    // TODO: Store shapes in a vector of trait objects and iterate through them
    // Hint: You'll need to use Box<dyn Shape> to store different shapes in the same vector
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(circle),
        // Add the rectangle and triangle to this vector
    ];

    println!("Shapes in collection:");
    // TODO: Iterate through shapes and print information for each

    // Bonus: Calculate the total area of all shapes
    // TODO: Use iterator methods to sum the areas of all shapes
}
```

## How to Run Your Code

1. First, modify the starter code in `01_trait_workshop_starter.rs` according to the requirements
2. Run your code from the bootcamp root directory with:
   ```
   cargo run --bin module6_01
   ```

## Expected Output

Your output should be similar to:

```
Shape: Circle
Area: 78.54
Perimeter: 31.42

Shape: Rectangle
Area: 15.00
Perimeter: 16.00

Shape: Triangle
Area: 6.00
Perimeter: 12.00

Shapes in collection:
Circle - Area: 78.54, Perimeter: 31.42
Rectangle - Area: 15.00, Perimeter: 16.00
Triangle - Area: 6.00, Perimeter: 12.00
```

## Tips

- Use the `f64` type for floating-point calculations
- Remember to implement the trait for each shape type
- Consider adding a constructor method (`new`) for each shape
- For the triangle, you can use Heron's formula to calculate the area
- Round the output to 2 decimal places for clarity
- Use `Box<dyn Shape>` to create a collection of trait objects
