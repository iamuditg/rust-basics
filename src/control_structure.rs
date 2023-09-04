
/*
1. IF Statement
2. Match Statement
3. Loops and While loop
4. For loop
5. Break and Continue statement
*/

pub fn control_structure() {
    let some_number = 80;
    let flag = true;
    if (some_number > 30 && some_number < 50) && flag {
       println!("Number is less then 50 and greater then 30");
    }else if some_number > 50 && some_number < 100 {
        println!("Number is less then 100 and grater then 50");
    }else {
        println!("Number not accepted")
    }
    println!("the line will execute");

    // let value = if flag {
    //     1;
    // }else {
    //     0;
    // };
    // println!("value is {:?}",value);

    let some_number  = 101 ;
    match some_number {
        1 => {println!("Not Matched")},
        4..=100 => {println!("Matched")},
        _ => println!("Not Matched overflow")
    }

    let marks  = 50;
    let mut grade = 'N';

    match marks {
        10..=50 => grade = 'B',
        50..=100 => grade = 'A',
        _ => grade = 'F'
    }
    println!("{}",grade);

    // infinite loop
    // loop {
    //     println!("This is an infinite loop")
    // }

    let my_number = 5;
    let mut guess = false;
    println!("Guess my number which is betweeen 1 to 20");
    while guess != true {
        let mut number = String::new();
        std::io::stdin().read_line(&mut number).expect("Failed to read input");

        let number: u8 = number.trim().parse().expect("invalid input");

        if my_number == number {
            println!("you guessed the number correctly");
            guess = true;
        }else {
            println!("not correct guess")
        }
    }

    let mut some_vec = vec![45,30,25,23,64,23];
    for i in &some_vec {
        println!("{} some vec value",i);
    }

    let mut var_one = 30;
    loop {
        var_one = var_one - 1;
        if var_one % 13 == 0 {
            break;
        }
    }
    println!("the highest number is {}",var_one);

    let mut var  = 0;
    let mut count = 0;
    loop {
        var = var + 1;
        if var % 5 == 0 && var % 3 == 0 {
            println!("The number is divisible by both 3 and 5 is {} \n",var);
            count = count + 1;
            if count == 3 {
                break;
            }else {
                continue;
            }
        }
        println!("{}",var);
    }
}