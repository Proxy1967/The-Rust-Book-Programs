// TASK: Convert temperatures between Fahrenheit and Celsius.

use std::io;
use std::io::Write;

fn main() {
    loop {
        // get a number from the user
        println!("");
        print!("Enter temperature (using format 10 C or -40 F) or type q to quit: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error reading");

        let input = input.trim();

        // if q or Q exit the program
        if input.to_lowercase() == "q" {
            break;
        }

        // get the temperature digits and scale from the input
        let (temperature, scale) = input.split_at(input.len() - 1);

        // cast temperature from string to float
        let temperature = temperature.trim().parse::<f64>().unwrap();

        // convert the temperature from one scale to another
        let converted_temperature = convert_temperature(temperature, scale);

        // print the converted temperature
        if scale == "C" {
            println!("{temperature:.2} ˚C = {converted_temperature:.2} ˚F");
        } else if scale == "F" {
            println!("{temperature:.2} ˚F = {converted_temperature:.2} ˚C");
        }
    }
}

fn convert_temperature(temp: f64, scale: &str) -> f64 {
    match scale {
        "C" => (temp * 9.0 / 5.0) + 32.0,
        "F" => (temp - 32.0) * (5.0 / 9.0),
        _ => 0.0,
    }
}
