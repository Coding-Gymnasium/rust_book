// fn main() {
//     // let s = "hello"; // type string literal // cannot be mutated
//     // let mut s = String::from("hello"); // type String
//     // s.push_str(", world!"); // push_str() appends a literal to a String
//     // println!("{}", s);
//
//     // let s1 = String::from("hello");
//     // let s2 = s1;
//     //
//     // println!("{}, world", s1); // this erros out because s1 value was 'moved'to s2
//
//     let s1 = String::from("hello");
//     let s2 = s1.clone();
//     println!("s1 = {}, s2 = {}", s1, s2);
//
//     // However is different with integers, because types like integers have a known size at compile.
//     let x = 5;
//     let y = x;
//
//     println!("x = {}, y = {}", x, y);

//     // Ownership and Functions
//     let s = String::from("hello"); // s comes into scope
//     // s's value moves into the function
//     // and so is no longer valid here
//     takes_ownership(s); 

//     let x = 5; // x comes into scope
//
//     // println!("{}", s); // this throws a compile-time error because s was moved to takes_ownership()

//     makes_copy(x)
//     // x would move int the function, but i32 is Copy, 
//     // so it's okay to still user x afterwards
// } // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

// fn takes_ownership(some_string: String) {
//     // some_string comes into scope.
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and 'drop' is called. The backing memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope.
//     println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens


//----------- References and Borrowing ------------- //
//--------------------------------------------------//

fn main() {
    println!("*--- References and Borrowing ---*");
    println!("*--------------------------------*");

    let s1 = String::from("hello"); // not mutable

    let len = calculate_length(&s1); // & indicates it's a reference

    println!("The length of '{}' is {}.", s1, len);

    println!("*--- Mutable References ---*");

    let mut s = String::from("hello"); // mutable
    change(&mut s); // the &mut indicates it's a mutable reference. However there can be up to one mutable reference at a time.
    println!("{}", s);

    let r1  = &mut s;
    // let r2  = &mut s; // this fails becuase there is a second mutable borrow
    // println!("{}, {}", r1, r2); // this fails because references second mutable borrow
    println!("{}", r1);
    let range_to_comma = r1.find(',').unwrap_or(r1.len());
    r1.replace_range(..range_to_comma, "Hola");
    println!("{}", r1);

    //--- Allow for mutliple mutable references
    let mut s2 = String::from("hello");

    {
        let r3 = &mut s2;
        println!("{} r3", r3);
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r4 = &mut s2;
    println!("{} r4", r4);

    //---- Can't combine mutable and immutable references
    let mut s3 = String::from("hello");

    let r5 = &s3; // no problem
    let r6 = &s3; // no problem
    // let r7 = &mut s3; // BIG PROBLEM

    // println!("{}, {}, and {}", r5, r6, r7); // error becuase cannot borrow `s3` as mutable because it is also borrowed as immutable
    println!("{} r5 and {} r6", r5, r6);
    // variables r1 and r2 will not be used after this point

    let r7 = &mut s3; //now it's possible, since the scopes no longer overlap.
    println!("{} r7", r7);

    // ----- Dangling References
    // let reference_to_nothing = dangle(); // errors out because dangle()'s return type contains a borrowed value, but there is no value for it to be borrowed from
    let reference_to_no_dangle = no_dangle();
    println!("{}", reference_to_no_dangle)
}

//------- other functions
/*
fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away. Danger!
*/

fn no_dangle() -> String {
    let s = String::from("hello, no dangle");
    s
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
