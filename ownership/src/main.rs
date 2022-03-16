fn main() {
   // let s = "hello"; // type string literal
    let mut s = String::from("hello"); // type String
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s);
}
