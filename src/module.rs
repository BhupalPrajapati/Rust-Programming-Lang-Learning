// define Module

mod product {
    use category::Category;
    pub struct Product {
        id: u64,
        name: String,
        price: f64,
        category: Category,
    }

    // define category modue
    mod category {
        pub enum Category {
            Electronics,
            Clothing,
            Books,
        }
    }
    impl Product {
        pub fn calculate_tax(&self) -> f64 {
            self.price * 0.1
        }
        pub fn product_price(&self) -> f64 {
            self.price + self.calculate_tax()
        }
    }
}
// Define Module
mod customer {
    pub struct Customer {
        id: i32,
        name: String,
        email: String,
    }
}

// define Module

mod order {
    // use std::iter::Product;

    use crate::customer::Customer;
    use crate::product::Product;
    struct Order {
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
}

fn main() {}
