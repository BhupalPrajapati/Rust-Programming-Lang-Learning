#[warn(non_snake_case)]
#[warn(unused_variables)]

//    here we are writing the code for the postfix expression evaluation

/*
1. priority of operatiors
    -> +,-1
    -> *,/
    -> ^

2. if scanned operator is <- then the top of the stack in priority then pop operator until we have low priority. Add the popped elements to the postfix

3. if "(" pust it to the stack

4. if ")" pust it to the stack

5. if operand then just add in the postfix
 */

fn new_stacK(maxsize: usize) -> Vec<String> {
    let vec = Vec::with_capacity(maxsize);
    vec
}

// function for pop operations

fn pop(stack: &mut Vec<String>) -> Option<String> {
    let poped_val = stack.pop();
    println!("The poped value is: {:?}", poped_val);
    poped_val
}

// function for push operations
fn push(stack: &mut Vec<String>, item: String, maxsize: usize) {
    if stack.len() == maxsize {
    } else {
        stack.push(item);
        println!("stack {:?}", stack);
    }
}

// function for calculating the size of the stack

fn size(stack: &Vec<String>) -> usize {
    stack.len()
}

// this function is convert the given experssion is inot the individual symbols and will be return the inidivual symbols as element of string vector
// input of this function is string and output is vector of string

fn individual(input_exp: String) -> Vec<String> {
    let mut tokenized_input: Vec<String> = Vec::new();
    let input_chars: Vec<char> = input_exp.chars().collect();
    // Here temp is a temporary variable
    let mut temp: Vec<char> = Vec::new();

    for i in input_chars {
        // checking input is operator or operands
        if i != '+' && i != '-' && i != '/' && i != '*' && i != '^' && i != '(' && i != ')' {
            temp.push(i);
            continue;
        } else {
            if temp.len() == 0 {
                // we will ad the operator in that case if there is no operands
                tokenized_input.push(i.to_string());
            } else {
                // i will use the connect function will collect all the individual characters in a variable temp
                // as a string and will add it as a single string element into the vector of tokenized_input
                tokenized_input.push(temp.clone().into_iter().collect());
                // next i will push current character which is given by the variable I into the tokenized_input which correspond s to operations
                tokenized_input.push(i.to_string());
                temp = vec![];
                // at the end i am flush the everything from the temp vector becasue I am now expecting another operand
                // this means each time  i will encounter a operations i will flush out the temp vector since i am expecting another operand now
            }
        }
        // Amy things is left in input after the parathesis push into the tokenizied_input
        // for eg : (33+45/3*(2+9)-50)-10. in this eg the last integer is not accepting the stack , so that we are write the code where we can add the last integer is also into the stack and it is also a part of of the stack
    }
    if temp.len() != 0 {
        tokenized_input.push(temp.into_iter().collect());
    }
    println!("Input Infix: {:?}", tokenized_input);
    tokenized_input
}

// by Using the following function we are calculate the post of the given expression

fn infix_to_postfix(input: Vec<String>) -> Vec<String> {
    let size_exp = input.len();
    let mut stack = new_stacK(size_exp);
    let mut post_fix = Vec::new();

    for i in input {
        match i.as_str() {
            "+" | "-" | "*" | "^" => {
                // check if the stack is not empty
                if size(&stack) == 0 {
                    push(&mut stack, i, size_exp);
                } else {
                    // here i will scan the priority of the operator and check it the top element of the stack or not

                    if priority(&i) > priority(stack.last().unwrap()) {
                        // if the priority of 1st element is greATER than the last elementr then it is push into the stacj
                        // if this condition is not true that means top of the element of the stack is lesser than or equals to the last element

                        push(&mut stack, i, size_exp);
                    } else {
                        while priority(&i) <= priority(stack.last().unwrap()) {
                            post_fix.push(pop(&mut stack).unwrap());
                            if stack.last() == None {
                                break;
                            }
                        }
                        push(&mut stack, i, size_exp);
                    }
                }
            }
            "(" => push(&mut stack, i, size_exp),
            ")" => {
                while stack.last().unwrap() != "(" {
                    post_fix.push(pop(&mut stack).unwrap());
                }
                pop(&mut stack).unwrap(); // remove the opening bracket from the stack
            }
            _ => post_fix.push(i),
        }
    }
    while size(&stack) != 0 {
        post_fix.push(pop(&mut stack).unwrap());
    }

    println!("Infix to Postfix: {:?}", post_fix);

    post_fix
}

fn priority(x: &String) -> u8 {
    //set the priority of the operator
    if (x == "+") | (x == "-") {
        1
    } else if (x == "*") | (x == "/") {
        2
    } else if x == "^" {
        3
    } else {
        0
    }
}

// function for evaluating postfix expressions  - final part we are calculating the final result of postfix expressions expersion

/*  Rules for the postfix expressions
    1. if operand -> push to stack
    2. if operation pop two elements perform the operation and then push into the stack

*/

fn postfix_evaluayion(postfix: Vec<String>) -> f32 {
    let size_exp = postfix.len();
    let mut result_stack = new_stacK(size_exp);
    for i in postfix {
        match i.as_str() {
            "+" | "-" | "*" | "/" | "^" => {
                // Remember the those two rules
                // We assume the first element is the operation and we call that element

                let oper = i;

                // next i will poped the element, the element which is poped which is second operand for the operation and element

                let op1 = pop(&mut result_stack).unwrap();
                let op2 = pop(&mut result_stack).unwrap();
                let result = operation(op1, op2, oper);

                // next we will push the result into the stack
                // the result need to converted into the string type

                push(&mut result_stack, result.to_string(), size_exp);
            }
            _ => push(&mut result_stack, i.to_string(), size_exp),
        }
    }
    // when all the symbols are scanned and no one left then we scan and pop and retrurn that

    pop(&mut result_stack).unwrap().parse::<f32>().unwrap()
}

// Here we doing the expersion for the input

fn operation(op1: String, op2: String, oper: String) -> f32 {
    // I will change the value of two operations is from String to f32
    let op1 = op1.parse::<f32>().unwrap();
    let op2 = op2.parse::<f32>().unwrap(); // we need to unwrap the parse because it is return the result of option enum

    let result = match oper.as_str() {
        "+" => op1 + op2,
        "-" => op1 - op2,
        "*" => op1 * op2,
        "/" => op1 / op2,
        "^" => op1.powf(op2),
        _ => 0.0,
    };
    result
}

fn main() {
    let input_exp = String::from("(33+45/3*(2+9)-50)");
    println!("The original expression is {:?}", input_exp);

    let input_exp_tokenized = individual(input_exp);

    let postfix = infix_to_postfix(input_exp_tokenized);

    println!("The evaluation exp = {}", postfix_evaluayion(postfix));
}
