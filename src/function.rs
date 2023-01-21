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
