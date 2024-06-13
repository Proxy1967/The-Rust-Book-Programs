// fn main() {

// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// fn first_word(s: &str) -> &str { // `s: &str` instead of `s: &String` so we have more flexibility
//     let bytes = s.as_bytes();

//     for(i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }
//     &s[..]
// }

fn main() {
    let mut s = String::from("hello");
    for &item in s.as_bytes().iter() {
      if item == b'l' {
        s.push_str(" world");
      }
    }
    println!("{s}");
  }