mod product;
mod inventory;

use crate::inventory::Inventory;

fn main() {
    let mut inventory: Inventory = Inventory::new();

    // Add products
    inventory.add_product(String::from("PS5"), 499.99, 5);
    inventory.add_product(String::from("PS4"), 399.99, 10);

    // Display products
    inventory.display_product(String::from("PS4"));

    // Sell product
    inventory.sell_product(String::from("PS5"), 2);
}
