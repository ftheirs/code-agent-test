mod calculator;
mod parser;

use std::process;

fn main() {
    let (num1, num2, operator) = match parser::parse_arguments() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    let result = match operator.as_str() {
        "+" => calculator::add(num1, num2),
        "-" => calculator::subtract(num1, num2),
        "*" => calculator::multiply(num1, num2),
        "/" => match calculator::divide(num1, num2) {
            Ok(res) => res,
            Err(err) => {
                eprintln!("Error: {}", err);
                process::exit(1);
            }
        },
        _ => {
            eprintln!("Error: Unsupported operator: {}", operator);
            process::exit(1);
        }
    };

    println!("{}", result);
}
