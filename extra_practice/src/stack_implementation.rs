fn new_stack(maxsize: usize) -> Vec<u32> {
    let vec = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<u32>) -> Option<u32> {
    let poped_val = stack.pop();
    println!("The poped value is: {:?}", poped_val);
    poped_val
}

fn push(stack: &mut Vec<u32>, item: u32, maxsize: usize) {
    if stack.len() == maxsize {
        println!("Can't add more")
    } else {
        stack.push(item);
        println!("stack {:?}", stack);
    }
}

fn size(stack: &Vec<u32>) -> usize {
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
    println!("Let's create the stack for the our use");
    println!("Please mention the size of the stack");

    let size_stack = input();

    // create the stack
    let mut stack = new_stack(size_stack as usize); // convert u32 to usize that why we used the as alias usize

    // display the menu of the user that can select and perform the action according to his action
    loop {
        println!("\n\n ****** Menu *****\n");
        println!("1.Push \n, 2.Pop \n, 3.Dispaly \n, 4.size\n, 5.Exit");
        println!("\n Please enter your choice ");
        let choice = input();
        match choice {
            1 => {
                println!("Enter the value to be Insert");
                let item = input();
                push(&mut stack, item, size_stack as usize); // convert u32 to usize that why we used the as);
            }
            2 => println!("The element which is poped is {:?}", pop(&mut stack)),
            3 => println!("The stack is {:?}", stack),
            4 => println!("The size of the stack is {}", size(&stack)),
            // 5 => println!("Exiting..
            5 => break,
            _ => println!("Invalid choice, try again"),
        }

        // in the end pf the loop i will aks the user again he want to stop or continue
        println!("Do you want to continue 1 - Yes / 0 - No");
        let status = input();
        if status == 1 {
            continue;
        } else {
            break;
        }
    }
}
