#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(calculate(2.0, 3.0, "+").unwrap(), 5.0);
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(calculate(5.0, 2.0, "-").unwrap(), 3.0);
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(calculate(4.0, 2.0, "*").unwrap(), 8.0);
    }

    #[test]
    fn test_division() {
        assert_eq!(calculate(10.0, 2.0, "/").unwrap(), 5.0);
    }

    #[test]
    fn test_division_by_zero() {
        assert_eq!(calculate(5.0, 0.0, "/").unwrap_err(), "Error: Division by zero");
    }

    #[test]
    fn test_invalid_operator() {
        assert_eq!(calculate(2.0, 3.0, "%").unwrap_err(), "Error: Invalid operator");
    }
}

fn calculate(num1: f64, num2: f64, operator: &str) -> Result<f64, String> {
    match operator {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                Err("Error: Division by zero".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err("Error: Invalid operator".to_string()),
    }
}
