fn main() {
    //--- example 1
    // this code will continue repeating unti manually stopped with ctrl-c

    // loop {
    //     println!("again!");
    // }

    //--- example 2

    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {}", count);
    //     let mut remaining = 10;
    //
    //     loop {
    //         println!("remaining = {}", remaining);
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }
    //
    //     count += 1;
    // }
    //
    // println!("End count = {}", count);

    // --- example 3

    // let mut counter = 0;
    //
    // let result = loop {
    //     counter += 1;
    //     println!("The count is {}", counter);
    //
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    //
    // println!("The result is {}", result);

    // --- example 4

    // let mut number = 3;
    //
    // while number != 0 {
    //     println!("{}!", number);
    //
    //     number -= 1;
    // }
    //
    // println!("LIFTOFF!!!");

    // --- example 5

    // this method is slow.
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // --- example 6
    // using For loop is safer than the example above.

    let b = ["Bob", "Jane", "John", "Mary", "Tim"];

    for element in b {
        println!("the name is: {}", element);
    }

    // --- example 7
    // using for loop and rev (to reverse the range)

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("Take Off!🚀");
}
