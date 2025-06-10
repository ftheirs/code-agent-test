use std::env;
use std::process;

mod calculator;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: calculator <number1> <operator> <number2>");
        process::exit(1);
    }

    let num1 = args[1].parse::<f64>().unwrap_or_else(|_| {
        eprintln!("error: invalid number: `{}`", args[1]);
        process::exit(1);
    });

    let operator = &args[2];

    let num2 = args[3].parse::<f64>().unwrap_or_else(|_| {
        eprintln!("error: invalid number: `{}`", args[3]);
        process::exit(1);
    });

    let result = calculator::calculate(num1, operator, num2).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    println!("{}", result);
}
