// After Nodw we are defining the another structure for the root node in the tree

use std::process::Output;

struct BinarySearchTree {
    root: Node,
}

#[derive(Clone)]
struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// create a function for the node
impl Node {
    fn new(value: i32) -> Self {
        // initialize the field of the Node
        Node {
            val: value,
            left: None,
            right: None,
        }
    }
    // insert function to insert node into the binary search tree
    fn insert(&mut self, value: i32) {
        // chcek the value is > than the value of the Node
        if value > self.val {
            match self.right {
                None => {
                    self.right = Some(Box::new(Node {
                        val: value,
                        left: None,
                        right: None,
                    }))
                }
                Some(ref mut node) => node.insert(value),
            }
        } else {
            match self.left {
                None => {
                    self.left = Some(Box::new(Node {
                        val: value,
                        left: None,
                        right: None,
                    }))
                }
                Some(ref mut node) => node.insert(value),
            }
        }
    }
}

// and the another function is used to traverse the whole trr
fn traveral(node: Option<Box<Node>>, low: i32, high: i32, mut output: &mut Vec<i32>) {
    // checking the node is not null
    if !node.is_none() {
        // inside this function we are checking the 3 conditions that are based on the binary tree principe
        //  1) check the value is grether or lower than previous or current value or upper vale
        if node.as_ref().unwrap().val >= low && node.as_ref().unwrap().val <= high {
            output.push(node.as_ref().unwrap().val);
        }

        // to traverse the left side also
        if node.as_ref().unwrap().val >= low {
            traveral(node.as_ref().unwrap().left.clone(), low, high, &mut output);
        }

        if node.as_ref().unwrap().val <= high {
            traveral(node.as_ref().unwrap().right.clone(), low, high, &mut output)
        }
    }
}
// next we are define the product in the range
fn product_in_range(root: Node, low: i32, high: i32) -> Vec<i32> {
    // for the stroing the value of the vector
    let mut output: Vec<i32> = Vec::new();

    // calling the traversal function
    traveral(Some(Box::new(root)), low, high, &mut output);
    output
}
fn main() {
    let product_prices = vec![9, 614, 20, 1, 30, 8, 17, 5];
    let mut bst = BinarySearchTree {
        root: Node::new(product_prices[0]),
    };
    for i in 1..product_prices.len() {
        bst.root.insert(product_prices[i]);
    }

    // calling the range of function

    let result = product_in_range(bst.root, 7, 20);
    print!("{:?}", result);
}
