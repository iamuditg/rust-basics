

fn collections() {
    // vectors
    let mut astronauts: Vec<String> = Vec::new();
    astronauts.push(String::from("Shepard"));
    astronauts.push(String::from("Grissom"));
    astronauts.push(String::from("Glenn"));
    println!("astronauts is {:?}",astronauts);
    let third = astronauts.get(1);
    println!("one is {:?}",third);

    // hashMaps
    let mut mission = HashMap::new();
    mission.insert("Hadfield",3);
    mission.insert("Hurley",3);
    println!("mission is {:?}",mission);
    let hurley = mission.get("Hurley");
    println!("hurley is {:?}",hurley);
}