use std::io;
use rand;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::fmt;
use std::collections::HashMap;
use crate::arraytuples::{array, tuples};
use crate::flowControl::flow_control;
use crate::function::{add, add_return_type};
use crate::guess_game::guess;
use crate::ownership::ownership;
use crate::structs::Student;

mod data_type;
mod guess_game;
mod flowControl;
mod function;
mod arraytuples;
mod ownership;
mod structs;
mod enums;
mod collections;

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?} {:?}",six,none);

    //let s1 = Student::build(String::from("Rob"),90,95,99);
    //s1.highest();
    //ownership();
    //array();
    // let sum = add_return_type(2,3);
    // println!("{}",sum);
    //flow_control();
    //assignment_one();
}

fn assignment_one() {
    let mut ans = String::new();
    println!("Which is the longest river in world");
    for n in 1 .. 4 {
        ans.clear();
        io::stdin().read_line(&mut ans).expect("Fail");
        ans = ans.trim().to_string();
        if ans == "Nile" {
            println!("you guessed correctly");
            break;
        }else {
            if n != 3 {
                println!("Try Failed");
            }else {
                println!("You are failed");
            }
        }
    }
}


//fn compare_and_print<T: fmt::Display + PartialEq + From<U>,U: fmt::Display + PartialEq + Copy>(a: T,b: U) {
fn compare_and_print<T,U>(a: T,b: U)
 where T: fmt::Display + PartialEq + From<U>,
       U: fmt::Display + PartialEq + Copy
{
    if a == T::from(b) {
        println!("{} is equal to {}",a,b)
    }else {
        println!("{} is NOT equal to {}",a,b);
    }
}









