fn main() {
    // let number = 6;

    // --- example 1
    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }

    // --- example 2
    // if number != 0 {
    //     println!("number was something other than zero");
    // }

    // --- example 3

    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }


    // --- example 4
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // the result of each arm must be of the same type. It can be mixed, for example:
    // if condition { 5 } else { 'six' } throws an error.

    println!("The value of the number is: {}", number);
}

//// ------ NOTE
// the condition must be a bool.
// The example below would throw an error. It should get a boolean but got an integer.
    // if number {
    //     println!("condition was true");
    // } else {
//
// When checking multiple conditions Rust executes the first body for which the condition is true.
