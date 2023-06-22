
// -------------------------------------------------------
//                    Generics
// -------------------------------------------------------

use std::process::Output;

struct Point<T, U> {
    x: T,
    y: U
}

impl<T: std::fmt::Debug,U: std::fmt::Debug> Point<T,U> {
    fn printing(&self) {
        println!("The value of the point coordinates are {:?}, {:?}",self.x,self.y);
    }
}

pub fn square<T: std::ops::Mul<Output= T> + Copy> (x: T) -> T {
    x*x
}


pub fn generics() {
    let p1: Point<i32,i32> = Point{x: 5,y: 5};
    let p2: Point<f64,f64> = Point{x: 1.0,y: 4.0};
    p1.printing();
    p2.printing();
}