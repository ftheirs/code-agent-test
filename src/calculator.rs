pub fn calculate(first_number: f64, operator: &str, second_number: f64) -> Result<f64, String> {
    match operator {
        "+" => Ok(first_number + second_number),
        "-" => Ok(first_number - second_number),
        "*" => Ok(first_number * second_number),
        "/" => {
            if second_number == 0.0 {
                Err("Error: Division by zero.".to_string())
            } else {
                Ok(first_number / second_number)
            }
        }
        _ => Err("Error: Invalid input.".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::calculate;

    #[test]
    fn test_addition() {
        assert_eq!(calculate(2.0, "+", 2.0).unwrap(), 4.0);
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(calculate(5.0, "-", 3.0).unwrap(), 2.0);
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
        assert_eq!(calculate(10.0, "/", 0.0).unwrap_err(), "Error: Division by zero.");
    }

    #[test]
    fn test_invalid_operator() {
        assert_eq!(calculate(5.0, "%", 3.0).unwrap_err(), "Error: Invalid input.");
    }
}
