//Example 1 ---->
#[derive(Debug)]
pub struct Student {
    name: String,
    c: i32,
    java: i32,
    rust: i32
}

impl Student {
  pub fn highest(&self) {
        if self.c > self.java && self.c > self.rust {
            println!("Highest marks in C");
        }else if self.java > self.c && self.java > self.rust{
            println!("highest marks in java");
        }else {
            println!("highest marks in rust");
        }
    }
   pub fn build(name: String,c: i32,java: i32,rust: i32) -> Student {
        Student {
            name,c,java,rust
        }
    }
}


// Example 2 --->
impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn new(name: &str) -> Shuttle {
        Shuttle{
            name: String::from(name),
            crew_size: 7,
            propellant:0.0
        }
    }
}

struct Color(u8,u8,u8);

#[derive(Debug)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

fn structs() {
    let mut vehicle = Shuttle{
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 856564.0
    };


    let vehicle2 = Shuttle{
        name: String::from("discover"),
        ..vehicle
    };

    let vehicle_name = vehicle.get_name();
    println!("name is {}", vehicle_name);
    println!("name is {}", vehicle.name);
    vehicle.name = String::from("Atlantis");
    println!("vehicle is {:?}",vehicle);
    println!("vehicle is {:?}",vehicle2);

    //let mut veh = Shuttle::new("hdhd");
    //print!("veh is {}",veh);

    let red = Color(22,0,0);
    print!("first value is {}",red.0);
}

struct Person {
    citizenship: String,
    name: String,
    age: i32,
    gender: char,
    salary: i32
}

impl Person {
    fn new() -> Self {
        Person{
            citizenship: String::from("Nouman Azam"),
            name: String::from("Pakistan"),
            age: 40,
            gender: 'M',
            salary: 40_000,
        }
    }
    fn compute_taxes(&self) -> f32 {
        (self.salary as f32 /3.) * 0.5
    }
}

pub fn struct_example() {
    println!("struct example");
    
    let person1: Person = Person{
        citizenship: String::from("Nouman Azam"),
        name: String::from("Pakistan"),
        age: 40,
        gender: 'M',
        salary: 40_000,
    };

    println!("{} {} {}",person1.citizenship,person1.salary,person1.gender);
    println!("{}",person1.compute_taxes());

    let person2 = Person::new();
    println!("{:?}",person2.salary)
}