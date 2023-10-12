use crate::product::Product;

pub(crate) struct Inventory {
    products: Vec<Product>
}

impl Inventory {
    pub(crate) fn new() -> Self {
        Inventory {
            products: Vec::new(),
        }
    }

    pub(crate) fn add_product(&mut self, name: String, price: f64, quantity: u32) {
        self.products.push(Product::new(name, price, quantity))
    }

    pub(crate) fn get_product(&mut self, name: String) -> Result<&mut Product, &'static str> {
        if let Some(product) = self.products.iter_mut().find(|p| p.name == name) {
            Ok(product)
        } else {
            Err("Error: Product not found")
        }
    }

    pub(crate) fn display_product(&mut self, name: String){
        self.get_product(name).unwrap().display();
    }

    pub(crate) fn sell_product(&mut self, name: String, quantity: u32) {
        let product: &mut Product = self.get_product(name).unwrap();
        if (product.quantity - quantity) >= 0 {
            *product.quantity -= quantity;
            println!("Successful sale: {} {} sold. New quantity in stock: {}", product.name, quantity, product.quantity)
        } else {
            println!("Error: There is not enough in stock")
        }
    }
}