pub fn calculate(num1: f64, operator: &str, num2: f64) -> Result<f64, String> {
    match operator {
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
        _ => Err(format!("Invalid operator: {}", operator)),
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
        assert_eq!(calculate(5.0, "-", 3.0).unwrap(), 2.0);
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(calculate(2.0, "*", 3.0).unwrap(), 6.0);
    }

    #[test]
    fn test_division() {
        assert_eq!(calculate(6.0, "/", 3.0).unwrap(), 2.0);
    }

    #[test]
    fn test_division_by_zero() {
        assert_eq!(calculate(5.0, "/", 0.0).is_err(), true);
    }

    #[test]
    fn test_invalid_operator() {
        assert_eq!(calculate(5.0, "%", 0.0).is_err(), true);
    }
}
