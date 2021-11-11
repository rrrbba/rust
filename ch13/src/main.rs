use std::thread;
use std::time::Duration;



fn main() {
    //hard coded intensity number, indicates whether user wants low or high intensity workout
    let simulated_user_specified_value = 10;
    // hard coded random number that will generate some variety in the workout plans 
    let simualated_random_number =7;

    generate_workout(
        simulated_user_specified_value, 
        simualated_random_number
    );
}

//stand in for hypothetical calc that takes about 2 seconds to run
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

//Prints the workout plans based on the inputs and calls to sim_exp_cal
fn generate_workout(intensity: u32, random_number: u32) {

    let expensive_result = simulated_expensive_calculation(intensity);
    
    if intensity < 25 {
        println!("Today, do {} pushups!", 
        expensive_result
    );
        println!("Next, do {} situps!", 
        expensive_result
    );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!",
            expensive_result
            );
        }
    }
}