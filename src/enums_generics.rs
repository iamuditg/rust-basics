

/*
1. Enums
2. Generics
3. Option/Result Enum
*/
enum Conveyance {
    Car = 10,
    Train = 11,
    Air = 12,
}

impl Conveyance {
    fn travel_allowance(&self, miles: i32) -> f32 {
       let allowance = match self {
           Conveyance::Car => miles as f32 * 14.0 * 2.0,
           Conveyance::Train => miles as f32 * 18.0 * 2.0,
           Conveyance::Air => miles as f32 * 30.0 * 2.0,
       };
        allowance
    }
}
pub fn enums_generics() {
    // enums
    let participant_1 = Conveyance::Car;
    let participant_2 = Conveyance::Train;
    let participant_3 = Conveyance::Air;
    //println!("{} {} {}",participant_1 as i32,participant_2 as i32,participant_3 as i32);
    println!("participant_1 {}",participant_1.travel_allowance(60));
    println!("participant_2 {}",participant_2.travel_allowance(60));
    println!("participant_3 {}",participant_3.travel_allowance(60));

    // generics
    println!("square: {}",square(5.6));

    let p1 = Point{ x: 5, y: 6 };
    let p2 = Point{ x: 5.6, y: 6.7 };
    let p3 = Point{ x: 3, y: 2.2 };
    println!("{} {} {}",p1.x,p2.x,p3.x);
    p1.printing();

    // option enum
    let mut disease: Option<String> = None;
    disease = Some(String::from("Diabetes"));
    match disease {
        None => {println!("You donot have any disease")}
        Some(disease_name) => {println!("You have the disease of {}",disease_name)}
    }

    let s1 = Some("Some String");
    println!("{:?} and unwrap {:?}",s1,s1.unwrap());

    let number = Some(6);
    if square_one(number) != None {
        println!("the result of the square {:?}",square_one(Some(6)).unwrap());
    }else {
        println!("We do not have any value");
    }

    // result enum
    println!("{:?}",division(9.0,3.0));
    println!("{:?}",division(5.0,0.0));

    let some_vec_one = vec![5,5,2,1,5,9];
    let result = match some_vec_one.get(5) {
        Some(a) => Ok(a),
        None => Err("The value is not exists")
    };
    println!("result : {:?} ",result);
}

/*enum Result<T,E> {
    ok(T),
    Err(E),
}*/

fn division(divident: f64, divisor: f64) -> Result<f64,String> {
    if divisor == 0.0 {
        Err(String::from("Error: division is zero"))
    }else {
        Ok(divident/divisor)
    }
}

fn square_one(num: Option<i32>) -> Option<i32> {
    match num {
        None => None,
        Some(number) => Some(number * number)
    }
}

struct Point<T,U> {
    x: T,
    y: U
}

impl <T: std::fmt::Debug,U: std::fmt::Debug> Point<T,U> {
    fn printing(&self) {
        println!("{:?} {:?}",self.x,self.y);
    }
}

fn square<T>(x: T) -> T
    where T: std::ops::Mul<Output = T> + Copy{
    x*x
}

// fn square<T: std::ops::Mul<Output = T> + Copy>(x: T) -> T {
//     x*x
// }