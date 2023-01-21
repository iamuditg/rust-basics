
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

fn enums() {
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