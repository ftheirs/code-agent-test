fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <number1> <operator> <number2>", args[0]);
        std::process::exit(1);
    }

    let num1_str = &args[1];
    let operator = &args[2];
    let num2_str = &args[3];

    let num1: f64 = match num1_str.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Invalid number provided: {}", num1_str);
            std::process::exit(1);
        }
    };

    let num2: f64 = match num2_str.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Invalid number provided: {}", num2_str);
            std::process::exit(1);
        }
    };

    let result = match operator.as_str() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                eprintln!("Error: Division by zero");
                std::process::exit(1);
            } else {
                num1 / num2
            }
        }
        _ => {
            eprintln!("Error: Invalid operator: {}", operator);
            std::process::exit(1);
        }
    };

    println!("{}", result);
}
