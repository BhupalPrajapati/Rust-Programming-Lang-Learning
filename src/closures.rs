#[warn(dead_code)]
struct Employe {
    name: String,
    id: u32,
    age: u32,
}

// fn validate(name:&str)->bool{
//     name.len()!=0
// }
fn validate_user(
    name: &str,
    age: u32,
    banned_user_name: &str,
    simple_va: fn(&str, &str) -> bool,
    advanced_va: fn(u32) -> bool,
) -> bool
// here define trait bounds where Fn trait can bound with generic as a concrete type

    // where 
    //      V1:FnOnce(&str)->bool,
    //       V2:Fn(u32)->bool,
{
    simple_va(name, banned_user_name) && advanced_va(age)
}

// Functional pointer

fn variable_input_simple(name: &str, banned_user_name: &str) -> bool {
    name.len() != 0 && name != banned_user_name
}

fn veriable_input_advance(age: u32) -> bool {
    age >= 30
}
fn main() {
    let emp = Employe {
        name: "Manxe".to_string(),
        id: 2,
        age: 40,
    };
    // println!("The validate Employe is {}",validate(&emp.name));

    // the varibale can be captured through the 3 ways

    // Here I am implementing the closure and Move
    // Move keyword is used for the converts a refernces envirnomental to full ownership to value
    /*
         let  banned_user = String::from("Someone");

        let variable_input_simple = move |name:&str|{
        let banned_user_name = banned_user;
        name.len()!=0};

        let veriable_input_advance = |age:u32|age>=30;

    */

    // println!("{}",variable_input(&emp.name));

    let banned_user = "someoone";
    println!(
        "{}",
        validate_user(
            &emp.name,
            emp.age,
            banned_user,
            variable_input_simple,
            veriable_input_advance
        )
    )
}
