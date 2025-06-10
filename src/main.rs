use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: calculator <number1> <operator> <number2>");
        return;
    }

    let num1 = args[1].parse::<f64>().unwrap();
    let operator = &args[2];
    let num2 = args[3].parse::<f64>().unwrap();

    println!("{} {} {} = {}", num1, operator, num2, calculate(num1, operator, num2));
}

fn calculate(num1: f64, operator: &str, num2: f64) -> f64 {
    match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => panic!("Invalid operator"),
    }
}
