use std::io;
use std::io::Read;

pub fn flow_control() {
   // example_two();
   // example_three();
    //  example_five();
    // let num = if 3 > 2 {
    //     println!("if block");
    //     ()
    // };
    // println!("{:?}",num);
    example_six();
}

fn example_six() {
    let mut  n = 0;
    // loop {
    //     if n < 5 {
    //         println!("Hello");
    //         n += 1;
    //     }else {
    //         println!("bye");
    //         break;
    //     }
    // }
    // let mut  n = 1;
    // while n <= 5 {
    //     println!("hello");
    //     n += 1;
    // }
    for n in 1 .. 10 {
        println!("{}", n);
    }
}

fn example_five() {
    let mut o = String::new();
    let a = 10;
    let b = 20;
    println!("Choosen operation to be performed");
    println!("1. +\n2. -\n3. *\n4. /");
    io::stdin().read_line(&mut o).expect("Failed");
    let o:char = o.trim().parse().expect("Failed");

    if o == '+' {
        println!("{} ",a+b);
    }else if o == '-'  {
        println!("{}",a-b);
    }else if o == '*' {
        println!("{}",a*b);
    }else if o == '/' {
        println!("{}",a/b);
    }else {
        println!("wrong type");
    }
}

fn example_three() {
    let a = 100;
    let b = 20;
    let c = 50;
    if a > b && a > c {
        println!("A is greatest");
    }else if b > a && b > c {
        println!("B is greatest");
    }else {
        println!("C is greatest");
    }
}

fn example_two() {
    let mut name_str = String::new();
    let mut age = String::new();
    let mut ch = String::new();
    println!("Enter Name and Age:");
    io::stdin().read_line(&mut name_str).expect("Failed");
    io::stdin().read_line(&mut age).expect("Failed");
    let age:i32 = age.trim().parse().expect("Failed");
    println!("Do you want to create account");
    io::stdin().read_line(&mut ch).expect("Failed");
    ch = ch.trim().to_string();
    if ch == "y" {
        if age < 10 {
            println!("Yout age is less");
        }else {
            println!("Your account is created");
            println!("Do you want to upload");
            ch.clear();
            io::stdin().read_line(&mut ch).expect("Failed");
            ch = ch.trim().to_string();
            if ch == "y" {
                if age < 13 {
                    println!("you cannot upload photo");
                }else {
                    println!("you can upload your photo");
                }
            }else {
                println!("thanks for visiting");
            }
        }
    }

}

fn example_one() {
    let x = 3;
    let y = 5;
    // conditional
    if x > y {
        println!("x is greater than y");
    }else {
        println!("x is not greater than y");
    }

    // loops
    let mut count = 0;
    loop {
        if count == 10 {
            break;
        }
        count += 1;
        println!("count is {}",count)
    };

    // while loops
    count = 0;
    let letters = ['a','b','c'];
    while count < letters.len() {
        println!("count is {}",letters[count]);
        count += 1;
    }

    // for loops
    let message = ['a','b','c','d','e'];
    for (index,&item) in message.iter().enumerate() {
        println!("item {} is {}",index,item);
        if item == 'c'{
            break;
        }
    }
    let mut matrix = [[1,2,3],[4,5,6],[7,8,9]];
    for row in matrix.iter_mut() {
        for num in row.iter_mut() {
            *num += 10;
            print!("{}\t",num);
        }
        println!();
    }
}