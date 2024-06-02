// Briing the memory module in the Scope
use std::mem::size_of;
fn main(){
    println!("hello world");
    // sized types
    println!{"size of the array is:{}",size_of::<i32>()};
    struct pool{
        x:bool,  // bool = 1  bit
        y: i64, // 8 bit(i32 = 4bit)
    }
    println!("pool size is: {}",size_of::<pool>());

  
    // Unsized types

    // specially slices are unsized types in rust

    println!{"[i32] size is :{}",size_of::<&[i32]>()};
    let a:[i32;3];
}