/*
SMART POINTER
*/

// BOX Pointer
#[derive(Debug)]
enum List {
    Cons(i32,Box<List>),
    Nil,
}

use List::{Cons, Nil};
pub fn box_pointer(){
    let single_value = Box::new(0.625);
    let x = 0.625;
    println!("single value and x value is {}", x == *single_value);
    let list = List::Cons(1,Box::new(Cons(2, Box::new(Cons(3, List::Nil)))));
    println!("{:?}",list.0)
}

