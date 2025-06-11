se std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <number1> <operator> <number2>", args[0]);
        eprintln!("Supported operators: +, -, *, /");
        process::exit(1);
    }

    let num1_str = &args[1];
    let operator = &args[2];
    let num2_str = &args[3];

    let num1: f64 = match num1_str.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Invalid number provided for number1: {}", num1_str);
            process::exit(1);
        }
    };

    let num2: f64 = match num2_str.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Invalid number provided for number2: {}", num2_str);
            process::exit(1);
        }
    };

    let result = match operator.as_str() {
        "+" => add(num1, num2),
        "-" => subtract(num1, num2),
        "*" => multiply(num1, num2),
        "/" => divide(num1, num2),
        _ => {
            eprintln!("Error: Unsupported operator: {}", operator);
            eprintln!("Supported operators: +, -, *, /");
            process::exit(1);
        }
    };

    match result {
        Ok(val) => println!("{}", val),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn add(a: f64, b: f64) -> Result<f64, String> {
    Ok(a + b)
}

fn subtract(a: f64, b: f64) -> Result<f64, String> {
    Ok(a - b)
}

fn multiply(a: f64, b: f64) -> Result<f64, String> {
    Ok(a * b)
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero is not allowed.".to_string())
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2.0, 3.0).unwrap(), 5.0);
        assert_eq!(add(-1.0, 1.0).unwrap(), 0.0);
        assert_eq!(add(0.0, 0.0).unwrap(), 0.0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5.0, 2.0).unwrap(), 3.0);
        assert_eq!(subtract(2.0, 5.0).unwrap(), -3.0);
        assert_eq!(subtract(0.0, 0.0).unwrap(), 0.0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2.0, 3.0).unwrap(), 6.0);
        assert_eq!(multiply(-2.0, 3.0).unwrap(), -6.0);
        assert_eq!(multiply(0.0, 5.0).unwrap(), 0.0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(6.0, 3.0).unwrap(), 2.0);
        assert_eq!(divide(5.0, 2.0).unwrap(), 2.5);
        assert!(divide(1.0, 0.0).is_err());
        assert!(divide(0.0, 0.0).is_err());
    }
}
