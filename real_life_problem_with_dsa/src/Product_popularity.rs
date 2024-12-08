// in this section we are planing to find the popularity of a product in a company. for eg a company have various product
// and they want to analayse the popularity of product, they set the popularity parameter like customer feedvack
// and rating. according to popularity they are apply the own statrages for that product and managae tham

//popularity scores
//Description
// Given some product along with the its respecitively popularity scores
// we want to determine if popularity is falcuting, increasing or decreasing

// Sollution
// using hashmap, the key is the product name and value is the vector containing the
// popularity scores

use std::collections::HashMap;

fn popularity_analysis(scores: Vec<i32>) -> bool {
    // ouput od this function is bool, bcz popularity is in true or false(increasing or decreasing)
    let mut increasing = true;
    let mut decreasing = true;
    for i in 0..scores.len() - 1 {
        // interating through the certain product
        // popularity is increasing means , the product are in acemding order . the nxt value>previous

        if scores[i] > scores[i + 1] {
            increasing = false;
        }
        if scores[i] < scores[i + 1] {
            decreasing = false;
        }
    }
    return increasing || decreasing; // this function is return thr values, if both the increaign tand decreaing is true
}
fn main() {
    // create mutable empty hashmap

    let mut product = HashMap::new();

    // add the product in the product hashmap

    product.insert("product1", vec![1, 2, 3, 4, 5]);
    product.insert("product2", vec![5, 4, 3, 2, 1]);
    product.insert("product3", vec![1, 2, 3, 4, 5]);
    product.insert("product4", vec![5, 4, 3, 2, 1]);
    product.insert("product5", vec![1, 2, 3, 4, 5]);
    product.insert("product6", vec![5, 4, 3, 2, 1]);
    product.insert("product7", vec![4, 5, 6, 3, 4]);
    product.insert("product8", vec![5, 4, 3, 2, 1]);
    product.insert("product9", vec![8, 8, 7, 6, 5, 4, 4, 1]);

    // iterating over the all product and decide the popularity of that product

    for (product_id, popularity) in product {
        if popularity_analysis(popularity) {
            println!("{} popularity is increasing or increasing", product_id);
        } else {
            println!("{product_id} is faluting");
        }
    }
}
