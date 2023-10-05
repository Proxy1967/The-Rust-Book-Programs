// `loop` loops the program forever or until you explicitly tell it to stop

// fn main() {
//     loop {
//         println!("again!");
//     }
// }

// ---------------------------------------------------------- //

// Returning values from loops

// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}");
// }

// ---------------------------------------------------------- //

// Loop labels to disambiguate between multiple loops

// fn main() {
//     let mut count = 0;
//     'counting_up: loop { // notice the out loop has the label 'counting_up
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break; // this breaks the inner loop
//             }
//             if count == 2 {
//                 break 'counting_up; // the label 'counting_up is used here to break the outer loop
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }

// ---------------------------------------------------------- //

// Conditional loops with while

// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

// ---------------------------------------------------------- //

// Looping through a collection with for

// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("the value is: {}", a[index]);

//         index += 1;
//     }
// }

// ---

// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a {
//         println!("the value is: {element}");
//     }
// }

// ---

fn main() {
    for number in (1..4).rev() { // loop by starting from 1 and ending before the last number - 4 in this case
        println!("{number}!");   // rev() reverses the range
    }
    println!("LIFTOFF!!!");
}