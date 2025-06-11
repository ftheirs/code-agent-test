n main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        eprintln!("Error: Incorrect number of arguments. Usage: <number1> <operator> <number2>");
        return;
    }

    let num1_str = &args[1];
    let operator = &args[2];
    let num2_str = &args[3];

    let num1: f64 = match num1_str.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Invalid number format for first argument.");
            return;
        }
    };

    let num2: f64 = match num2_str.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Invalid number format for third argument.");
            return;
        }
    };

    let result = match operator.as_str() {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                Err("Error: Division by zero.")
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err("Error: Invalid operator. Supported operators are +, -, *, /."),
    };

    match result {
        Ok(val) => println!("{}", val),
        Err(err) => eprintln!("{}", err),
    }
}
