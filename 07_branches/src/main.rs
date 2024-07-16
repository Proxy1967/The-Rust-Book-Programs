// Simple if statement

// fn main() {
//     let number = 7;

//     if number < 5 { // this condition MUST evaluate to a boolean value
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }

// ---------------------------------------------------------- //

// Handling multiple condition with else if

// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3"); // Rust only executes the block for the first true condition
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2"); // thats why this block is not executed
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// ---------------------------------------------------------- //

// Using if in a let statement

fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 }; // expressions must be of the same type
    let number = if condition { 5 } else { "six" }; // this does not compile

    println!("The value of number is : {number}");
}