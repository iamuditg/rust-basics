use std::io;
use rand;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::fmt;
use std::collections::HashMap;
use crate::arraytuples::{array, tuples};
use crate::data_type::data_types;
use crate::enums::{enum_ex, enums};
use crate::flowControl::flow_control;
use crate::function::{add, add_return_type};
use crate::generics::square;
use crate::guess_game::guess;
use crate::ownership::ownership;
use crate::problem_one::search_anagram;
use crate::structs::{struct_example, Student};
use crate::traits::trait_main;

mod data_type;
mod guess_game;
mod flowControl;
mod function;
mod arraytuples;
mod ownership;
mod structs;
mod enums;
mod collections;
mod generics;
mod lifetimes;
mod traits;
mod smartpointers;
mod problem_one;

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn main() {
   let words = vec!["the".to_string(),"teh".to_string(),
   "het".to_string(),"stupid".to_string(),"studpi".to_string(),
   "apple".to_string(),"appel".to_string()];

    let grouping = search_anagram(words);

    let input_word = String::from("tech");
    for i in grouping.into_iter() {
        if i.contains(&input_word) {
            println!("The group of the word is {:?}",i)
        }
    }
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









