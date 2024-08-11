// consider a senecarios where a company tell the own employee to attend the online meeting and want to dispaly the name of employee by using a data structure to present it in alphabetical order. the employe can leave and join random, so the data structure should allow the updation accordingly . after that this secarios company is also want to show the employee in a paganated form. that means company want to show only 10 employees in a pages like. in summary we need to display the pagination and dispaly function which is dispaly the 10 number of people in a page.
// used the tools for this probkem is :
// BTs+stack

use std::cell::RefCell;
use std::rc::Rc;



#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct Node {
    value: String,
    left: Link,
    right: Link,
}
// lets define the link type, which will be the an optional node of wrapped around by the Rc and rough cell
type Link = Option<Rc<RefCell<Node>>>;

impl Node {
    fn new(val: String) -> Self {
        // inside the function i will rteurn a new value with the value being equal to the val and the left and right being to set the none
        Node {
            value: val,
            left: None,
            right: None,
        }
    }
    // add insert function
    fn insert(&mut self, val: String) {
        // if val is greater than self.value then we will match on right
        if val > self.value {
            match &self.right {
                None => self.right = Some(Rc::new(RefCell::new(Self::new(val)))),
                Some(node) => node.borrow_mut().insert(val.to_string()),
            }
        } else {
            match &self.left {
                None => self.right = Some(Rc::new(RefCell::new(Self::new(val)))),
                Some(node) => node.borrow_mut().insert(val.to_string()),
            }
        }
    }
}

// Define a wrapper struct for the constructing a new binary search
#[derive(Debug, Default, PartialEq, Clone)]
struct BinarySearchTree {
    root: Node,
}
impl BinarySearchTree {
    fn new(val: String) -> Self {
        BinarySearchTree {
            root: Node::new(val.to_string()),
        }
    }
    // add the insert function to insert the node ino the tree
    fn insert(&mut self, val: String) {
        // call the insert function to the root from the that it is define into the insert vblock
        self.root.insert(val.to_string());
    }
}

// here we are implement the stack of the node with the
struct DisplayLobby {
    stack: Vec<Rc<RefCell<Node>>>,
}
impl DisplayLobby {
    // as mention first we need insert all the left nodesof root into the stack
    fn new(root: Option<Rc<RefCell<Node>>>) -> Self {
        // inside function we will create the mutable stack
        let mut stack = Vec::new();
        // i will push the all left child to stack by calling the push_all_left function by cloning root and stack input
        Self::push_all_left(root.clone(), &mut stack);
        DisplayLobby { stack }
    }
    fn push_all_left(mut p: Option<Rc<RefCell<Node>>>, stack: &mut Vec<Rc<RefCell<Node>>>) {
        while let Some(link) = p.clone() {
            // add node to the stack
            stack.push(p.clone().unwrap());
            // update the node to left of the link
            p = link.borrow().left.clone();

            // this will be terminated when there was no node left in the stack
        }
    }

    fn next_name(&mut self) -> String {
        // this func do two things, 1st is pop() element from the stack and

        let node = self.stack.pop().unwrap();
        // store in a varibale
        let name = &node.borrow().value;
        // next grap the right child of the node
        let mut next_node = node.borrow().right.clone();

        // return

        Self::push_all_left(next_node, &mut self.stack);
        name.to_string()
    }

    fn next_page(&mut self) -> Vec<String> {
        // in this varibale contains the 10 name of the participants in an alphabetical order
        let mut resultant_name = Vec::new();
        for i in 0..10 {
            if !self.stack.is_empty() {
                resultant_name.push(self.next_name());
            } else {
                break;
            }
        }
        return resultant_name;
    }
}

fn main() {
    // add some node in the binary tree
    let mut bst = BinarySearchTree::new("John".to_string());
    let names = vec![
        "Latasha",
        "John",
        "David",
        "Anna",
        "brown",
        "Eric",
        "Sophia",
        "Olivia",
        "Michael",
        "Alexander",
        "Emma",
        "Daniel",
        "Chloe",
        "Bruce",
        "David",
        "James",
        "Sarah",
        "Emily",
        "zoneman",
        "youtube",
        "youtube"
    ];
    for name in names {
        bst.insert(name.to_string());
    }

    let mut displa = DisplayLobby::new(Some(Rc::new(RefCell::new(bst.root))));
    println!("Participants on the first page {:?}", displa.next_page());

    println!("Participants on the Second page {:?}", displa.next_page());
}
