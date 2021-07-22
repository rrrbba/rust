use std::env; //bringing parent module into scope
use std::fs;//to handle files
use std::process; //provides abort and exit for terminating the current process
use std::error::Error;

fn main() {

    //Collect command line arguments and prints them
    let args : Vec<String> = env::args().collect();
    //debug formatter
    // println!("{:?}", args); 

    
    let config = Config::new(&args).unwrap_or_else(|err| { //unwrap allows to define custom non-panic error handling
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query); 
    println!("In file {}", config.filename);

    run(config);


}

// Holds the logic that was in the main fn that isn't involved with setting up configuration or handling errors
fn run(config: Config) -> Result<(), Box<dyn Error>> { //fn will return a type that implements the Error trait

    //Takes the filename, opens the file and returns a Result<String> of the file's contents 
    let contents = fs::read_to_string(config.filename)?; //the ? will return the error value from the current fn for the caller to handle
    
    println!("With text:\n{}", contents);

    Ok(()) //means calling run for its side effects only, it doesn't rn a value needed
}

struct Config {
    query: String,
    filename: String,
}

//associates the new fn with Config
impl Config { 

    //Extracted functionality for parsing arguments (holds the logic for determining which argument goes in which variable) 
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        //a lot devs avoid using clone to fix ownership problems because of runtime cost
        let query = args[1].clone(); //program name
        let filename = args[2].clone(); 

        Ok(Config { query, filename })
    }

}