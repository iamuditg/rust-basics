use std::io;
use rand;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::fmt;
use std::collections::HashMap;
use crate::guess_game::guess;

mod data_type;
mod guess_game;

fn main() {
    guess();
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









