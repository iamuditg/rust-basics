
/*
1. Stack Implementation
*/
use std::io::Read;

pub fn stack_implementation() {
    stack_imp();
}

// Stack Implementation
fn new_stack(maxsize: usize) -> Vec<u32> {
    let vec: Vec<u32> = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<u32>) -> Option<u32> {
    let poped_val = stack.pop();
    println!("The popped value is {:?}",poped_val);
    poped_val
}

fn push(stack: &mut Vec<u32>, item: u32, maxsize: usize) {
    if stack.len() == maxsize {
        println!("Cannot add more");
    }else {
        stack.push(item);
        println!("Stack {:?}",stack)
    }
}

fn size(stack: &Vec<u32>) -> usize {
    stack.len()
}

fn input() -> u32 {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("Failed to read input");
    let n: u32 = n.trim().parse().expect("invalid input");
    n
}



fn stack_imp() {
    println!("Let us first create a stack for our use");
    println!("Please mention th size of the stack");
    let size_stack = input();

    let mut stack = new_stack(size_stack as usize);

    loop {
        println!("\n\n ***** Menu **** \n");
        println!("1. Push \n 2. Pop \n 3. Display \n 4. Size \n 5. Exit");
        println!("\n Enter the choice: ");
        let choice = input();
        match choice {
            1 => {
                println!("Enter the value to be insert");
                let item = input();
                push(&mut stack, item, size_stack as usize);
            }
            2 => println!("the element which is poped is {:?}", pop(&mut stack)),

            3 => println!("the element are {:?}", stack),

            4 => println!("the size of the stack is {}", size(&stack)),

            5 => break,

            _ => println!("invalid choice")
        }

        println!("do you want to continue 1 = Yes / 0 = No");

        let status = input();
        if status == 1 {
            continue;
        } else {
            break;
        }
    }

}