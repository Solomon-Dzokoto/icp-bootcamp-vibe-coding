# Exercise 1: Module Explorer

## Problem Statement

Create a multi-module Rust project that models a simple e-commerce system with proper code organization. The project should include modules for products, users, orders, and inventory management.

## Learning Objectives

- Practice organizing code using modules and submodules
- Implement proper visibility controls using `pub` and `pub(crate)`
- Use the `use` statement to bring items into scope
- Create a clean API between modules

## Starter Code

```rust
// src/main.rs or lib.rs

// TODO: Define your module structure here
// Hint: You'll need to create modules for products, users, orders, and inventory

fn main() {
    // TODO: Use your modules to create a simple e-commerce workflow:
    // 1. Create some products
    // 2. Add products to inventory
    // 3. Create a user
    // 4. Create an order for the user with some products
    // 5. Print order details
}
```

## How to Run Your Code

1. First, modify the starter code in `01_module_explorer_starter.rs` to fix the ownership issues
2. Run your code from the bootcamp root directory with:
   ```
   cargo run --bin module4_01
   ```

## Expected Structure

Your project should have a module structure similar to this:

```
src/
├── main.rs (or lib.rs)
├── product.rs
├── user.rs
├── order.rs
└── inventory.rs
```

Each module should define appropriate types and functions with proper visibility modifiers.

## Requirements

1. **Product Module**

   - Define a `Product` struct with fields for id, name, price, and description
   - Implement methods for creating and displaying products

2. **User Module**

   - Define a `User` struct with fields for id, name, email, and address
   - Implement methods for creating and displaying users

3. **Order Module**

   - Define an `Order` struct that associates users with purchased products
   - Include order status using an enum
   - Implement methods for creating, updating, and displaying orders

4. **Inventory Module**
   - Define functions to add, remove, and check stock of products
   - Track product quantities

## Tips

- Think carefully about which items should be public and which should be private
- Consider using submodules for related functionality
- Remember that you can use `pub(crate)` to restrict visibility to the current crate
- Use proper paths when referring to items in other modules
