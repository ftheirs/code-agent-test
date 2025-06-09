use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Your Name", version = "0.1", about = "A simple calculator", long_about = None)]
struct Args {
    /// First number
    #[arg(name = "num1")]
    num1: f64,

    /// Second number
    #[arg(name = "num2")]
    num2: f64,

    /// Operator (+, -, *, /)
    #[arg(name = "operator")]
    operator: String,
}

fn calculate(num1: f64, num2: f64, operator: &str) -> Result<f64, String> {
    match operator {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                Err("Error: Division by zero".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err("Error: Invalid operator".to_string()),
    }
}

fn main() {
    let args = Args::parse();

    let result = calculate(args.num1, args.num2, &args.operator);

    match result {
        Ok(value) => println!("{}", value),
        Err(err) => eprintln!("{}", err),
    }
}
