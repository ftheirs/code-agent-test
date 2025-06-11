use std::env;
use std::process;

// Define custom error types
#[derive(Debug, PartialEq)]
enum CalculatorError {
    InvalidArgumentCount,
    InvalidNumberFormat(String),
    UnsupportedOperator(String),
    DivisionByZero,
}

impl std::fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalculatorError::InvalidArgumentCount => {
                write!(f, "Error: Invalid number of arguments. Usage: <number1> <operator> <number2>")
            }
            CalculatorError::InvalidNumberFormat(s) => {
                write!(f, "Error: Invalid number format: {}", s)
            }
            CalculatorError::UnsupportedOperator(s) => {
                write!(f, "Error: Unsupported operator: {}", s)
            }
            CalculatorError::DivisionByZero => {
                write!(f, "Error: Division by zero is not allowed.")
            }
        }
    }
}

// Arithmetic functions
fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> Result<f64, CalculatorError> {
    if b == 0.0 {
        Err(CalculatorError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("{}", CalculatorError::InvalidArgumentCount);
        process::exit(1);
    }

    let num1_str = &args[1];
    let operator = &args[2];
    let num2_str = &args[3];

    let num1: f64 = match num1_str.parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("{}", CalculatorError::InvalidNumberFormat(num1_str.clone()));
            process::exit(1);
        }
    };

    let num2: f64 = match num2_str.parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("{}", CalculatorError::InvalidNumberFormat(num2_str.clone()));
            process::exit(1);
        }
    };

    let result = match operator.as_str() {
        "+" => Ok(add(num1, num2)),
        "-" => Ok(subtract(num1, num2)),
        "*" => Ok(multiply(num1, num2)),
        "/" => divide(num1, num2),
        _ => Err(CalculatorError::UnsupportedOperator(operator.clone())),
    };

    match result {
        Ok(val) => println!("{}", val),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2.0, 3.0), 5.0);
        assert_eq!(add(-1.0, 1.0), 0.0);
        assert_eq!(add(0.0, 0.0), 0.0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5.0, 2.0), 3.0);
        assert_eq!(subtract(2.0, 5.0), -3.0);
        assert_eq!(subtract(0.0, 0.0), 0.0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2.0, 3.0), 6.0);
        assert_eq!(multiply(-2.0, 3.0), -6.0);
        assert_eq!(multiply(0.0, 5.0), 0.0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(6.0, 3.0).unwrap(), 2.0);
        assert_eq!(divide(-6.0, 3.0).unwrap(), -2.0);
        assert_eq!(divide(0.0, 5.0).unwrap(), 0.0);
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(divide(10.0, 0.0), Err(CalculatorError::DivisionByZero));
    }
}
