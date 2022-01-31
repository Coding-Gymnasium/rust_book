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
    let sum = 5 + 10;

    // substraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // results in 0.
    
    // remainder
    let remainder = 43 % 5;
}
