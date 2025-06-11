use std::env;
use std::process;

// Define custom error types
#[derive(Debug, PartialEq)]
enum CalculatorError {
    InvalidInput,
    DivisionByZero,
    MalformedArguments,
}

// Implement arithmetic functions
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

// Function to parse arguments and perform calculation
fn calculate() -> Result<f64, CalculatorError> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        return Err(CalculatorError::MalformedArguments);
    }

    let num1: f64 = args[1].parse().map_err(|_| CalculatorError::InvalidInput)?;
    let operator: &str = &args[2];
    let num2: f64 = args[3].parse().map_err(|_| CalculatorError::InvalidInput)?;

    match operator {
        "+" => Ok(add(num1, num2)),
        "-" => Ok(subtract(num1, num2)),
        "*" => Ok(multiply(num1, num2)),
        "/" => divide(num1, num2),
        _ => Err(CalculatorError::MalformedArguments),
    }
}

fn main() {
    match calculate() {
        Ok(result) => println!("{}", result),
        Err(CalculatorError::InvalidInput) => {
            eprintln!("Error: Invalid input. Please provide valid numbers.");
            process::exit(1);
        }
        Err(CalculatorError::DivisionByZero) => {
            eprintln!("Error: Division by zero is not allowed.");
            process::exit(1);
        }
        Err(CalculatorError::MalformedArguments) => {
            eprintln!("Usage: calculator <number1> <operator> <number2>");
            eprintln!("Supported operators: +, -, *, /");
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
        assert_eq!(add(-2.0, 3.0), 1.0);
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
        assert_eq!(divide(6.0, 3.0), Ok(2.0));
        assert_eq!(divide(5.0, 2.0), Ok(2.5));
        assert_eq!(divide(10.0, 0.0), Err(CalculatorError::DivisionByZero));
    }

    // Testing error conditions for calculate() requires refactoring to accept args as parameters
    // For now, we rely on integration tests for CLI argument validation
}
