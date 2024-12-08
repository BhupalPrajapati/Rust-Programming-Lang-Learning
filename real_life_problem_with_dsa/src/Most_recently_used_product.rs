// Most Recently used product
// -Description
// A business is interesting in knowing the product that has been purchased more recently by customers

// used
// -Hashmap and Doubly Linkedlist

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
#[derive(Debug)]
struct Node {
    prod_id: i32,
    prev: Link,
    next: Link,
}
impl Node {
    fn new(elem: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            prod_id: elem,
            prev: None,
            next: None,
        }))
    }
}
type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug, Default)]
struct List {
    head: Link,
    tail: Link,
}
impl List {
    fn new() -> List {
        List {
            head: None,
            tail: None,
        }
    }

    // here we want to push back to the element
    pub fn push_back(&mut self, elem: i32) -> Link {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail.clone());
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail.clone());
            }
        }
        self.tail.clone()
    }

    pub fn remove_front(&mut self) -> Option<Link> {
        self.head.take().map(|old_head| {
            // this statement is gives as new head
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                    self.head.clone()
                }
                None => {
                    self.tail.take();
                    None
                }
            }
        })
    }

    // here we are define the move to tail function for the iterating the node from head to tail

    fn move_to_tail(&mut self, node: &Rc<RefCell<Node>>) {
        // to grep the previous node details to need to the borrow function accoordinds that
        // nd access the previous field which contains the previous information

        let prev = node.borrow().prev.as_ref().map(|a| Rc::clone(a));

        // with the same way we can also grep the next node information
        let next = node.borrow().next.as_ref().map(|a| Rc::clone(a));

        match (prev, next) {
            (None, None) => {}
            (Some(_), None) => {}
            (None, Some(next)) => {
                node.borrow_mut().next = None;
                next.borrow_mut().prev = None;
                self.head = Some(next.clone());

                // next we uupdate necessary tail of the linked list
                let prev_tail = self.tail.as_ref().unwrap();
                // next we will update the next pointer of the previous tail
                prev_tail.borrow_mut().next = Some(node.clone());
                //next set the previous of the node to that of prev_tail
                node.borrow_mut().prev = Some(prev_tail.clone());
                //finaly update the tail

                self.tail = Some(node.clone());
            }
            (Some(prev), Some(next)) => {
                // set next of the node to null
                node.borrow_mut().next = None;
                prev.borrow_mut().next = Some(next.clone());
                next.borrow_mut().prev = Some(prev.clone());

                // do the reference of the prev node
                let prev_tail = self.tail.as_ref().unwrap();
                // prev_tail wii be become the node
                prev_tail.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(prev_tail.clone());
                self.tail = Some(node.clone());
            }
        }
    }
}

#[derive(Debug)]
struct MR_Item {
    map: HashMap<i32, Rc<RefCell<Node>>>,
    item_list: List,
    size: i32,
    capacity: i32,
}

impl MR_Item {
    fn new(capacity: i32) -> Self {
        Self {
            map: HashMap::new(),
            item_list: List::new(),
            size: 0,
            capacity: capacity,
        }
    }

    fn purchased(&mut self, prod_id: i32) {
        if let Some(node) = self.map.get(&prod_id) {
            self.item_list.move_to_tail(node);
        } else {
            if self.size >= self.capacity {
                if let Some(prev_head) = self.item_list.remove_front() {
                    self.map.remove(&prev_head.unwrap().borrow().prod_id);
                }
            }
            let node = self.item_list.push_back(prod_id).unwrap();
            self.map.insert(prod_id, node);
            self.size += 1;
        }
    }

    fn print(&self) {
        let mut traversal = self.item_list.head.clone();
        while let Some(temp) = traversal {
            print!("{} ", temp.borrow().prod_id);
            traversal = temp.borrow().next.clone();
        }
        println!();
    }
}

fn main() {
    let mut items_list = MR_Item::new(3);
    items_list.purchased(10);
    items_list.print();

    items_list.purchased(15);
    items_list.print();

    items_list.purchased(20);
    items_list.print();
}
