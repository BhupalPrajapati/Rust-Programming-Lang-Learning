fn main(){
    /* 
    println!("HELLO");
    let some_option = Some("Alice".to_owned()); // to_owned is used for the owned the value, that is the used further in the in move value
    match &some_option{
        Some( inner_value) => println!("value is {}", inner_value),
        None => println!("value is none"),
    }
    // here i want to access some_option value in outer side of loop, bcz the owenrship of the some-option variable is take the owneship is inner_value variable,
    // bac inner_value variable is take string, so that inner_value variable is take the owenrship of the somw_option variable
      // for that we can used the ref keyword which take the owernship of that variable so that we can used the that varibae in insie thatr 
    println!("value is {:?}", some_option);
    */


         // Now the example of as_ref

    let some_option = Some("Alice".to_owned());
    let some_1 = &some_option;
    let some_2 = some_option.as_ref();
    try_me(option_name: some_option.as_ref());
    println!("value is {:?}", some_option);
}
fn try_me(option_name:Option<&String>){
    match option_name{
        Some(inner_value) => println!("value is {}", inner_value),
        None => println!("value is none"),
    }     
}