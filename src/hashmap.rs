use std::collections::HashMap;

pub fn hash_map_ex() {
    let mut person:HashMap<&str,i32> = HashMap::new();
    person.insert("a",10);
    person.insert("b",20);
    println!("{:?}",person.get("b").unwrap());

    if person.contains_key("c") {
        println!("not exists");
    }else {
        println!("exits");
    }

    for (name,age) in &person {
        println!("{} {}",name,age);
    }

    let some_vec = vec![5,5,5,2,6,3,2,6,7];
    let mut free_vec:HashMap<i32,u32> = HashMap::new();
    for i in &some_vec {
        let freq : &mut u32 = free_vec.entry(*i).or_insert(0);
        *freq += 1;
    }
    println!("{:?}",free_vec);
}