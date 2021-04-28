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

    //Arrays
    //name: [type; number of elements] = [array elements]
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    //initializing an array that contain the same value for each element -> [initial value; number of elements] 
    let c = [3; 5]; // [3, 3, 3, 3, 3]
    //Accessing elements in an array
    let c_second = c[1];
    println!("The value of the second element in array c is: {}", c_second);

    //Functions
    println!("Hello");
    another_function();
}

//Could've defined this function before the main function as well, rust doesn't care where
fn another_function() {
    println!("Another function!");
}

