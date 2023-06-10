
struct Person {
    citizenship: String,
    name: String,
    age: u8,
    gender: char,
    salary: i32,
}

struct Student {
    name_std: String,
    age: u8,
    sex: char,
    country: String,
}

trait GeneralInfor {
    fn info(&self) -> (&str,u8,char);

    fn country_info(&self) -> &str;
}

impl GeneralInfor for Person {
    fn info(&self) -> (&str, u8, char) {
        (&self.name,self.age,self.gender)
    }

    fn country_info(&self) -> &str {
        &self.citizenship
    }
}

impl GeneralInfor for Student {
    fn info(&self) -> (&str, u8, char) {
        (&self.name_std,self.age,self.sex)
    }

    fn country_info(&self) -> &str {
        &self.country
    }
}

pub fn trait_main() {
    let person1: Person = Person {
        citizenship: String::from("Noe"),
        name: String::from("Pakistan"),
        age: 40,
        gender: 'M',
        salary: 40_000,
    };

    let student1 = Student{
        name_std: String::from("aam"),
        age: 15,
        sex: 'M',
        country: String::from("USA"),
    };

    println!("the basic person {:?}",person1.info());
    println!("the basic student {:?}",person1.info());

}