//  String Reverse using the Stack

fn new_stack(maxsize: usize) -> Vec<char> {
    let vec = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<char>) -> Option<char> {
    let poped_val = stack.pop();
    println!("The poped value is: {:?}", poped_val);
    poped_val
}

fn push(stack: &mut Vec<char>, item: char, maxsize: usize) {
    if stack.len() == maxsize {
        println!("Can't add more")
    } else {
        stack.push(item);
        println!("stack {:?}", stack);
    }
}

fn size(stack: &Vec<char>) -> usize {
    stack.len()
}

// taking the user input

fn input() -> u32 {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input");

    // changing the input in the u32

    let n = n.trim().parse().expect("invalid input");
    n
}

fn main() {
    let input_string = String::from("This is the string reversel");  // 'l' is the last char is pushed into the stack and 'l' is first char when your perform the pop() operations on thsi
    let size_stack = input_string.len();

    // let the store the result of stack into a variable of the stack

    let mut stack = new_stack(size_stack);

    // i will create a new string which store the reverse of the string

    let mut rev_string = String::new();

    // i want to push the all string back into the stack

    for i in input_string.chars(){
        push(&mut stack, i, size_stack);
    }

    // i want to know the size of the stack
    for i in 0..size(&stack){
        rev_string.push(pop(&mut stack).unwrap());
    }

    println!("The original string{:?}",input_string);
    println!("The reversed string is: {:?}", rev_string);
    

}
