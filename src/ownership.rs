use std::env::var;
use crate::function::{heap_function, stack_func};

fn process_fuel(propellant: String) -> (String, usize) {
    println!("processing propellant {} ...",propellant);
    let length = propellant.len();
    (propellant,length)
}

pub fn ownership() {
    // shadowing variables
    let planet = "Earth";
    {
        println!("planet is {}", planet);
        let mut planet = 4;
        println!("value is {}",planet);
    }
    println!("planet is {}", planet);

    // string
    let mut message = String::from("Earth");
    message.push_str(" is home");
    println!("message is {}",message);

    //ownership
    let outer_planet: String;
    {
        let mut inner_planet = String::from("Mercury");
        outer_planet = inner_planet.clone();
        inner_planet.clear();
        println!("inner planet is {}",inner_planet);
    }
    println!("outer planet is {}",outer_planet);


    let rocket_fuel = String::from("RP-1");
    let (rocket_fuel,length) = process_fuel(rocket_fuel);
    println!("rocket_fuel is {} and length is {}",rocket_fuel,length);

    let x: f64 = 32.6;
    let y: f64 = x;

    println!("x: {}, y: {}",x,y);

    let s1: String = String::from("abc");
    let s2: &String = &s1;
    println!("s1: {}, s2: {}",s1,s2);

    let vec_1: Vec<i32> = vec![5,6,7,3,2];
    let vec_2: Vec<i32> = vec_1.clone();

    println!("Vec 1 : {:?}, Vec2: {:?}",vec_1,vec_2);

    let stack_num: i32 = 32;
    let mut heap_vec: Vec<i32> = vec![4,5,6];

    stack_func(stack_num);
    println!("{}",stack_num);

    heap_function(&mut heap_vec);
    println!("the value {:?}",heap_vec);
}