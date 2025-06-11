se std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <number1> <operator> <number2>", args[0]);
        process::exit(1);
    }

    let num1: f64 = args[1].parse().unwrap_or_else(|_| {
        eprintln!("Error: First argument is not a valid number.");
        process::exit(1);
    });

    let operator = &args[2];

    let num2: f64 = args[3].parse().unwrap_or_else(|_| {
        eprintln!("Error: Third argument is not a valid number.");
        process::exit(1);
    });

    let result = match operator.as_str() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                eprintln!("Error: Division by zero is not allowed.");
                process::exit(1);
            }
            num1 / num2
        }
        _ => {
            eprintln!("Error: Invalid operator. Supported operators are +, -, *, /.");
            process::exit(1);
        }
    };

    println!("{}", result);
}
