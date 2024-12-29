// in this code we can see the how to used the weak references with the tree

use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)] // By using this no need to manual implemenatatio of struct
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 1,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node{
        value : 2,
        parent:RefCell::new(Weak::new()),
        children:RefCell::new(vec![Rc::clone(&leaf)]),  // now in this children we ar putting the value of leaf node
        // parent are supposed to take the ownership of the children, therefore we have used the Rc::clone() to clone the value of leaf node
        // this means that as the leaf is alive the parent is guaranteed to be alive
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // here we are using Rc::downgrade() to convert the branch node to a weak reference
    // may not cycyle in this case
    // in this case the branch is pointing to the child and child is pointing back to the parent
    // absence ot weak pointer it would create the cycle 

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // here we are using the upgrade() to convert the weak reference to the strong reference

    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
}

