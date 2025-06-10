use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Your Name", version = "1.0", about = "A simple command-line calculator", long_about = None)]
struct Args {
    /// The first number
    num1: f64,

    /// The operator (+, -, *, /)
    op: String,

    /// The second number
    num2: f64,
}

fn calculate(num1: f64, op: &str, num2: f64) -> Result<f64, String> {
    match op {
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
        _ => Err("Invalid operator: Please use +, -, *, or /".to_string()),
    }
}

fn main() {
    let args = Args::parse();

    let result = calculate(args.num1, &args.op, args.num2);

    match result {
        Ok(res) => println!("{}", res),
        Err(err) => {
            eprintln!("Invalid input: Please provide two numbers and a valid operator (+, -, *, /)");
            std::process::exit(1);
        }
    }
}
