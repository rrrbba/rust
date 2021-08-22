use std::error::Error;
use std::fs;


pub struct Config {
    pub query: String,
    pub filename: String,
}

//associates the new fn with Config
impl Config {
    //Extracted functionality for parsing arguments (holds the logic for determining which argument goes in which variable)
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        //a lot devs avoid using clone to fix ownership problems because of runtime cost
        let query = args[1].clone(); //program name
        let filename = args[2].clone(); 

        Ok(Config { query, filename })
    }
}

// Holds the logic that was in the main fn that isn't involved with setting up configuration or handling errors
pub fn run(config: Config) -> Result<(), Box<dyn Error>> { //fn will return a type that implements the Error trait

    //Takes the filename, opens the file and returns a Result<String> of the file's contents 
    let contents = fs::read_to_string(config.filename)?; //the ? will return the error value from the current fn for the caller to handle
    
    // println!("With text:\n{}", contents);

    Ok(()) //means calling run for its side effects only, it doesn't rn a value needed
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

//Test for taking a query and the text to search for the query in, and it will only return the lines from the text that contain the query
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}