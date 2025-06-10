use std::env;

pub fn parse_arguments() -> Result<(f64, f64, String), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        return Err("Incorrect number of arguments. Usage: calculator <num1> <operator> <num2>".to_string());
    }

    let num1 = match args[1].parse::<f64>() {
        Ok(n) => n,
        Err(_) => return Err("Invalid number: {}".to_string()),
    };

    let operator = args[2].clone();

    let num2 = match args[3].parse::<f64>() {
        Ok(n) => n,
        Err(_) => return Err("Invalid number: {}".to_string()),
    };

    Ok((num1, num2, operator))
}
