use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: cli-calculator <number1> <operator> <number2>");
        std::process::exit(1);
    }

    let number1: f64 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: Invalid number '{}'", args[1]);
            std::process::exit(1);
        }
    };

    let operator = &args[2];

    let number2: f64 = match args[3].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: Invalid number '{}'", args[3]);
            std::process::exit(1);
        }
    };

    let result = match operator.as_str() {
        "+" => cli_calculator::add(number1, number2),
        "-" => cli_calculator::subtract(number1, number2),
        "*" => cli_calculator::multiply(number1, number2),
        "/" => match cli_calculator::divide(number1, number2) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!("Error: Invalid operator '{}'", operator);
            std::process::exit(1);
        }
    };

    println!("{}", result);
}