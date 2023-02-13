// use std::collections::HashMap;
// use std::io;
//
// pub fn collection_example() {
//     let mut n = String::new();
//     let mut name = String::new();
//     let mut no = String::new();
//     let v_name:Vec<String> = Vec::new();
//     let v_no:Vec<String> = Vec::new();
//     let mut c = 1;
//     let mut choice = String::new();
//
//     println!("How many contacts you want to store");
//     io::stdin().read_line(&mut n).expect("Fail");
//     let n:u32 = n.trim().parse().expect("Fail");
//     while (c <= n) {
//         name.clear();
//         no.clear();
//         println!("Enter name than no.");
//
//         io::stdin().read_line(&mut name).expect("Fail");
//         let name:String = name.trim().parse().expect("Failed");
//
//         io::stdin().read_line(&mut no).expect("Fail");
//         let no:String = no.trim().parse().expect("Failed");
//
//         v_name.push(name);
//         v_no.push(no);
//         c += 1
//     }
//     println!("{:?} {:?}",v_name,v_no);
//     let contact:HashMap<&String,&String> = v_name.iter().zip(v_no.iter()).collect();
//     println!("{:?}",contact);
// }
//
//
// fn collections() {
//     // vectors
//     let mut astronauts: Vec<String> = Vec::new();
//     astronauts.push(String::from("Shepard"));
//     astronauts.push(String::from("Grissom"));
//     astronauts.push(String::from("Glenn"));
//     println!("astronauts is {:?}",astronauts);
//     let third = astronauts.get(1);
//     println!("one is {:?}",third);
//
//     // hashMaps
//     let mut mission = HashMap::new();
//     mission.insert("Hadfield",3);
//     mission.insert("Hurley",3);
//     println!("mission is {:?}",mission);
//     let hurley = mission.get("Hurley");
//     println!("hurley is {:?}",hurley);
// }