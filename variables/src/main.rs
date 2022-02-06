use std::io;

fn main() {
    //------ Muatable Variables

    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    //------ Constant Variables
    // can't be mutated and must be type annotated.
    // by convention written with all caps and underscores.

    // const NUMBER_OF_PLANETS: u32 = 8;
    // const ONE_MILLION: u32 = 1_000_000;
    //
    // println!("{}", NUMBER_OF_PLANETS);
    // println!("{}", ONE_MILLION);
    //
    // //------ Shadowing Variables
    //
    // let x = 5;
    //
    // let x = x + 1;
    //
    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {}", x);
    // }
    //
    // println!("The value of x is: {}", x);
    //
    // let spaces = "  ";
    // let spaces = spaces.len();
    // println!("{}", spaces);

    //------ Data Types
    // Scalar types: integers, floating-point numbers, Booleans, and characters.

    // Integers
    // let a: i32 = 90_222; // Decimal
    // let b: i32 = 0xff; // Hex
    // let c: i32 = 0o77; // Octal
    // let d: i32 = 0b1111_0000; // Binary
    // let e: u8 = b'A'; // Byte (u8 only)

    // integer overflow
    // let f: u8 = 255; // maximum value for u8. 256 would cause integer overflow.

    // Floating-Point type
    // let x = 2.0; // f64 is the default and has double precision
    // let y: f32 = 3.0; // f32 is a single-precision float

    //---- Numeric Operations

    // addition
    // let sum = 5 + 10;
    //
    // // substraction
    // let difference = 95.5 - 4.3;
    //
    // // multiplication
    // let product = 4 * 30;
    //
    // // division
    // let quotient = 56.7 / 32.2;
    // let floored = 2 / 3; // results in 0.
    //
    // // remainder
    // let remainder = 43 % 5;

    ////---- Booleans

    // let t = true;
    // let f: bool = false; // with explicit type annotation.

    ////---- Character

    // let c = 'z';
    // let z = 'ℤ';
    // let heart_eyed_cat = '😻';

    ////---- Compound Types
    // Rust two primitive compound types:
    // tuples and Arrays

    //---- Tuple type
    // A tuple is a general way of grouping together a number of values with a variety of type into
    // one compound type. Tuples have a fixed length: once declare, they cannot grow or shrink in
    // size.

    let tup: (i32, f64, u8) = (500, 6.4, 1); // the types are optional.

    // Destructuring

    let (_x, y, _z) = tup; // place underscores ahead of unused variables.
    println!("the value of y is: {}", y);

    // Dot notation (using a period)

    let five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;

    println!("First value is: {}", five_hundred);

    //--- Array type
    // arrays must have the same time. In Rust arrays must have fixed length.

    let a = [1, 2, 3, 4, 5, 6];
    // or
    let a1: [i32; 6 ] = [7, 8, 9, 10, 11, 12]; // writing the array type.

    // Accessing Array elements

    let one = a[0];
    let twelve = a1[5];
    println!("one: {}, twelve: {}", one, twelve);

    // Invalid array element access.
    // Trying to access and index beyond the array's length causes Rust to panic. 
    // example:

    let b = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = b[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

    // output:
    // Please enter an array index.
    // 6
    // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 6', src/main.rs:138:19
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
}

