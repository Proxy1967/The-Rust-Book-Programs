use std::collections::HashMap;

fn main() {
    let mut vec = vec![1, 5, 10, 2, 15, 5, 20, 11, 21];

    vec.sort();

    println!("sorted: {:?}", vec);

    // median
    
    // if length of vec is odd get the middle value
    // else add up two middle values and divide by 2
    if vec.len() % 2 != 0 {
        println!("odd");
        let middle_index = vec.len() / 2;
        println!("median: {}", vec[middle_index]);
    } else {
        println!("even");
        let middle_index = vec.len() / 2;
        let median = (vec[middle_index - 1] + vec[middle_index]) / 2;
        println!("median: {}", median);
    }

    // mode

    let mut map = HashMap::new();
    let mut freq = 0;
    let mut mode = 0;

    // get the amount of times each value occurs
    for value in vec {
        let count = map.entry(value).or_insert(0);
        *count += 1;
    }

    println!("mode map: {:?}", map);

    // get the value of mode
    for (key, value) in map {
        if value > freq {
            mode = key;
            freq = value;
        }
    }

    println!("mode: {}", mode);

}
