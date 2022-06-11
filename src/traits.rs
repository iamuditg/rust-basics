
#[derive(PartialEq)]
struct Satellite {
    name: String,
    velocity: f64
}
struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32
}

trait Description {
    fn describe(&self) -> String;
}

impl Description for Satellite {
    fn describe(&self) -> String {
        format!("the {} flying at {} miles per second!",self.name,self.velocity)
    }
}

impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!("the {} flying at {} miles high with {}",self.name,self.altitude,self.crew_size)
    }
}

fn get_displayable() -> impl fmt::Display {
    13
}

fn traits() {
    let hubble = Satellite{ name: String::from("Hubble Telescope"), velocity: 4.72 };
    let hubble2 = Satellite{ name: String::from("Hubble Telescope"), velocity: 4.75 };
    println!("equal is {}",hubble == hubble2);
    let iss = SpaceStation{
        name: String::from("international space station"),
        crew_size: 6,
        altitude: 254
    };
    println!("hubble is {}",hubble.describe());
    println!("iss is {}",iss.describe());
    println!("output is {}",get_displayable());
}