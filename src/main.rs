use std::collections::HashMap;
use std::io::{self, Write};

// Struct to represent a product
#[derive(Debug)]
struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: u32,
}

impl Product {
    fn new(name: String, description: String, price: f64, quantity: u32) -> Self {
        Product {
            name,
            description,
            price,
            quantity,
        }
    }
}

// Struct to represent the inventory management system
struct Inventory {
    products: HashMap<String, Product>,
}

impl Inventory {
    fn new() -> Self {
        Inventory {
            products: HashMap::new(),
        }
    }

    // Add a product to the inventory
    fn add_product(&mut self, name: String, description: String, price: f64, quantity: u32) {
        if self.products.contains_key(&name) {
            println!("Product with the same name already exists.");
        } else {
            let product = Product::new(name.clone(), description, price, quantity);
            self.products.insert(name, product);
            println!("Product added successfully.");
        }
    }

    // Edit a product in the inventory
    fn edit_product(&mut self, name: &str, description: String, price: f64, quantity: u32) {
        if let Some(product) = self.products.get_mut(name) {
            product.description = description;
            product.price = price;
            product.quantity = quantity;
            println!("Product edited successfully.");
        } else {
            println!("Product not found.");
        }
    }

    // Delete a product from the inventory
    fn delete_product(&mut self, name: &str) {
        if let Some(_) = self.products.remove(name) {
            println!("Product deleted successfully.");
        } else {
            println!("Product not found.");
        }
    }

    // Generate report of the inventory
    fn generate_report(&self) {
        if self.products.is_empty() {
            println!("Inventory is empty.");
        } else {
            println!("Inventory Report:");
            println!("-----------------");
            println!(
                "{:<20} {:<20} {:<10} {:<10}",
                "Name", "Description", "Price", "Quantity"
            );
            for (_, product) in &self.products {
                println!(
                    "{:<20} {:<20} {:<10.2} {:<10}",
                    product.name, product.description, product.price, product.quantity
                );
            }
        }
    }
}

fn main() {
    let mut inventory = Inventory::new();

    loop {
        println!("\nMenu:");
        println!("1. Add Product");
        println!("2. Edit Product");
        println!("3. Delete Product");
        println!("4. Generate Report");
        println!("5. Exit");
        print!("Please enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice. Please enter a number between 1 and 5.");
                continue;
            }
        };

        match choice {
            1 => {
                // Add Product
                println!("Enter product details:");
                print!("Name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read input");
                print!("Description: ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin()
                    .read_line(&mut description)
                    .expect("Failed to read input");
                print!("Price: ");
                io::stdout().flush().unwrap();
                let mut price = String::new();
                io::stdin()
                    .read_line(&mut price)
                    .expect("Failed to read input");
                let price: f64 = match price.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input for price. Please enter a valid number.");
                        continue;
                    }
                };
                print!("Quantity: ");
                io::stdout().flush().unwrap();
                let mut quantity = String::new();
                io::stdin()
                    .read_line(&mut quantity)
                    .expect("Failed to read input");
                let quantity: u32 = match quantity.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input for quantity. Please enter a valid number.");
                        continue;
                    }
                };
                inventory.add_product(
                    name.trim().to_string(),
                    description.trim().to_string(),
                    price,
                    quantity,
                );
            }
            2 => {
                // Edit Product
                println!("Enter product name to edit:");
                print!("Name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read input");
                println!("Enter new product details:");
                print!("Description: ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin()
                    .read_line(&mut description)
                    .expect("Failed to read input");
                print!("Price: ");
                io::stdout().flush().unwrap();
                let mut price = String::new();
                io::stdin()
                    .read_line(&mut price)
                    .expect("Failed to read input");
                let price: f64 = match price.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input for price. Please enter a valid number.");
                        continue;
                    }
                };
                print!("Quantity: ");
                io::stdout().flush().unwrap();
                let mut quantity = String::new();
                io::stdin()
                    .read_line(&mut quantity)
                    .expect("Failed to read input");
                let quantity: u32 = match quantity.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input for quantity. Please enter a valid number.");
                        continue;
                    }
                };
                inventory.edit_product(
                    &name.trim(),
                    description.trim().to_string(),
                    price,
                    quantity,
                );
            }
            3 => {
                // Delete Product
                println!("Enter product name to delete:");
                print!("Name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read input");
                inventory.delete_product(&name.trim());
            }
            4 => {
                // Generate Report
                inventory.generate_report();
            }
            5 => {
                // Exit
                println!("Exiting program.");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a number between 1 and 5.");
            }
        }
    }
}
