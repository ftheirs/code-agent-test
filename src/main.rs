use clap::Parser;

/// A simple calculator CLI
#[derive(Parser, Debug)]
#[command(author = "Your Name", version = "0.1.0", about = "", long_about = None)]
struct Args {
    /// The first number
    num1: f64,

    /// The operator (+, -, *, /)
    op: String,

    /// The second number
    num2: f64,
}

fn calculate(num1: f64, op: &str, num2: f64) -> Result<f64, String> {
    match op {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                Err("Division by zero!".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err("Invalid operator!".to_string()),
    }
}

fn main() {
    let args = Args::parse();

    match calculate(args.num1, &args.op, args.num2) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => eprintln!("Error: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::calculate;

    #[test]
    fn test_addition() {
        assert_eq!(calculate(2.0, "+", 3.0).unwrap(), 5.0);
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(calculate(5.0, "-", 2.0).unwrap(), 3.0);
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(calculate(3.0, "*", 4.0).unwrap(), 12.0);
    }

    #[test]
    fn test_division() {
        assert_eq!(calculate(10.0, "/", 2.0).unwrap(), 5.0);
    }

    #[test]
    fn test_division_by_zero() {
        assert!(calculate(5.0, "/", 0.0).is_err());
    }

    #[test]
    fn test_invalid_operator() {
        assert!(calculate(5.0, "%", 2.0).is_err());
    }
}
