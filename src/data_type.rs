/*
 data Types
*/
pub fn data_types() {
    // arithmetic operator
    //let a = 10;
    //let b = 3;
    //let mut c = a-b;
    //println!("c is {}",c);
    let a  = 10.0;
    let b = 3.0;
    //let c = a as f64 / (b + 1.0);
    let c = a/b;
    println!("c is {:8.3}", c);

    // boolean data types
    let a = true;
    let b = false;
    println!("a is {} and b is {}",a,b);
    println!("NOT a is {}",!a);
    println!("a AND b is {}",a & b);
    println!("a OR b is {}",a|b);
    println!("a XOR b is {}",a ^ b);

    let a=13;
    let b =2.3;
    let c: f32 = 120.0;

    let average = (a as f64 + b + c as f64)/3.0;
    assert_eq!(average,45.1);

    // String
    let some_string: &str = "Fixed length string";
    println!("{}",some_string);

    let mut growable_string: String = String::from("This string will grow");
    println!("{}",growable_string);
    growable_string.push('s');
    println!("{}",growable_string);
    growable_string.pop();
    println!("{}",growable_string);

    // arrays
    let mut letters = ['a','b','c'];
    letters[0] = 'x';
    println!("first letters is {}",letters[0]);

    let mut words = ['d','e','f'];
    words[0] = 'c';
    println!("first words is {}",words[0]);

    let numbers: [i32;5];
    numbers = [0,0,0,0,0];
    let index: usize = numbers.len();
    println!("last number is {} and len is {}",numbers[4],index);

    let words: [&str;5];
    words = ["words","heelo","udit","gurani","good"];
    let size: usize = words.len();
    println!("all words is {:?} and len is {}",words,size);

    let parking_lot = [[1,2,3], [4,5,6]];
    let number = parking_lot[0][1];
    println!("number is {}",number);

    // tuples
    let mut stuff: (u8,f32,char) = (10, 3.14, 'x');
    stuff.0 += 3;
    let first_item = stuff.0;
    println!("first_item is {}",first_item);
    let (a,b,c) = stuff;
    println!("b is {}",b);
}