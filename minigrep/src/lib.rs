use std::error::Error;
use std::fs;
use std::env;


pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
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
        
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

// Holds the logic that was in the main fn that isn't involved with setting up configuration or handling errors
pub fn run(config: Config) -> Result<(), Box<dyn Error>> { //fn will return a type that implements the Error trait

    //Takes the filename, opens the file and returns a Result<String> of the file's contents 
    let contents = fs::read_to_string(config.filename)?; //the ? will return the error value from the current fn for the caller to handle

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(()) //means calling run for its side effects only, it doesn't rn a value needed
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let mut results = Vec::new();
    
    for line in contents.lines(){ //line is method that handles line-by-line iteration of strings
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

//lowercase the query and the line before comparing them
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        //calls on each line before checking whether it contains query to lowercase all characters
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
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