// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 33];

//     let result = largest_i32(&number_list);
//     println!("Largest number is {result}");

//     let char_list = vec!['a', 'y', 'm', 'q'];

//     let result = largest_char(&char_list);
//     println!("Largest char is {result}");
// }

//** Using Generic Types
//**

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
    
//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

//** Using generics in Structs

// struct Point<T> {
//     x: T,
//     y: T
// }

// struct PointDiff<T, U> {
//     x: T,
//     y: U
// }

// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 5.1, y: 10.9 };
//     let wont_work = Point { x: 5, y: 10.9 };
//     let will_work = PointDiff { x: 5, y: 10.9 };
// }

//** Using generics in Enums

// #![allow(unused)]
// fn main() {
//     enum Option<T> {
//         Some(T),
//         None,
//     }

//     enum Result<T, E> {
//         Ok(T),
//         Err(E),
//     }
// }

//** Using generics in Methods

// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// fn main() {
//     let p = Point { x: 5, y: 10 };

//     println!("p.x = {}", p.x());
// }

//** Lifetimes **//
//**

// fn main() {
    // let string1 = String::from("abcd");
    // let string2 = "xyz";

    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is {result}");
// }

// fn main() {
    // let string1 = String::from("very long string");

    // {
        // let string2 = String::from("xyz");
        // let result = longest(string1.as_str(), string2.as_str());
        // println!("The longest string is {result}");
    // }
// }


//** DOES NOT WORK
fn main() {
    let string1 = String::from("very long string");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str()); // `string2` goes out of scope after this line, so we would get a dangling reference in `result`
    }                                                         // but we don't because we have specified the lifetimes `'a` of all the references
    println!("The longest string is {result}");               // and the generic lifetime takes the shortest lifetime of the two values passed into the function
}


//** DOES NOT WORK
// We are returning a reference to str, but we do not know which reference we are returning,
// are we returning a reference to `x` or `y` ?

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// WORKS
// The lifetime of the reference returned by the `longest` function is the same as
// the smaller of the lifetimes of the values referred to by the function arguments

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}