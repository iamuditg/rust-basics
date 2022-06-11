use rand::Rng;
use std::io;

pub fn guess() {
    println!("Guess the number!");

    // let secret_number = rand::thread_rng().gen_range(1,101);
    // println!("the secret number is: {}",secret_number);

    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guesses {}", guess);
}