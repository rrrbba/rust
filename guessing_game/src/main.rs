use std::io; //standard library
use rand::Rng;


fn main() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number); //this line only for testing

    println!("Please input your guess:");

    //mutable variable because of mut (immutable would be let guess = ...)
    let mut guess = String::new(); //:: means new is an assoc. function of the String type (an assoc. fn is implemented on a type = static method)

    io::stdin().read_line(&mut guess) //& means that this argument is a reference
        .expect("Failed to read line");

    println!("\nYou guessed: {}", guess);
}


//std fn returns an instance of std::io::Stdin, which is a type that repres. a handle to the standard input for terminal
//reference = Gives a way to let multiple parts of the code access one piece of data without needing to copy it into memory multiple times. They are immutable by default which is why (&mut ref)
// Result type = enumerations (a type that can have a fixed set of values and those values are called the enum's variants)
    //variants of the Result type = Ok / Err
    //the purpose of these Result types is to encode error-handling info
//io::Result has the .expect method that you can call
    //if Err value -> it will display message that is passed in as an argument of expect