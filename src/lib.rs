mod tests {
    use super::super::calculate;

    #[test]
    fn test_addition() {
        assert_eq!(calculate(2.0, 3.0, "+").unwrap(), 5.0);
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(calculate(5.0, 3.0, "-").unwrap(), 2.0);
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(calculate(2.0, 3.0, "*").unwrap(), 6.0);
    }

    #[test]
    fn test_division() {
        assert_eq!(calculate(6.0, 3.0, "/").unwrap(), 2.0);
    }

    #[test]
    fn test_division_by_zero() {
        assert_eq!(calculate(6.0, 0.0, "/").is_err(), true);
    }

    #[test]
    fn test_invalid_operator() {
        assert_eq!(calculate(2.0, 3.0, "%/").is_err(), true);
    }
}
