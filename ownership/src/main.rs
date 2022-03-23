fn main() {
    // let s = "hello"; // type string literal // cannot be mutated
    // let mut s = String::from("hello"); // type String
    // s.push_str(", world!"); // push_str() appends a literal to a String
    // println!("{}", s);

    // let s1 = String::from("hello");
    // let s2 = s1;
    //
    // println!("{}, world", s1); // this erros out because s1 value was 'moved'to s2

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // However is different with integers, because types like integers have a known size at compile.
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // Ownership and Functions
    let s = String::from("hello"); // s comes into scope
    // s's value moves into the function
    // and so is no longer valid here
    takes_ownership(s); 

    let x = 5; // x comes into scope

    // println!("{}", s); // this throws a compile-time error because swas moved to takes_ownership()

    makes_copy(x)
    // x would move int the function, but i32 is Copy, 
    // so it's okay to still user x afterwards
} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope.
    println!("{}", some_string);
} // Here, some_string goutes out of scope and 'drop' is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope.
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens
