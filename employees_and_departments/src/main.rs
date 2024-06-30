use std::io;
use std::collections::HashMap;

fn main() {
    println!("Who you want to add where ? - \"Add [employee] to [department]\"");

    let mut input = String::new();
    let mut words = Vec::new();
    let mut employee_to_department = HashMap::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input = input.split_whitespace();

    for word in input {
        words.push(word);
    }

    employee_to_department.insert(words.get(1).unwrap(), words.get(words.len() - 1).unwrap());

    dbg!(employee_to_department);
}
