
// Topics
/*
1. COMMENTS
2. PRINT STATEMENT
3. VARIABLE AND DATA TYPES
4. Shadowing
5. Constant
6. Strings
7. Tuples
8. Arrays
9. Vectors
10. Functions
11. Inputs
*/
pub fn basics_rust(){
    // PART 1 ----

    // Comments
    // this is our program
    /*
    This is how we add multi line comments i rust
    */

    // PART 2 ----
    println!("Hello from rust");
    print!("Same line");
    println!("first number {} second number {}",10,20);

    // PART 3 ----

    // variable and data types
    let x = 5.5;
    let y = 5*5;
    println!("{} {}",x,y);
    //scalar type data type
    // integer
    // signed - i8,i16,i32,i64
    // unsigned - u8,u16,u32,u64
    println!("the maximum number in i8 is = {}",std::i8::MAX);
    println!("the maximum number in u8 is = {}",std::u8::MAX);
    // float
    println!("the maximum number in f32 is = {}",std::f32::MAX);
    // bool
    let not_equal = 18 != 18;
    println!("the value of condition is {}",not_equal);
    // char
    let c1 = 'a';
    println!("{}",c1);
    // variable
    let (first_number,second_number) = (250,34);
    println!("{} {}",first_number,second_number);
    let x =255;
    println!("the value octal {:o}, hexa {:x} , binary {:b}",x,x,x);
    let n1  = 14;
    let n2 = 15.6;
    let x = n1 as f64 + n2 ;
    println!("the value {}",x);
    // shadowing
    let mut r = 65;
    {
    let r = 60;
        println!("inside {}",r);
    }
    println!("outside {}",r);
    // constants
    const MAX_SALARY:i32 = 100_000;
    println!("{}",MAX_SALARY);

    // strings
    let some_string = "Fixed length string";
    let mut growable_string = String::from("This string will grow");
    println!("{} {}",growable_string,some_string);
    growable_string.push('s');
    println!("push {}",growable_string);
    growable_string.pop();
    println!("pop {}",growable_string);
    growable_string.push_str("New string added");
    println!("new str {}",growable_string);

    // tuples
    let my_information = ("salary",40_000);
    println!("{} {} {:?}",my_information.0,my_information.1,my_information);
    let (salary_str , salary_value) = my_information;
    println!("{} {}",salary_str,salary_value);

    // arrays
    let mut number_arrays = [4,5,2,4,2];
    println!("{}",number_arrays[0]);
    println!("{:?}",number_arrays);
    let array_with_same_array = [0;10];
    println!("{:?}",array_with_same_array);
    let subset = &number_arrays[0..3];
    println!("{:?}",subset);

    // vectors
    let mut number_vec: Vec<i32> = vec![1,23,23143,4,5454];
    println!("{}",number_vec[0]);
    number_vec[1] = 4;
    println!("{:?}",number_vec);
    number_vec.push(90);
    println!("{:?}",number_vec);
    number_vec.remove(2);
    println!("{:?}",number_vec);

    // functions
    basic_fn();
    functions_with_inputs("Udit",40_000);
    let ans = function_with_inputs_outputs(3,4);
    println!("{}",ans);
    let result = function_with_inputs_multiple_outputs(3,4);
    println!("{:?}",result);

    let full_name = {
        let first_name = "Udit";
        let last_name = "Gurani";
        format!("{} {}",first_name,last_name)
    };
    println!("{}",full_name);

    // inputs
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("Failed to read input");
    let n: f64 = n.trim().parse().expect("invalid");
    println!("{:?}",n);

}

fn basic_fn() {
    println!("this is basic functions");
}

fn functions_with_inputs(name: &str, salary: i32) {
    println!("{} {}",name,salary)
}

fn function_with_inputs_outputs(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn function_with_inputs_multiple_outputs(num1: i32, num2: i32) ->(i32,i32,i32)  {
    (num1*num2,num1+num2,num1-num2)
}