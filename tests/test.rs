#[cfg(test)]
mod tests {
    use super::*;
    use rust_cli_calculator::calculator;

    #[test]
    fn test_add() {
        assert_eq!(calculator::add(2.0, 3.0), 5.0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(calculator::subtract(5.0, 3.0), 2.0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(calculator::multiply(2.0, 3.0), 6.0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(calculator::divide(6.0, 3.0), Ok(2.0));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(calculator::divide(6.0, 0.0), Err("Division by zero".to_string()));
    }
}
