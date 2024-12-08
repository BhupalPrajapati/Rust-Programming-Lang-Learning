/*
          // here we learn about the function inputs and coercion
// fn vowel(word:&String)->u8{  // String ref
fn vowel(word:&str)->u8{
    let vowel_count = word.chars()
        .into_iter()
        .filter(|x|(*x == 'a')|(*x == 'e')|(*x == 'i')|(*x== 'o')|(*x == 'u')).count();
    vowel_count as u8
}

fn main() {
    let check = "Bhupal".to_string();
    println!("Number of vowels in the word: {}", vowel(&check));  // it is check by using the string ref not string slice

    // checking with the string slice

    println!("{}",vowel("Bhupal"));

    // In above example we are seen the we able to convert a data types to another data types without leadiing or anythings
    // so that why we are used the Coecision used


}
*/

/*
// here is another example where we can see the example

// fn length_str(x:&Box<String>){

    fn length_str(x:&str){   // a reference to the box is being coerced to a string slice
    println!("length the string is {} is {}",x,x.len());

}
fn main() {
    // let box_str = Box::new("hello");
    // length_str(&box_str);

     length_str("Bhupal");
}

*/

// A refernce to a vector containig some type will be converted to a refernce to an array containing of the same type

pub mod some_effective_programming_tips;
pub mod todo_and_somemore;

fn square_values(num_vec: &[i32]) {
    for i in num_vec {
        println!(" Value of Square is : {}", i * i);
    }
}
fn main() {
    let vec_values = vec![1, 2, 3, 4, 5];
    square_values(&vec_values);

    let vec_array = [1, 2, 3, 4, 5];
    square_values(&vec_array);
}
