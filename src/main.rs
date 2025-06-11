se std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <number1> <operator> <number2>", args[0]);
        eprintln!("Supported operators: +, -, *, /");
        process::exit(1);
    }

    let num1_str = &args[1];
    let operator_str = &args[2];
    let num2_str = &args[3];

    let num1: f64 = match num1_str.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: First argument '{}' is not a valid number.", num1_str);
            process::exit(1);
        }
    };

    let num2: f64 = match num2_str.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Third argument '{}' is not a valid number.", num2_str);
            process::exit(1);
        }
    };

    let operator = match operator_str.as_str() {
        "+" => '+',
        "-" => '-',
        "*" => '*',
        "/" => '/',
        _ => {
            eprintln!("Error: Invalid operator '{}'. Supported operators are +, -, *, /.", operator_str);
            process::exit(1);
        }
    };

    let result = match calculate(num1, operator, num2) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    println!("{}", result);
}

fn calculate(num1: f64, operator: char, num2: f64) -> Result<f64, String> {
    match operator {
        '+' => Ok(num1 + num2),
        '-' => Ok(num1 - num2),
        '*' => Ok(num1 * num2),
        '/' => {
            if num2 == 0.0 {
                Err("Division by zero is not allowed.".to_string())
            } else {
                Ok(num1 / num2)
            }
        },
        _ => Err(format!("Invalid operator: {}", operator)), // Should ideally not be reached due to earlier validation
    }
}
