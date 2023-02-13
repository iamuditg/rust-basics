
fn best_fuel<'a,'b>(x: &'a str,y: &'b str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        x
    }
}

fn lifetimes() {
    let propellant;
    //let rp1 = String::from("RP-1");
    {
        let rp1 = String::from("RP-1");
        propellant = &rp1;
        println!("propellant is {}",propellant);
    }
    //println!("propellant is {}",propellant);
    let result;
    let propellant1 = String::from("RP-1");
    {
        let propellant2 = String::from("LNG");
        result = best_fuel(&propellant1,&propellant2);
    }
    println!("result is {}",result);
}

