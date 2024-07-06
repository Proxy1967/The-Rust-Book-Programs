// use std::fs::File;

// fn main() {
//     let greeting_file_result = File::open("hello.txt");

//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => panic!("Problem opening the file: {:?}", error)
//     };
// }

// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let greeting_file_result = File::open("hello.txt");

//     // The Err variant that `File::open` returns is `io::Error` which has a method called `kind()`
//     // we can call that method to get `io::ErrorKind` value which we use in another `match` statement

//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") { // `File::create` also returns `Result(T, E)` so we use another `match`
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {:?}", e)
//             },
//             other_error => {
//                 panic!("Problem opening the file: {:?}", other_error);
//             }
//         }
//     };
// }

// use std::fs::File;
// use std::io::{self, Read};

// fn main() {
//     fn read_username_from_file() -> Result<String, io::Error> {
//         let username_file_result = File::open("hello.txt");

//         let mut username_file = match username_file_result {
//             Ok(file) => file,
//             Err(e) => return Err(e)
//         };

//         let mut username = String::new();

//         match username_file.read_to_string(&mut username) {
//             Ok(_) => Ok(username),
//             Err(e) => Err(e)
//         }
//     }
// }

//** The `?` operator acts similarly to `match`. 
//** If the returned value is `Result` it will return either the `Ok` or the `Err` value
//** It is used mainly for propagating errors, if a function returns a `Result` or `Option` value,
//** we can use the `?` op and the function will return the appropriate values

// use std::fs::File;
// use std::io::{self, Read};

// fn main() {
//     fn read_username_from_file() -> Result<String, io::Error> {
//         let mut username_file = File::open("hello.txt")?;
//         let mut username = String::new();
    
//         username_file.read_to_string(&mut username)?;
//         Ok(username)
//     }    
// }

//** Shorter version of the previous program

// use std::fs::File;
// use std::io::{self, Read};

// fn main() {
    // fn read_username_from_file() -> Result<String, io::Error> {
        // let mut username = String::new();
    
        // File::open("hello.txt")?.read_to_string(&mut username)?;
    
        // Ok(username)
    // }    
// }

//** Even shorter version using only `fs::read_to_string()`
//** This opens the file, creates a new `String`, reads the contents of the file
//** puts the contents into that `String`, and returns it.

use std::fs;
use std::io;

fn main() {
    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }
}
