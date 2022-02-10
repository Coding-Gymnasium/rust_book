fn main() {
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}


// fn five() -> i32 {
//     5
// }
//
// fn main() {
//     let x = five();
//
//     println!("The value of x is: {}", x);
// }

// fn main() {
//     another_function(5, 'h');
//     // let y = 6;
//     let y = {
//         let x = 3;
//         x + 1
//     };
//     println!("The value of y is: {}", y);
// }
//
// fn another_function(value: i32, unit_label: char) {
//     println!("The value of x is: {}{}", value, unit_label);
// }
