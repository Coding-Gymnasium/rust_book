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
}
