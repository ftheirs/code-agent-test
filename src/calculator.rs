pub fn calculate(num1: f64, operator: char, num2: f64) -> Result<f64, String> {
    match operator {
        '+' => Ok(num1 + num2),
        '-' => Ok(num1 - num2),
        '*' => Ok(num1 * num2),
        '/' => {
            if num2 == 0.0 {
                Err("Division by zero".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err("Invalid operator".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::calculate;

    #[test]
    fn test_addition() {
        let result = calculate(2.0, '+', 3.0).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_subtraction() {
        let result = calculate(5.0, '-', 3.0).unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_multiplication() {
        let result = calculate(4.0, '*', 6.0).unwrap();
        assert_eq!(result, 24.0);
    }

    #[test]
    fn test_division() {
        let result = calculate(10.0, '/', 2.0).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_division_by_zero() {
        let result = calculate(10.0, '/', 0.0);
        assert!(result.is_err());
    }
}
