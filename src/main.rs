use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: calculator <number1> <operator> <number2>");
        process::exit(1);
    }

    let num1_str = &args[1];
    let operator = &args[2];
    let num2_str = &args[3];

    let num1: f64 = match num1_str.parse() {
        Ok(num) => num,        
        Err(_) => {
            eprintln!("error: invalid number: `{}`", num1_str);
            process::exit(1);
        }
    };

    let num2: f64 = match num2_str.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("error: invalid number: `{}`", num2_str);
            process::exit(1);
        }
    };

    let result = match operator.as_str() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                eprintln!("error: division by zero");
                process::exit(1);
            } else {
                num1 / num2
            }
        }
        _ => {
            eprintln!("error: invalid operator: `{}`", operator);
            process::exit(1);
        }
    };

    println!("{}", result);
}
