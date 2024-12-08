// here bring refcell into the scope
use std::cell::RefCell;
fn main() {
    /*
    // this program is show error becz rule of borrowing not vaild here(borrowing syas that at a time there will be one mutable or multiple immutable variable, not both at same time)
    let mut x = 50;
    let  x1 = &x;
    println!("{}",x1);
    let x2 = &x;
    println!("{}",x2);
    let x3 = &mut x;
    println!("{} {}",x1,x2);
    */

    // this program is run with help of Refcell pointer

    let x = RefCell::new(10);
    let x1 = x.borrow();
    let x2 = x.borrow();
    let x3 = x.borrow_mut();
    println!("{} {}", x1, x2);
}
