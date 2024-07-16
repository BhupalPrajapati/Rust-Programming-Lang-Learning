    // Suggesting the item for the shopping cart
     //  given the list of items, return the the couple of items wih its matching prices woth the items

     // the given tools are used in this problem
        // HashSet and Vectors
use std::collections::HashSet;

fn product_suggestions(product_price:Vec<i32>,amount:i32)->Vec<Vec<i32>>{
    let mut price_hash = HashSet::new();
    let mut offer = Vec::new();

    // compare the 1's value with the each value in the product and check the 1st value is paired up with any of the value in the given amount
    // on the same way, we can procedue the 2nd element and compare with the other element in the list

    // for that, we can use the 2 look , the outer loop is iterate the value and 2nd loop is check it agenist the possible other value in the list

    for i in product_price{
        let diff = amount - i;
        // initalize the initailly does not contins any value

        if price_hash.get(&diff).is_none(){
            price_hash.insert(i);
        } else {
            offer.push(vec![i,diff]);
        }
    }
    offer
}
fn main(){
      let product_price = vec![11,30,55,34,35,10,19,20,60,5,23];
      let b = product_suggestions(product_price, 50);
      println!("{:?}",b);
}