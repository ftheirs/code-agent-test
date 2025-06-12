se std::env;

enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "+" => Some(Operator::Add),
            "-" => Some(Operator::Subtract),
            "*" => Some(Operator::Multiply),
            "/" => Some(Operator::Divide),
            _ => None,
        }
    }
}

fn calculate(num1: f64, num2: f64, operator: Operator) -> Result<f64, String> {
    match operator {
        Operator::Add => Ok(num1 + num2),
        Operator::Subtract => Ok(num1 - num2),
        Operator::Multiply => Ok(num1 * num2),
        Operator::Divide => {
            if num2 == 0.0 {
                Err("Error: Division by zero".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <number1> <operator> <number2>", args[0]);
        eprintln!("Operators: +, -, *, /");
        std::process::exit(1);
    }

    let num1_str = &args[1];
    let operator_str = &args[2];
    let num2_str = &args[3];

    let num1: f64 = match num1_str.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Invalid number format for '{}'", num1_str);
            std::process::exit(1);
        }
    };

    let operator = match Operator::from_str(operator_str) {
        Some(op) => op,
        None => {
            eprintln!("Error: Invalid operator '{}'", operator_str);
            std::process::exit(1);
        }
    };

    let num2: f64 = match num2_str.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Invalid number format for '{}'", num2_str);
            std::process::exit(1);
        }
    };

    match calculate(num1, num2, operator) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(calculate(5.0, 3.0, Operator::Add).unwrap(), 8.0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(calculate(5.0, 3.0, Operator::Subtract).unwrap(), 2.0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(calculate(5.0, 3.0, Operator::Multiply).unwrap(), 15.0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(calculate(6.0, 3.0, Operator::Divide).unwrap(), 2.0);
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(calculate(5.0, 0.0, Operator::Divide).unwrap_err(), "Error: Division by zero".to_string());
    }

    #[test]
    fn test_operator_from_str() {
        assert!(matches!(Operator::from_str("+").unwrap(), Operator::Add));
        assert!(matches!(Operator::from_str("-").unwrap(), Operator::Subtract));
        assert!(matches!(Operator::from_str("*").unwrap(), Operator::Multiply));
        assert!(matches!(Operator::from_str("/").unwrap(), Operator::Divide));
        assert!(Operator::from_str("x").is_none());
    }
}
