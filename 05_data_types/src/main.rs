fn main() {

    // simple scalar data types

    let a = 5; // i32 => signed 32-bit integer

    let b: u64 = 500; // u64 => unsigned 64-bit integer

    let x = 2.0; // f64 => signed 64-bit floating point number - double precision

    let y: f32 = 3.0; // f32 => signed 32-bit floating point number - single precision

    // ---------------------------------------------------------- //

    // numeric operations

    // addition
    let sum = 5 + 10;
    dbg!(sum); // `dbg!` prints and returns the value of a given expression for quick and dirty debugging.

    // subtraction
    let difference = 95.5 - 4.3;
    dbg!(difference);
    
    // multiplication
    let product = 4 * 30;
    dbg!(product);
    
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    dbg!(quotient);
    dbg!(truncated);
    
    // remainder
    let remainder = 43 % 5;
    dbg!(remainder);

    // ---------------------------------------------------------- //

    // booleans

    let t = true;

    let f: bool = false; // with explicit type annotation

    // ---------------------------------------------------------- //

    // characters

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // ---------------------------------------------------------- //

    // compound types

    // tuple

    let tup: (i32, f64, u8) = (500, 6.4, 1); // can contain different data types

    let tup = (500, 6.4, 1); 

    let (x, y, z) = tup; // destructuring a tuple

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0; // access the first element of the tuple

    let six_point_four = x.1; // access the second element of the tuple

    let one = x.2; // access the third element of the tuple

    // array

    let a = [1, 2, 3, 4, 5]; // must contain elements of same data types

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5]; // `[i32; 5]` i32 is the data type of each element, after the semicolon, 5 indicates five array elements 

    let a = [3; 5]; // 5 array elements of value 3
    dbg!(a);

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    dbg!(first, second);

}