// fn main() {
    // let v = vec![1, 2, 3, 4, 5];
    
    // let third: &i32 = &v[2];
    // println!("The third element is {third}");
    
    // let third: Option<&i32> = v.get(2);
    // match third {
        // Some(third) => println!("The third element is {third}"),
        // None => println!("There is no third element"),
    // }
// }

// fn main() {
//     let v = vec![100, 32, 57];
//     for n_ref in &v {
//         let n_plus_one: i32 = *n_ref + 1;
//         println!("{n_plus_one}");
//         println!("{n_ref}");
//     }
// }

// fn main() {
//     let mut s1 = String::from("foo");
//     let s2 = "bar";
//     s1.push_str(s2);
//     println!("{s1}");
//     println!("{s2}");
// }


// fn main() {
    // use std::collections::HashMap;

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // // let team_name = String::from("Blue");
    // // let score = scores.get(&team_name).copied().unwrap_or(0); // get returns an Option<&>
    // // println!("{score}");

    // for (key, value) in &scores {
        // println!("{key}: {value}");
    // }

// }

// fn main() {
    // use std::collections::HashMap;

    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");

    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);

    // // using field_name or field_value will throw an error
    // println!("{field_name}");

// }

// fn main() {
    // use std::collections::HashMap;

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Blue"), 25); // overwrite the 10

    // println!("{:?}", scores);
// }

// fn main() {
    // use std::collections::HashMap;

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);

    // scores.entry(String::from("Yellow")).or_insert(50); // will create a new key, value pair `"Yellow": 50`
    // scores.entry(String::from("Blue")).or_insert(50); // will not update the old key, value pair - the value is already present for the key `"Blue": 10`

    // println!("{:?}", scores);
// }

fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // entry looks at the `word` if its in it will return the `count` if not `or_insert(0)` executes
        *count += 1; // dereference `count` because `or_insert()` returns a mut reference (&mut V)
    }

    println!("{:?}", map);
}