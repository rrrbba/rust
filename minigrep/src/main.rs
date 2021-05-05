use std::env; //bringing parent module into scope

fn main() {

    //Collect command line arguments and prints them
    let args : Vec<String> = env::args().collect();
    println!("{:?}", args); //debug formatter

    let query = &args[1]; //program name
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}