
/*
1. structs
*/
struct Person {
    citizenship: String,
    name: String,
    age: i32,
    gender: char,
    salary: i32
}

impl Person {
    fn new() -> Self {
        Person {
            citizenship: String::from("USA"),
            name: String::from("ABC"),
            age: 40,
            gender: 'M',
            salary: 35_000,
        }
    }
    fn compute_taxes(&self) -> f32 {
        (self.salary as f32 / 3.) * 0.5
    }
}

struct Numbers(i32,i32);

impl Numbers {
    fn greater(&self) -> i32 {
        if self.0 > self.1 {self.0} else { self.1 }
    }

    fn lesser(&self) -> i32 {
        if self.0 < self.1 {self.0} else {self.1}
    }
}
pub fn stge_example() {
    // structs
    let person1 = Person {
        citizenship: "Udit".to_string(),
        name: "Ind".to_string(),
        age: 40,
        gender: 'M',
        salary: 40_0000,
    };

    println!(" {} {}",person1.salary,person1.citizenship);
    println!("the taxes on person {} is {}",person1.name,person1.compute_taxes());

    let person2 = Person::new();
    println!("{}",person2.salary);

    let person3 = Person{
        name: String::from("Udit"),
        ..person2
    };
    println!("{}",person3.name);

    let some_nums = Numbers(32,16);
    println!("The value of the two fields are {} and {}",some_nums.0,some_nums.1);
    println!("The greater of the two numbers is {}",some_nums.greater());
}