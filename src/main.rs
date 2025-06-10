use clap::Parser;

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

    println!("Calculator started with arguments: {:?}", args);

    // Implement calculation logic here
}
