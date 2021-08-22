use std::env; //bringing parent module into scope
use std::process; //provides abort and exit for terminating the current process

use minigrep::Config;

fn main() {

    //Collect command line arguments and prints them
    let args : Vec<String> = env::args().collect();
    //debug formatter
    // println!("{:?}", args); 

    
    let config = Config::new(&args).unwrap_or_else(|_err| { //unwrap allows to define custom non-panic error handling
        process::exit(1);
    });


    //if run returns an err value, call prcoess exit 1
    if let Err(_e) = minigrep::run(config) { 

        process::exit(1);
    }


}

