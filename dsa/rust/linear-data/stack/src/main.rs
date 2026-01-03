use std::vec::Vec;

fn main() {
    println!("Stack: ");

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    
    //len() stack size
    assert_eq!(stack.len(), 2);

    //access value by index
    assert_eq!(stack[0], 1);
    
    //extend() add list 
    stack.extend([3, 4, 5]);
    let num = 0;
    stack.push(num);
   
    //FIFO
    assert_eq!(stack[stack.len() -1], 0);
    println!("Last element added => {}", stack[stack.len()-1]);
    println!("is the first in the queue and is at the top");
    println!("if it is removed, the one at the top will be => {}", stack[stack.len()-2]);

    println!("{:?}", stack);
    
    //remove elements
    stack.pop();
    stack.pop();
    
    loop {
        stack.pop();
        if stack.len() == 0 {
            break;
        }
    }

    println!("{:?}", stack);
    println!("stack empty");

}
