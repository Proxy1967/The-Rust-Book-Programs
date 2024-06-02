// TASK: Generate the nth Fibonacci number.
use std::io;
use std::io::Write;

fn main() {
    loop {
        // take input from the user
        println!("");
        print!("Enter the index of the Fibonacci number you want to generate ");
        io::stdout().flush().unwrap();

        let mut n: String = String::new();
        io::stdin().read_line(&mut n).expect("Error reading");

        let n = n.trim().parse::<i32>().unwrap();

        // compute and print out the n-th Fibonacci number
        println!(
            "The {n}-th Fibonacci number is {}",
            generate_fibonacci_number(n)
        );
    }
}

fn generate_fibonacci_number(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return generate_fibonacci_number(n - 1) + generate_fibonacci_number(n - 2);
    }
}
