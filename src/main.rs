use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    num1: f64,
    num2: f64,
    op: String,
}

fn calculate(num1: f64, num2: f64, op: &str) -> Result<f64, String> {
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
        _ => Err("Invalid operator!".to_string()),
    }
}

fn main() {
    let args = Cli::from_args();

    match calculate(args.num1, args.num2, &args.op) {
        Ok(result) => println!("{}", result),
        Err(e) => println!("Error: {}", e),
    }
}
