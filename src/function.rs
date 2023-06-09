fn square(x: i32) -> i32 {
    println!("squaring is x is {}",x);
    x * x
}

pub fn add(a:i32,b:i32) {
    println!("{}",a+b);
}

pub fn add_return_type(a:i32,b:i32) -> i32 {
    return a + b;
}

pub fn subtract(a1: i32, a2: i32) -> i32 {
    return a1 + a2
}

pub fn subtract_print(mut a1: i32) {
    a1 = a1 -32;
    println!("{}",a1);
}

pub fn stack_func(mut var:i32) {
    var = 56;
    println!("Var: {}",var);
}

pub fn heap_function(var: &mut Vec<i32>) {
    var.push(50);
    println!("Var: {:?}",var)
}