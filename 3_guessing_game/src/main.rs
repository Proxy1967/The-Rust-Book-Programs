use rand::Rng; // to generate a random number
use std::cmp::Ordering; // Ordering has enums Less, Greater, and Equal
use std::io; // for input/output // to generate random numbers

fn main() {
    println!("Guess the number!"); // print the name of the game

    // to track number of guesses a player made
    let mut num_of_guesses: u32 = 0;

    let secret_number = rand::thread_rng().gen_range(1..=100); // generate a random number that is local to the current thread of execution, `gen_range` is inclusive of 1 and 100

    println!("The secret number is: {secret_number}"); // USED ONLY FOR DEBUGGING, NOT IN FINAL CODE

    loop {
        println!("Please input your guess."); // ask the user for an input

        let mut guess = String::new(); // create a new mutable string variable `guess`

        io::stdin() // call the standard input `stdin()` function from the standard library `io`
            .read_line(&mut guess) // call `read_line` from standard input passing a mutable reference to the `guess` variable
            .expect("Failed to read line"); // `read_line` returns a `Result` value, it returns `Ok` and `Err`,
                                            // using `expect()` we crash the program with the string inputted as the argument to the function

        // convert string `guess` to u32
        // `trim()` gets rid of whitespace and \n \r, `parse()` converts a string to another type and returns an enum `Result` (`Ok` and `Err`)
        let guess: u32 = match guess.trim().parse() { // match the `Result` enum of `parse()`
            Ok(num) => num, // if `Result` is `Ok` it returns a value `num` which we will save in `guess`
            Err(_) => continue, // `_` is a catchall value
        };

        // increment the number of guesses a player made
        num_of_guesses += 1;

        println!("You guessed: {guess}"); // print the `guess` variable

        // compare guess and secret_number
        // match the result of the comparison to either Ordering::Less or Ordering::Greater or Ordering::Equal
        // when matched run the appropriate function
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => { 
                println!("You win!");
                println!("Number of guesses: {num_of_guesses}\n");
                break;
            }
        }
    }
}
