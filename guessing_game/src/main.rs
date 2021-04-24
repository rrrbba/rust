use std::io; //standard library
use std::cmp::Ordering; //enum with variants - less,greater,equal
use rand::Rng;


fn main() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {

        println!("Please input your guess:");

        //mutable variable because of mut (immutable would be let guess = ...)
        let mut guess = String::new(); //:: means new is an assoc. function of the String type (an assoc. fn is implemented on a type = static method)

        io::stdin().read_line(&mut guess) //& means that this argument is a reference
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() { //parse returns a Result type
            //u32 can only contain numerical characters (unsigned 32 bit integers)
            Ok(num) => num,
            Err(_) => continue, //underscore is a catchall value
        };

        println!("\nYou guessed: {}", guess);

        match guess.cmp(&secret_number) { //cmp compares two values
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            } 
        }
    }
}

// Notes
//std fn returns an instance of std::io::Stdin, which is a type that repres. a handle to the standard input for terminal
//reference = Gives a way to let multiple parts of the code access one piece of data without needing to copy it into memory multiple times. They are immutable by default which is why (&mut ref)
// Result type = enumerations (a type that can have a fixed set of values and those values are called the enum's variants)
    //variants of the Result type = Ok / Err
    //the purpose of these Result types is to encode error-handling info
//io::Result has the .expect method that you can call
    //if Err value -> it will display message that is passed in as an argument of expect
//a Match expression is made up of -> arms
    //an Arm consists of a pattern and the code that should be run if the value given to the beginning of the match expression fits that arm's pattern 
//switching from an Expect call to a Match expression is how you generally move from crashing on an error to handeling the error