
fn process_fuel(propellant: String) -> (String,usize) {
    println!("processing propellant {} ...",propellant);
    let length = propellant.len();
    (propellant,length)
}

fn ownership() {
    // shadowing variables
    let planet = "Earth";
    {
        println!("planet is {}", planet);
        let mut planet = 4;
        println!("value is {}",planet);
    }
    println!("planet is {}", planet);

    // string
    let mut message = String::from("Earth");
    message.push_str(" is home");
    println!("message is {}",message);

    //ownership
    let outer_planet: String;
    {
        let mut inner_planet = String::from("Mercury");
        outer_planet = inner_planet.clone();
        inner_planet.clear();
        println!("inner planet is {}",inner_planet);
    }
    println!("outer planet is {}",outer_planet);


    let rocket_fuel = String::from("RP-1");
    let (rocket_fuel,length) = process_fuel(rocket_fuel);
    println!("rocket_fuel is {} and length is {}",rocket_fuel,length);
}