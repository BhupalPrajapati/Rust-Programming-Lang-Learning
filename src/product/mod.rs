pub use Product::prduct;
pub struct Product {
    id: u64,
    name: String,
    price: f64,
    category: Category,
}
impl Product {
    pub fn new(id: u64, name: String, price: f64, category:Category) -> Product {
        Product {
            id,
            name,
            price,
            category,
        }
    }
}

// define category modue
mod category;
impl Product {
    pub fn calculate_tax(&self) -> f64 {
        self.price * 0.1
    }
    pub fn product_price(&self) -> f64 {
        self.price + self.calculate_tax()
    }
}
