se clap::Parser;
use std::process;

mod calculator;
mod errors;
mod parser;

use calculator::calculate;
use errors::CalcError;
use parser::Args;

fn main() {
    let args = Args::parse();

    let result = calculate(args.num1, args.operator, args.num2);

    match result {
        Ok(val) => println!("{}", val),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
