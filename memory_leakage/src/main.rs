// in this code we are work with the memory leakage and work to avoid in the future

pub mod weak;

use std::cell::RefCell;
use std::rc::{Rc,Weak};

#[derive(Debug)]
struct Node {
    next: Option<Weak<RefCell<Node>>>,
}
// using the Drop() crate, which is used to drop the value of function when its called and automatically cleaned up value at that time
impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping node: {:?}", self);
    }
}

fn main() {
    let a = Rc::new(RefCell::new(Node { next: None }));
    // i want to alos display the Rc count by using the strong count variable
    // println!("a strong count: {}", Rc::strong_count(&a)); // only display the strong count


    println!("a strong count: {}, a weak count: {:?}", Rc::strong_count(&a), Rc::weak_count(&a)); // display the both strong and weel count ref

    // lets create the another node and assign the value of a to b
    let b = Rc::new(RefCell::new(Node {
        next: Some(Rc::downgrade(&a)),
    })); // here we are using the Rc::clone() to clone the value of a to b
    println!("B is created:\n a count {:?}, a week count :{:?}", Rc::strong_count(&a),Rc::weak_count(&a));
    println!("b count: {:?}, b week count:{:?}",Rc::strong_count(&b),Rc::weak_count(&b));

    // next i will another node C which will point towards Node B
    // The next field of C which will be containing the reference to B
    let c = Rc::new(RefCell::new(Node {
        next: Some(Rc::downgrade(&b)),
    }));
    println!("a strong count: {}", Rc::strong_count(&a));
    println!("b strong count: {}", Rc::strong_count(&b));
    println!("c strong count: {}", Rc::strong_count(&c));

    // everything is fine now , The node B is pointing towards A and the node C is pointing towards B, which point towards A is not pointhing to anywhere

    // here i want to create the cycle
    // (*a).borrow_mut().next = Some(Rc::clone(&c));  // the Node B is pointing towards to A and the Node C is poiting to towards B and A is pointing towards C. creating a non ending cycle here
    // lets display the count of three node after creating the cycle


    (*a).borrow_mut().next = Some(Rc::downgrade(&c));  // this now causes the making cycle and memeory leakgae
    println!("After creating the cycle: \n a count:{:?}, a week count:{:?}", Rc::strong_count(&a),Rc::weak_count(&a));
    println!("After creating the cycle: \n b count:{:?}, b week count:{:?}", Rc::strong_count(&b),Rc::weak_count(&b));
    println!("After creating the cycle: \n c count:{:?}, c week count:{:?}", Rc::strong_count(&c),Rc::weak_count(&c));


    // at the end i am printing the node A  
    // println!("A is {:?}", a);  // this will cause overflow  due to reference cycle


    println!("A is {:?}", a);  // after the using the weak ref then it does not go stack overflow, and it prevents the memory leakgae

    // the drop function is only called when the strong count value is 1, but in our case the strong count value is 2, so the drop function is not called
    // so we need to break the cycle to avoid the memory leakage
    // to break the cycle w need the brrow_mut() function to break the cycle

    // there are one alternative method to handle this. Rust provide the nice solution to handle this by using the Weak<T> crate
    // The Weak<T> crate is used to create the weak reference to the Rc<T> crate



}
