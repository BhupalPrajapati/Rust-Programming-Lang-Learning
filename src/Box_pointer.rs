// here we use libray of rc pointer

use std::rc::Rc;

fn main(){
    /* 


    // let x=0.4;
    // let y = Box::new(x);
    // // println!("{}",y);
    // let z = &x;
    // println!("{}",z);
    #[allow(dead_code)]

    let list = List::Cons(2,Some(Box::new( List::Cons(3,Some(Box::new(List::Cons(4, None)))))));
    println!("{}",list)

}
// let create enum of list

// this enum id modified using some mehod like Option
enum List {
    Cons(i32,Option<Box<List>>),
    // Nill,

*/

                    
                             // Take another example of Box Pointer(Use case of Box)
/* 
 let data_1 = Huge_date;
 let data_2 = Box::new(Huge_Date);
 let date_3 = data_1; // here all the data are copied into date_3 bcz it is stored in stack
 let date_4 = data_2;  // here small amount of data only copied bcz it is contains in heap
                                        // When you assign data_2 to date_4, only the pointer (the memory address) is copied, not the actual data. 

*/


/* 
               
                         // take another example of box pointer with help of trait object
   let data_1 = Huge_date;
   let date_2 = Box::new(Huge_date);
   let date_3 = date_1;
   let date_4 = date_2;  
   let date_5 = Box::new(Small_date);
   // new define vec a type which imple storage trait
   // to enbale vector to stored different tell the compiler that stored different type of data
   // here we can used the trait Object
   let data:Vec<Box<dyn Storage>> = vec![Box::new(date_3),date_4,date_5];                    
}


struct Huge_date;

struct Small_date;
trait Storage{}
impl Storage for Huge_date  {
    
}
impl Storage for Small_date {
    
}
*/


                                          // Rc Smart Pointer
let a = Rc::new(List::Cons(1,Some(Rc::new(List::Cons(2, None)))));
println!("{}",Rc::strong_count(&a));
{
let b = List::Cons(3, Some(Rc::clone(&a)));
println!("{}",Rc::strong_count(&a));
let c = List::Cons(4, Some(Rc::clone(&a)));
println!("{}",Rc::strong_count(&a));

}
println!("{}",Rc::strong_count(&a));
} 

enum List {
    // Here we used Rc instead of Box
    Cons(i32,Option<Rc<List>>)
}