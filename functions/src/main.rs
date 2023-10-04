// Parameters - one parameter

// fn main() {
//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }

// ---------------------------------------------------------- //

// Parameters - two parameters

// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// ---------------------------------------------------------- //

// Statements and Expressions

// fn main() {
//     let y = {
//         let x = 3; // this is a statement, statements do not return values, so you cant do let x = y = 6;
//         x + 1 // this doesn't have a semicolon, expressions do not include ending semicolons, Expressions evaluate to a value
//               // If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value
//     };

//     println!("The value of y is: {y}"); // y = 4
// }

// ---------------------------------------------------------- //

// Function with return values

// fn five() -> i32 { // `->` means returns, in this case i32, we don't need to name the return value
//     5 // we don't have to use the `return` keyword because Rust returns the value of the final expression of the function
//       // if we have to return a value before the end of the function we can use the `return` keyword
// }

// fn main() {
//     let x = five();

//     println!("The value of x is: {x}");
// }

// Another example

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1 // this is an expression so no need for a semicolon, if we put a semicolon it would become a statement
          // statements do not return any values which is expressed by `()` unit type
}