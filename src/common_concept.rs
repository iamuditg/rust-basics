

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
pub fn common_concept() {
    variable();
    constants();
    shadowing();
    data_types();
    functions();
}

fn functions() {
    // functions
    another_function();
    parameter_function(5,'h');
    let x = return_function(5);
    println!("the value of x: {x}");
}

fn return_function(x: i32) -> i32 {
    x + 1
}

fn parameter_function(p0: i32, p1: char) {
    println!("{p0} and {p1}");
}

fn another_function() {
    println!("**FUNCTION**");
}

fn data_types() {
    // SCALER DATA TYPES
    // data types
    println!("**DATA TYPES**");
    // integer
    let integer: i32 = 5;
    println!("integer {}",integer);
    // floating point types
    let x = 2.0;
    let y: f32 = 3.0;
    println!("floating {} {}",x,y);
    // numeric operations
    println!("**NUMERIC Operation**");
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;
    println!("{} {} {} {} {} {}",sum,difference,product,quotient,truncated,remainder);
    // boolean type
    println!("**BOOLEAN**");
    let t = true;
    let f:bool = false;
    println!("{} {}",t,f);
    // character type
    println!("*CHARACTER TYPE**");
    let c = 'z';
    let z:char = 'Z';
    println!("{} {}",c,z);

    // COMPOUND TYPE
    println!("Tuple");
    let tup: (i32,f64, u8) = (500, 6.4, 1);
    let (x,y,z) = tup;
    println!("tuple: {} {y}",tup.1);

    println!("**ARRAY**");
    let a = [1,2,3,4,5];
    println!("{:?}",a)
}

fn variable() {
    // variables
    println!("***VARIABLE***");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is :{x}");
}

fn constants() {
    // constants
    println!("**CONSTANTS**");
    println!("{}",THREE_HOURS_IN_SECONDS);
}

fn shadowing() {
    // shadowing
    println!("**SHADOWING**");
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is :{x}");
    }
    println!("The value of x is: {x}");
}