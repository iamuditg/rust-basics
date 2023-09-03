
/*
RUST OWNERSHIP
- Each value in rust has a variable that's called its owner.
- there can be only one owner at a time.
- when the owner goes out of scope, the value will be dropped.
REFERENCES RULES
- one mutable reference in a scope
- many immutable references
- mutable and immutable cannot coexist
- scope of a references
- data should not change when immutable references are in scope
*/
use std::env::var;

pub fn f_ownership() {
    let x  = 32.6;
    let y = x;
    println!("{} {}",x,y);

    let s1 = String::from("abc");
    //let s2 = s1;
    //println!("s1: {} , s2: {}",s1,s2); // cannot use
    let s2 = &s1;
    println!("s1: {} , s2: {}",s1,s2);

    let vec_1 = vec![5,6,9,8,7];
    let vec_2 = vec_1.clone();
    println!("vec1: {:?},vec_2: {:?}",vec_1,vec_2);

    // {
    //     let my_name = String::from("udis");
    // }
    // println!("{}",my_name) // not allowed

    let stack_num = 32;
    let mut heap_vec = vec![1,2,3];
    stack_function(stack_num);
    println!("the value inside the main of stack_num {}",stack_num);

    heap_function(&mut heap_vec);
    println!("{:?}",heap_vec);

    // References Rules

    // let mut heap_num = vec![4,5,6];
    // let ref1 = &mut heap_num;
    // let ref2 = &mut heap_num;
    // println!("ref1: {:?} ref2: {:?}",ref1,ref2); // cannot

    let mut heap_num = vec![4,5,6];
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    println!("ref: {:?} ref2: {:?}",ref1,ref2);

    // let mut heap_num = vec![4,5,6];
    // let ref1 = &heap_num;
    // let ref2 = &heap_num;
    // let ref3 = &mut heap_num;
    // println!("{:?} {:?} {:?}",ref1,ref2,ref3); // cannot

    let mut heap_num = vec![4,5,6];
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    println!("{:?} {:?}",ref1,ref2);
    let ref3 = &mut heap_num;
    println!("{:?}",ref3)



}
fn heap_function(vec : &mut Vec<i32>) {
    vec.push(7);
    println!("Var: {:?}",vec);
}
fn stack_function(mut var:i32) {
    var = 56;
    println!("Var {}",var);
}