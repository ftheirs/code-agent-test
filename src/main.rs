se clap::Parser;
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about = "A simple command-line calculator", long_about = None)]
struct Args {
    /// The first number
    num1: f64,

    /// The operator (+, -, *, /)
    operator: String,

    /// The second number
    num2: f64,
}

fn main() {
    let args = Args::parse();

    let result = match args.operator.as_str() {
        "+" => Some(args.num1 + args.num2),
        "-" => Some(args.num1 - args.num2),
        "*" => Some(args.num1 * args.num2),
        "/" => {
            if args.num2 == 0.0 {
                eprintln!("Error: Division by zero is not allowed.");
                process::exit(1);
            }
            Some(args.num1 / args.num2)
        },
        _ => {
            eprintln!("Error: Invalid operator. Supported operators are +, -, *, /.");
            process::exit(1);
        }
    };

    if let Some(res) = result {
        println!("{}", res);
    }
}
