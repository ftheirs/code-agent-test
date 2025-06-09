use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[clap(author = "Your Name", version = "0.1.0", about = "A simple calculator CLI")]
struct Args {
    #[clap(short, long, value_parser)]
    num1: f64,

    #[clap(short, long, value_parser)]
    num2: f64,

    #[clap(short, long, value_enum)]
    operator: Operator,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn calculate(num1: f64, num2: f64, operator: Operator) -> Result<f64, String> {
    match operator {
        Operator::Add => Ok(num1 + num2),
        Operator::Subtract => Ok(num1 - num2),
        Operator::Multiply => Ok(num1 * num2),
        Operator::Divide => {
            if num2 == 0.0 {
                Err("Error: Division by zero".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
    }
}

fn main() {
    let args = Args::parse();

    match calculate(args.num1, args.num2, args.operator) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => eprintln!("{}", err),
    }
}
