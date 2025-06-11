se clap::Parser;
use thiserror::Error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "The first number")]
    num1: f64,
    #[arg(help = "The operator (+, -, *, /)")]
    operator: String,
    #[arg(help = "The second number")]
    num2: f64,
}

#[derive(Error, Debug)]
enum CalculatorError {
    #[error("Invalid operator: {0}. Supported operators are +, -, *, /")]
    InvalidOperator(String),
    #[error("Division by zero is not allowed")]
    DivisionByZero,
}

fn calculate(num1: f64, operator: &str, num2: f64) -> Result<f64, CalculatorError> {
    match operator {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                Err(CalculatorError::DivisionByZero)
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err(CalculatorError::InvalidOperator(operator.to_string())),
    }
}

fn main() {
    let args = Args::parse();

    match calculate(args.num1, &args.operator, args.num2) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
