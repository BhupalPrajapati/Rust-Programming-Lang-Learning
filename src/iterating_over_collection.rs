use std::collections::HashMap;

fn main() {
    let vec1 = vec![1, 2, 3, 4, 5];
    // let  vec1_iter = vec1.iter_mut();
    //     println!("{:?}",vec1_iter );

    //     println!("{}",vec1_iter);

    // let value1 = vec1_iter.next();
    // println!("{:?}",value1 );
    for values in &vec1 {
        println!("{values}");
    }
    println!("{:?}", vec1);

    // let implement the hashmap by using the collection

    let mut person: HashMap<String, i32> = HashMap::new();
    person.insert("Manxe".to_string(), 21);
    person.insert("Value".to_string(), 20);
    person.insert("Huf".to_string(), 30);
    for (name, age) in &person {
        println!(" {} {}", name, age);
    }
}
