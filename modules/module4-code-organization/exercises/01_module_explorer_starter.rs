use std::collections::HashMap;

// Product Module
mod product {
    #[derive(Debug, Clone)]
    pub struct Product {
        pub id: u32,
        pub name: String,
        pub price: f64,
        pub description: String,
    }

    impl Product {
        pub fn new(id: u32, name: &str, price: f64, description: &str) -> Self {
            Product {
                id,
                name: name.to_string(),
                price,
                description: description.to_string(),
            }
        }

        pub fn display(&self) -> String {
            format!("{} - ${:.2} ({})", self.name, self.price, self.description)
        }
    }
}

// Inventory Module
mod inventory {
    use super::product::Product;
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct Inventory {
        products: HashMap<u32, (Product, u32)>, // Product ID -> (Product, Quantity)
    }

    impl Inventory {
        pub fn new() -> Self {
            Inventory {
                products: HashMap::new(),
            }
        }

        pub fn add_product(&mut self, product: Product, quantity: u32) {
            self.products.insert(product.id, (product, quantity));
        }

        pub fn get_product(&self, id: u32) -> Option<&Product> {
            self.products.get(&id).map(|(product, _)| product)
        }

        pub fn update_quantity(&mut self, id: u32, quantity: i32) -> Result<u32, &'static str> {
            if let Some((_, current_quantity)) = self.products.get_mut(&id) {
                let new_quantity = *current_quantity as i32 + quantity;
                if new_quantity < 0 {
                    return Err("Insufficient stock");
                }
                *current_quantity = new_quantity as u32;
                Ok(*current_quantity)
            } else {
                Err("Product not found")
            }
        }

        pub fn list_products(&self) {
            println!("\n📦 Current Inventory:");
            println!("{}", "=".repeat(50));
            for (_, (product, quantity)) in &self.products {
                println!("• {} (Stock: {})", product.display(), quantity);
            }
            println!("{}", "=".repeat(50));
        }
    }
}

// User Module
mod user {
    #[derive(Debug)]
    pub struct User {
        pub id: u32,
        pub name: String,
        pub email: String,
    }

    impl User {
        pub fn new(id: u32, name: &str, email: &str) -> Self {
            User {
                id,
                name: name.to_string(),
                email: email.to_string(),
            }
        }
    }
}

// Order Module
mod order {
    use super::product::Product;
    use super::user::User;
    use chrono::Local;

    #[derive(Debug)]
    pub struct OrderItem {
        pub product: Product,
        pub quantity: u32,
    }

    #[derive(Debug)]
    pub struct Order {
        pub id: u32,
        pub user: User,
        pub items: Vec<OrderItem>,
        pub date: String,
        pub total: f64,
    }

    impl Order {
        pub fn new(id: u32, user: User) -> Self {
            Order {
                id,
                user,
                items: Vec::new(),
                date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                total: 0.0,
            }
        }

        pub fn add_item(&mut self, product: Product, quantity: u32) {
            self.total += product.price * quantity as f64;
            self.items.push(OrderItem { product, quantity });
        }

        pub fn display(&self) {
            println!("\n🛍️  Order Details");
            println!("{}", "=".repeat(50));
            println!("Order ID: #{}", self.id);
            println!("Customer: {} ({})", self.user.name, self.user.email);
            println!("Date: {}", self.date);
            println!("\nItems:");
            for item in &self.items {
                println!("• {}x {}", item.quantity, item.product.display());
            }
            println!("{}", "-".repeat(30));
            println!("Total: ${:.2}", self.total);
            println!("{}", "=".repeat(50));
        }
    }
}

use product::Product;
use inventory::Inventory;
use user::User;
use order::Order;

fn main() {
    // Initialize inventory
    let mut inventory = Inventory::new();

    // Create products
    let products = [
        Product::new(1, "Mechanical Keyboard", 149.99, "RGB backlit, Cherry MX switches"),
        Product::new(2, "Wireless Mouse", 79.99, "Ultra-lightweight gaming mouse"),
        Product::new(3, "4K Monitor", 399.99, "27-inch, HDR support"),
        Product::new(4, "USB-C Hub", 45.99, "7-in-1 adapter"),
    ];

    // Add products to inventory
    for product in &products {
        inventory.add_product(product.clone(), 10);
    }

    // Display initial inventory
    inventory.list_products();

    // Create a user
    let user = User::new(1, "Alice Johnson", "alice@example.com");

    // Create and process an order
    let mut order = Order::new(1, user);
    
    // Add items to order
    order.add_item(products[0].clone(), 1); // Keyboard
    order.add_item(products[1].clone(), 2); // Two mice

    // Update inventory
    inventory.update_quantity(1, -1).expect("Failed to update keyboard stock");
    inventory.update_quantity(2, -2).expect("Failed to update mouse stock");

    // Display order details
    order.display();

    // Show updated inventory
    inventory.list_products();
}