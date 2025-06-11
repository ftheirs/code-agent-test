use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: calculator <number1> <operator> <number2>");
        process::exit(1);
    }

    let num1 = args[1].parse::<f64>().unwrap();
    let operator = &args[2];
    let num2 = args[3].parse::<f64>().unwrap();

    let result = match operator.as_str() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                eprintln!("Error: Division by zero");
                process::exit(1);
            }
            num1 / num2
        }
        _ => {
            eprintln!("Error: Invalid operator");
            process::exit(1);
        }
    };

    println!("{}", result);
}
