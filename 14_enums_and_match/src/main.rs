// The standard library has Ipv4 and Ipv6 definition - https://doc.rust-lang.org/std/net/enum.IpAddr.html

// #![allow(unused)]
// fn main() {
//     enum IpAddr {
//         V4(u8, u8, u8, u8),
//         V6(String),
//     }
    
//     let home = IpAddr::V4(127, 0, 0, 1);
    
//     let loopback = IpAddr::V6(String::from("::1"));
// }

//** match control flow

// fn main() {
//     fn plus_one(x: Option<i32>) -> Option<i32> {
//         match x {
//             None => None,
//             Some(i) => Some(i + 1),
//         }
//     }
    
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);

//     dbg!(five);
//     dbg!(six);
//     dbg!(none);
// }

//** if let syntax

/* 
  The syntax if let takes a pattern and an expression separated by an equal sign.
  It works the same way as a match, where the expression is given to the match and the pattern is its first arm.
  In this case, the pattern is Some(max), and the max binds to the value inside the Some.
  We can then use max in the body of the if let block in the same way we used max in the corresponding match arm.
  The code in the if let block isn’t run if the value doesn’t match the pattern.
*/

fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max { // `Some(max)` is the pattern, and `config_max` the expression
        println!("The maximum is configured to be {}", max);
    }
}
