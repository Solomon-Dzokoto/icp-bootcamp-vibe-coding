use std::f64::consts::PI;
use std::fmt;

// Define the Shape trait with methods for area and perimeter calculation
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    
    fn name(&self) -> &str {
        "Unknown Shape"
    }

    // Add a method to check if the shape is valid
    fn is_valid(&self) -> bool {
        true
    }

    // Add a method to get shape properties as a formatted string
    fn properties(&self) -> String {
        format!("Shape: {}\nArea: {:.2}\nPerimeter: {:.2}",
                self.name(), self.area(), self.perimeter())
    }
}

// Implement Circle
#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Option<Self> {
        if radius <= 0.0 {
            None
        } else {
            Some(Self { radius })
        }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
    
    fn name(&self) -> &str {
        "Circle"
    }

    fn is_valid(&self) -> bool {
        self.radius > 0.0
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle(radius={})", self.radius)
    }
}

// Implement Rectangle
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Option<Self> {
        if width <= 0.0 || height <= 0.0 {
            None
        } else {
            Some(Self { width, height })
        }
    }

    // Additional method to check if rectangle is a square
    fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }
}

// Implement the Shape trait for Rectangle
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
    
    fn name(&self) -> &str {
        if self.is_square() {
            "Square"
        } else {
            "Rectangle"
        }
    }

    fn is_valid(&self) -> bool {
        self.width > 0.0 && self.height > 0.0
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(width={}, height={})", 
            self.name(), self.width, self.height)
    }
}

// Implement Triangle
#[derive(Debug)]
struct Triangle {
    side_a: f64,
    side_b: f64,
    side_c: f64,
}

impl Triangle {
    fn new(side_a: f64, side_b: f64, side_c: f64) -> Option<Self> {
        let triangle = Self { side_a, side_b, side_c };
        if triangle.is_valid() {
            Some(triangle)
        } else {
            None
        }
    }

    fn triangle_type(&self) -> &str {
        let sides = [self.side_a, self.side_b, self.side_c];
        let mut equal_sides = 0;
        
        for i in 0..3 {
            for j in i+1..3 {
                if (sides[i] - sides[j]).abs() < f64::EPSILON {
                    equal_sides += 1;
                }
            }
        }

        match equal_sides {
            3 => "Equilateral",
            1 => "Isosceles",
            _ => "Scalene"
        }
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        // Heron's formula
        let s = self.perimeter() / 2.0;
        (s * (s - self.side_a) * (s - self.side_b) * (s - self.side_c)).sqrt()
    }
    
    fn perimeter(&self) -> f64 {
        self.side_a + self.side_b + self.side_c
    }
    
    fn name(&self) -> &str {
        match self.triangle_type() {
            "Equilateral" => "Equilateral Triangle",
            "Isosceles" => "Isosceles Triangle",
            "Scalene" => "Scalene Triangle",
            _ => "Triangle"
        }
    }

    fn is_valid(&self) -> bool {
        // Triangle inequality theorem
        self.side_a > 0.0 && self.side_b > 0.0 && self.side_c > 0.0 &&
        self.side_a + self.side_b > self.side_c &&
        self.side_b + self.side_c > self.side_a &&
        self.side_a + self.side_c > self.side_b
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(sides={}, {}, {})", 
            self.name(), self.side_a, self.side_b, self.side_c)
    }
}

// Function to print shape information using trait bounds
fn print_shape_info<T: Shape + fmt::Display>(shape: &T) {
    println!("\n🔷 {}", shape);
    println!("├─ Area: {:.2}", shape.area());
    println!("├─ Perimeter: {:.2}", shape.perimeter());
    println!("└─ Valid: {}", if shape.is_valid() { "✓" } else { "✗" });
}

fn main() {
    println!("🎨 Shape Calculator\n");

    // Create and test Circle
    println!("Circle Examples:");
    if let Some(circle) = Circle::new(5.0) {
        print_shape_info(&circle);
    }
    // Test invalid circle
    match Circle::new(-1.0) {
        Some(_) => println!("Unexpected: Created invalid circle"),
        None => println!("✓ Correctly rejected invalid circle (negative radius)")
    }

    // Create and test Rectangle/Square
    println!("\nRectangle Examples:");
    if let Some(rectangle) = Rectangle::new(4.0, 6.0) {
        print_shape_info(&rectangle);
    }
    if let Some(square) = Rectangle::new(5.0, 5.0) {
        print_shape_info(&square);
    }

    // Create and test different types of triangles
    println!("\nTriangle Examples:");
    // Equilateral triangle
    if let Some(equilateral) = Triangle::new(5.0, 5.0, 5.0) {
        print_shape_info(&equilateral);
    }
    // Right triangle (3-4-5 triangle)
    if let Some(right) = Triangle::new(3.0, 4.0, 5.0) {
        print_shape_info(&right);
    }
    // Invalid triangle
    match Triangle::new(1.0, 1.0, 10.0) {
        Some(_) => println!("Unexpected: Created invalid triangle"),
        None => println!("✓ Correctly rejected invalid triangle (violates triangle inequality)")
    }

    // Create a collection of shapes and calculate total area
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle::new(5.0).unwrap()),
        Box::new(Rectangle::new(4.0, 6.0).unwrap()),
        Box::new(Triangle::new(3.0, 4.0, 5.0).unwrap()),
    ];

    println!("\n📊 Shape Collection Statistics:");
    println!("Number of shapes: {}", shapes.len());
    
    let total_area: f64 = shapes.iter().map(|s| s.area()).sum();
    let total_perimeter: f64 = shapes.iter().map(|s| s.perimeter()).sum();
    
    println!("Total area: {:.2}", total_area);
    println!("Total perimeter: {:.2}", total_perimeter);
    println!("Average area: {:.2}", total_area / shapes.len() as f64);
}