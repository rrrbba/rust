fn main() {
    //Shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    //Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (y, z, a) = tup; //pattern matching to destructure a tuple value
    println!("The value of y is: {}", y);
    let six_four = tup.1; //accessing an element in a tuple directly
    println!("The value of the second element is: {}", six_four);
}

