// use crate::product;

// use crate::product;

fn main() {
    // why passinf the some here bcz the
    let some_product = Some("laptop");
    let products = vec!["cellphone", "battery", "charger"];
    // we want to show if some-product is none then push into the products list

    // match some_product {
    //     Some(product) => products.push(product),
    //     _=>{}
    // };
    // println!("{:?}",some_product );

    // this is more simplyfied way to do this
    // if let Some(product)=some_product{
    //     products.push(product);
    // }

    //one more simpler method to do same thing called the extends method
    // extends method extend the collection with the content of an iteratir
    //      products.extend(some_product);
    //      println!("{:?}",products );

    // we are also used the iter() method for the convert the collection into iterator
    let product_item = products.iter().chain(some_product.iter());
    for prob in product_item {
        println!("{prob}");
    }
}
