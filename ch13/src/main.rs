use std::thread;
use std::time::Duration;



fn main() {
    let simulated_user_specified_value = 10;
    let simualated_random_number =7;

    generate_workout(
        simulated_user_specified_value, 
        simualated_random_number
    );
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

