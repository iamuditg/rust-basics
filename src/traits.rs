use std::ffi::CString;

/*
1. Traits
*/
struct Person {
    citizenship: String,
    name: String,
    age: u8,
    gender: char,
    salary: i32
}

struct Student {
    name_std: String,
    age: u8,
    sex: char,
    country: String
}

trait GeneralInfo {
    fn info(&self) -> (&str,u8,char);
    fn country_info(&self) -> &str;
}

impl GeneralInfo for Person {
    fn info(&self) -> (&str, u8, char) {
        (&self.name,self.age,self.gender)
    }

    fn country_info(&self) -> &str {
        &self.citizenship
    }
}

impl GeneralInfo for Student {
    fn info(&self) -> (&str, u8, char) {
        (&self.name_std,self.age,self.sex)
    }

    fn country_info(&self) -> &str {
        &self.country
    }
}

struct Data {
    some_data: Vec<i32>
}

trait BasicStats {
    fn mean(&self) -> f32;
    fn variance(&self) -> f32;
}

impl BasicStats for Data {
    fn mean(&self) -> f32 {
        let mut sum = 0;
        for i in self.some_data.iter() {
            sum += *i
        }
        sum as f32 / self.some_data.len() as f32
    }

    fn variance(&self) -> f32 {
       let mu = self.mean();
        let mut sum_squared_diff:f32 = 0.0;
        for i in self.some_data.iter() {
            sum_squared_diff += (*i as f32 - mu)* (*i as f32 -mu)
        }
        sum_squared_diff / self.some_data.len() as f32
    }
}

pub fn traits_ex() {
    let person = Person{
        citizenship: "india".to_string(),
        name: "Udit1".to_string(),
        age: 23,
        gender: 'M',
        salary: 20_0000,
    };

    let student = Student{
        name_std: "Udit".to_string(),
        age: 23,
        sex: 'M',
        country: "India".to_string(),
    };

    println!("{:?}",person.info());
    println!("{:?}",student.info());

    let my_data = Data {
        some_data: vec![5,6,3,6,2,9,3],
    };
    println!("the mean of the data {}",my_data.mean());
    println!("the variance of the data {}",my_data.variance());
}