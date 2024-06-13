/*
    Rust sometimes rejects both safe and unsafe programs, 
    here are some examples and how to fix them
*/

//** UNSAFE: Returning a reference to the stack
// https://rust-book.cs.brown.edu/ch04-03-fixing-ownership-errors.html#fixing-an-unsafe-program-returning-a-reference-to-the-stack

// fn return_a_string() -> &String {
//     let s = String::from("Hello world"); // declare an immutable variable `s`
//     &s // return a reference to the variables `s`, but at the end of the function `s` is dropped
//        // so the reference is pointing to invalid data
// }

//** UNSAFE: Not Enough Permissions - trying to mutate read-only data, or trying to drop data behind a reference
// https://rust-book.cs.brown.edu/ch04-03-fixing-ownership-errors.html#fixing-an-unsafe-program-not-enough-permissions

// fn main() {
// }

// fn stringify_name_with_title(name: &Vec<String>) -> String { // easies solution is to make name mutable but,
//     name.push(String::from("Esq."));                         // functions should not mutate their inputs if the caller would not expect it
//     let full = name.join(" ");
//     full
// }


//** UNSAFE: Aliasing and Mutating a Data Structure - using a reference to heap data that gets deallocated by another alias 
// https://rust-book.cs.brown.edu/ch04-03-fixing-ownership-errors.html#fixing-an-unsafe-program-aliasing-and-mutating-a-data-structure

// fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
//     let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap(); // immutable reference

//     for s in src {
//         if s.len() > largest.len()  {
//             dst.push(s.clone()); // this line deallocates `dst`, which could invalidate the reference `largest`
//         }
//     }
// }

// ** UNSAFE: Copying vs Moving Out of a Collection 
// https://rust-book.cs.brown.edu/ch04-03-fixing-ownership-errors.html#fixing-an-unsafe-program-copying-vs-moving-out-of-a-collection

// fn main() {
//     let v: Vec<String> = 
//       vec![String::from("Hello world")]; // `v` points to a heap location that points to the String location
//     let s_ref: &String = &v[0]; // `s_ref` points to the `Vec` not the `String`
//     let s: String = *s_ref; // `s` points to the `String`
//     // The problem is after `s` is dropped the `String` is dropped and `s_ref` and `v` point to nothing
//     // Both `Vec` and `String` are collections which data is on the heap, using a `Vec<i32>` won't cause the problem
// }

// ** SAFE: Mutating Different Tuple Fields
// https://rust-book.cs.brown.edu/ch04-03-fixing-ownership-errors.html#fixing-a-safe-program-mutating-different-tuple-fields

// fn main() {
//     let mut a = [0, 1, 2, 3];
//     let x = &mut a[1];
//     *x += 1;
//     println!("{a:?}");
// }


fn main() {
    let mut point = [0, 1];
    let mut x = point[0];
    let y = &mut point[1];
    x += 1;
    *y += 1;
    println!("{} {}", point[0], point[1]);
}