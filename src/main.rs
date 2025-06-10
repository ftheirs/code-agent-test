use clap::Parser;

mod calculator;

use calculator::calculate;

#[derive(Parser, Debug)]
#[command(author = "Your Name", version = "1.0", about = "A simple CLI calculator", long_about = None)]
struct Args {
    /// The first number
    #[arg(short, long)]
    num1: f64,

    /// The second number
    #[arg(short, long)]
    num2: f64,

    /// The operation to perform (+, -, *, /)
    #[arg(short, long)]
    operator: String,
}

fn main() {
    let args = Args::parse();

    match calculate(args.num1, args.num2, &args.operator) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
