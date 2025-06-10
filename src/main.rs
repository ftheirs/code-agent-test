use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: calculator <number1> <operator> <number2>");
        return;
    }

    let num1 = match args[1].parse::<f64>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid number1: {}", args[1]);
            return;
        }
    };

    let operator = &args[2];

    let num2 = match args[3].parse::<f64>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid number2: {}", args[3]);
            return;
        }
    };

    let result = match operator.as_str() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                eprintln!("Error: Division by zero");
                return;
            }
            num1 / num2
        }
        _ => {
            eprintln!("Invalid operator: {}", operator);
            return;
        }
    };

    println!("{}", result);
}
