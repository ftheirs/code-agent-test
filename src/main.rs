se std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: cargo run <number1> <operator> <number2>");
        process::exit(1);
    }

    let num1 = match args[1].parse::<f64>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Usage: cargo run <number1> <operator> <number2>");
            process::exit(1);
        }
    };

    let operator = args[2].chars().next().unwrap();

    let num2 = match args[3].parse::<f64>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Usage: cargo run <number1> <operator> <number2>");
            process::exit(1);
        }
    };

    let result = match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => {
            if num2 == 0.0 {
                eprintln!("Error: Division by zero is not allowed.");
                process::exit(1);
            } else {
                num1 / num2
            }
        }
        _ => {
            eprintln!("Usage: cargo run <number1> <operator> <number2>");
            process::exit(1);
        }
    };

    println!("{}", result);
}
