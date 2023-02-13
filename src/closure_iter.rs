use std::thread;
use std::time::Duration;

pub fn example() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity:u32,random_number: u32) {
    // let expensive_result = simulated_expensive_calculation(intensity);
    let expensive_closure = | num | {
        println!("Calculating Slowly....");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        println!("Today do {} pushups",expensive_closure(intensity));
        println!("Today do {} situps",expensive_closure(intensity));
    }else {
        if random_number == 3 {
            println!("Today take a breadk");
        }else {
            println!("Today run for {} minutes",expensive_closure(intensity))
        }
    }
}