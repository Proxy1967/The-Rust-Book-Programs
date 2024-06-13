// fn main() {
//     let mut v: Vec<i32> = vec![1, 2, 3];

//     let num: &mut i32 = &mut v[2];

//     *num += 1;

//     println!("Third element is {}", *num);
    
//     println!("Vector is now {:?}", v);
// }

/// from quiz

// fn incr(n: &mut i32) {
//     *n += 1;
// }

// fn main() {
//     let mut n = 1;
//     incr(&n);         // it does not compile because "&n" is an immutable reference
//     println!("{n}");  // but `incr` takes in a mutable reference
// }

/// from quiz

fn main() {
    let mut s = String::from("hello");
    let s2 = &s; // immutable reference
    let s3 = &mut s; // mutable reference // cannot borrow `s` as mutable because it is also borrowed as immutable

    s3.push_str(" world");
    println!("{s2}");
}