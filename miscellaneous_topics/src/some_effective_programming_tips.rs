// In this topics/section we are learning the various techniques for efficient coding in rust

//  1st topics
// Simplify the syntax nested condition using the match syntax

/*
fn main(){
    let cancer = true;
    let smooking = false;

    /*
    match cancer{
        true=>match smooking{
            true=>println!("You have cancer due to smooking"),
            falsee=>println!("Your cancer is not due to smooking"),
        }
        false=>match smooking{
            true=>println!("You are not cancer but you are smooking"),
            false=>println!("You are not cancer and not smooking"),
        }
    }
    */

    // the above match operation can be done with the following example by using the tuple, so by using these tyep of code simplicity your code code clean and perfect

    // this below code is more compact and more readable compared to the previous one
    match(cancer, smooking){
        (true, true)=>println!("You have cancer due to smooking"),
        (true, false)=>println!("Your cancer is not due to smooking"),
        (false, true)=>println!("You are not cancer but you are smooking"),
        (false, false)=>println!("You are not cancer and not smooking"),
    }
}
*/

//  2nd topics

// some times we have long vectors values of result and may be interested in pooling out 1st error in the list

// for e.g, we have client and client have some error message for the customers, for knowimg the error message
/*
 fn main() {
    // The message is comes from the server as following

    let response = vec![Ok(100),Err("Client error"),Ok(300),Err("Server error")]; // this sis the msg coming from the server to client
    // The collect method is extrimally powerful and we can use it on an iterator of result to either return a collection of key items or the very first error item if the list contains the error variant.
    // lets use the into_iter and then collect the responses
    let result: Vec<Result<i32, &str>> = response.into_iter().collect();

    println!("{:?}",result);
 }
*/

//  3rd topics

// we want to store a list of struct objects in an associated container such as HashMap key a field of the struct

// we store the list of person in a hashmap with the name of the person name as key

/*
use std::collections::HashMap;
 #[derive(Debug)]
 struct Person {
  name: String,
  age: u32,
 }

 // here we call the person_name, where we can iterate over the each employee name

 fn person_by_name(person:Vec<Person>)->HashMap<String,Person>{
        person.into_iter().map(|p|(p.name.clone(),p)).collect()
 }
 fn main() {
  let person1 = Person{
      name: "John".to_string(),
      age: 30,
  };

  let person2 = Person{
      name: "Jane".to_string(),
      age: 25,
  };
  let person3 = Person{
      name: "Mary".to_string(),
      age: 35,
  };

  // capture the all person in a vector

  let person = vec![person1, person2, person3];

  // I want to store this is into the hashmap

  // now call person_by_name function here

  let person_map = person_by_name(person);

  // for the printing the above values we need iterate through the loop
  for(name, details) in &person_map{
      println!("person {:?} has the details of {:?}",name,details);
  }


 }
 */

// another example for the this topic

fn main() {
    // for i in 9..0{     // if i write 9..0 then it work for reverse but it is not print anythings
    for i in (0..9).rev() {
        // the correct method is work like below, now this is work
        println!("{}", i);
    }
}
