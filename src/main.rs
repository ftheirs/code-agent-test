use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Your Name", version = "0.1.0", about = "A simple calculator", long_about = None)]
struct Args {
    /// The first number
    #[arg(short = 'a', long)]
    num1: f64,

    /// The second number
    #[arg(short = 'b', long)]
    num2: f64,

    /// The operator (+, -, *, /)
    #[arg(short = 'o', long)]
    operator: String,
}

fn calculate(num1: f64, num2: f64, operator: &str) -> Result<f64, String> {
    match operator {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                Err("Division by zero!".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err("Invalid operator!".to_string()),
    }
}

fn main() {
    let args = Args::parse();

    let result = calculate(args.num1, args.num2, &args.operator);

    match result {
        Ok(res) => println!("Result: {}", res),
        Err(err) => eprintln!("Error: {}", err),
    }
}
