// use crate::customer::Customer;
pub use::{Order,Product,Customer};


pub struct Order {
    id: u32,
    product: Product,
    customer: Customer,
    quantity: u32,
}

impl Order {
    fn calculate_discount(&self) -> f64 {
        if self.quantity > 5 {
            0.1
        } else {
            0.0
        }
    }
    fn total_bils(&self) -> f64 {
        let discount = self.calculate_discount();
        let tota_before_discount = self.product.product_price() * self.quantity as f64;
        tota_before_discount - (tota_before_discount * discount)
    }
}