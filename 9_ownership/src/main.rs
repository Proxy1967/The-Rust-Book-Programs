// Understanding Ownership - https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html

/// memory safe program

// fn read(y: bool) {
//     if y {
//         println!("y is true");
//     }
// }

// fn main() {
//     let x = true;
//     read(x);
// }

/// memory unsafe program

fn read(y: bool) {
    if y {
        println!("y is true");
    }
}

fn main() {
    read(x); // x is not defined - not in scope
    let x = true;
}