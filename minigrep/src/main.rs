use std::env; //bringing parent module into scope
use std::fs;//to handle files

fn main() {

    //Collect command line arguments and prints them
    let args : Vec<String> = env::args().collect();
    //debug formatter
    // println!("{:?}", args); 

    //Passing the vector to parse_config
    let (query, filename) = parse_config(&args);

    println!("Searching for {}", query);
    println!("In file {}", filename);

    //Takes the filename, opens the file and returns a Result<String> of the file's contents
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

//Extracted functionality for parsing arguments (holds the logic for determining which argument goes in which variable)
fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1]; //program name
    let filename = &args[2];

    (query, filename)
}