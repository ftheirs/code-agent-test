use std::env;
use std::process;

mod calculator;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Error: Invalid input.");
        process::exit(1);
    }

    let first_number = match args[1].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Invalid input.");
            process::exit(1);
        }
    };

    let operator = &args[2];

    let second_number = match args[3].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Invalid input.");
            process::exit(1);
        }
    };

    let result = calculator::calculate(first_number, operator, second_number);

    match result {
        Ok(value) => println!("Result: {}", value),
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}
