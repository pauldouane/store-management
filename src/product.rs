pub(crate) struct Product {
    pub(crate) name: String,
    price: f64,
    pub(crate) quantity: u32
}

impl Product {
    pub(crate) fn new(name: String, price: f64, quantity: u32) -> Product {
        Product {
            name, price, quantity
        }
    }

    pub(crate) fn display(&self) {
        println!("Name : {} / Price : {} / Quantity : {}",
                 self.name, self.price, self.quantity)
    }
}