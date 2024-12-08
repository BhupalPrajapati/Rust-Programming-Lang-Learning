// In this section we are found the highest price of the stock
// problem decription

// we have stockwise stock price, we ant to retrieve the highest stock price in any in week in little to no time.

// -tools
// Maxstacks, structures, Vectors

// we are implementing thid prb with the structure
// we have two stock, man and max stock

struct MaxStack {
    main_stack: Vec<i32>,
    max_stack: Vec<i32>,
}

impl MaxStack {
    fn new() -> Self {
        MaxStack {
            main_stack: Vec::new(),
            max_stack: Vec::new(),
        }
    }
    // implements the push and pop function
    // push is work for the main functions

    fn push(&mut self, value: i32) {
        self.main_stack.push(value);
        if !self.max_stack.is_empty() && self.max_stack.last().unwrap() > &value {
            self.max_stack.push(*self.max_stack.last().unwrap());
        } else {
            self.max_stack.push(value);
        }
    }

    // implemenation of the pop functins

    fn pop(&mut self) {
        self.main_stack.pop();
        self.max_stack.pop();
    }

    // here we calculate the max_val at any point of time .
    // the input of this function is refernce of the self and ouput will be i32

    fn max_val(&self) -> i32 {
        *self.max_stack.last().unwrap()
    }
}

fn main() {
    // let create the instnacves of maxStack structure
    let mut stack = MaxStack::new();
    stack.push(50);
    stack.push(80);
    stack.push(120);
    stack.push(150);
    stack.push(100);
    stack.push(140);
    stack.push(145);

    println!("Max value of the stock");
    println!("{}", stack.max_val());

    // lets back one week to find the max value from axStack

    println!("After going one week back");
    println!("MAx value of stock.");
    stack.pop();

    println!("{}", stack.max_val());
}
