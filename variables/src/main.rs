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
    let a: i32 = 90_222; // Decimal
    let b: i32 = 0xff; // Hex
    let c: i32 = 0o77; // Octal
    let d: i32 = 0b1111_0000; // Binary
    let e: u8 = b'A'; // Byte (u8 only)

    // integer overflow
    let f: u8 = 255; // maximum value for u8. 256 would cause integer overflow.
}
