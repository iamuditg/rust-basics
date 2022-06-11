
fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (index,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index]
        }
    }
    &s
}

fn produce_dangling_fuel() -> String {
    let new_fuel = String::from("RP-1");
    new_fuel
}

fn process_fuel_references(propellant: &mut String) -> usize {
    println!("processing borrowing references propellant {} ...",propellant);
    // mutable references
    propellant.push_str(" is highly flammable");
    let length = propellant.len();
    length
}


fn references() {
    // borrowing references
    let mut rocket_fuel = String::from("RP-1");
    let length = process_fuel_references(&mut rocket_fuel);
    println!("rocket_fuel is {} and length is {}",rocket_fuel,length);
    // dangling references
    let rocket_fuel = produce_dangling_fuel();
    println!("rocket_fuel is {}",rocket_fuel);

    // slices
    let message = String::from("Greeting from Earth!");
    let last_word = &message[15..15+5];
    println!("last_word is {}",last_word);
    let first_word = get_first_word(&message);
    println!("first_word is {}",first_word);
}