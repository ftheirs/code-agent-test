use clap::{App, Arg};

fn main() {
    let matches = App::new("calculator")
        .version("0.1.0")
        .author("Your Name")
        .about("A simple calculator CLI")
        .arg(
            Arg::new("num1")
                .help("The first number")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("operator")
                .help("The operator (+, -, *, /)")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("num2")
                .help("The second number")
                .required(true)
                .index(3),
        )
        .get_matches();

    let num1_str = matches.value_of("num1").unwrap();
    let operator = matches.value_of("operator").unwrap();
    let num2_str = matches.value_of("num2").unwrap();

    let num1: f64 = match num1_str.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Invalid number: {}", num1_str);
            return;
        }
    };

    let num2: f64 = match num2_str.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Invalid number: {}", num2_str);
            return;
        }
    };

    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                eprintln!("Error: Division by zero");
                return;
            } else {
                num1 / num2
            }
        }
        _ => {
            eprintln!("Error: Invalid operator: {}", operator);
            return;
        }
    };

    println!("{}", result);
}
