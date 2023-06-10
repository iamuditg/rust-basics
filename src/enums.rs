
#[allow(dead_code)]
enum coin {
    Penny,
    Nickel,
    Dime,
    Quater,
}

fn value_in_center(c: coin) -> u32 {
    match c {
        coin::Penny => {
            println!("Penny");
            1
        },
        coin::Nickel => 5,
        coin::Dime => 10,
        coin::Quater => 25,
    }
}

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64,f64),
    Triangle(f64,f64,f64)
}

pub fn enums() {
    let my_shape = Shape::Rectangle(1.2,1.4);
    println!("my shape {:?}",my_shape);
    // match operator
    match my_shape {
        Shape::Circle(r) => println!("Circle with radius {}",r),
        Shape::Rectangle(w,h) => println!("{} x {} Rectangle",w,h),
        Shape::Triangle(a,b,c) => println!("Triangle with sides {} {} {}",a,b,c)
    }

    let number = 4u8;
    let result = match number {
        0 => "zero",
        1 => "one",
        _ => {
            println!("{} did not match",number);
            "something else"
        }
    };
    println!("{}",result)

}

enum Conveyance {
    Car = 15,
    Train = 20,
    Air = 30,
}

impl Conveyance {
    fn travel_allowance(&self, miles: i32) -> f32 {
      let allowance =  match self {
            Conveyance::Car => miles as f32 * 14.0 * 2.0,
            Conveyance::Train => miles as f32 * 18.0 * 2.0,
            Conveyance::Air => miles as f32 * 30.0 * 2.0
        };
        allowance
    }
}

pub fn enum_ex() {
    let participant_1 = Conveyance::Car;
    let participant_2 = Conveyance::Train;
    let participant_3 = Conveyance::Air;

    //println!("the value of the option is {}", participant_1 as i32);
    println!("the participant 1 has a car allowance of {}",participant_1.travel_allowance(60));

    println!("the participant 2 has a train allowance of {}",participant_2.travel_allowance(120));

    println!("the participant 3 has a air allowance of {}",participant_3.travel_allowance(60));
}