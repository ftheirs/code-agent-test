mod calculator;
mod input_parser;
mod output_printer;

use calculator::{parse_operator};
use input_parser::parse_input;
use output_printer::print_output;

fn main() {
    let input = parse_input();

    match input {
        Ok(input_data) => {
            let operation = parse_operator(&input_data.operation);
            match operation {
                Ok(op) => {
                    let result = op.calculate(input_data.x, input_data.y);
                    print_output(result);
                }
                Err(err) => print_output(Err(err)),
            }
        }
        Err(err) => print_output(Err(err)),
    }
}
