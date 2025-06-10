#[cfg(test)]
mod tests {
    use calculator::{{parse_operator, Operation}};

    #[test]
    fn test_add() {
        let op = parse_operator("+").unwrap();
        assert_eq!(op.calculate(1.0, 2.0).unwrap(), 3.0);
    }

    #[test]
    fn test_subtract() {
        let op = parse_operator("-").unwrap();
        assert_eq!(op.calculate(5.0, 2.0).unwrap(), 3.0);
    }

    #[test]
    fn test_multiply() {
        let op = parse_operator("*").unwrap();
        assert_eq!(op.calculate(3.0, 2.0).unwrap(), 6.0);
    }

    #[test]
    fn test_divide() {
        let op = parse_operator("/").unwrap();
        assert_eq!(op.calculate(6.0, 2.0).unwrap(), 3.0);
    }

    #[test]
    fn test_divide_by_zero() {
        let op = parse_operator("/").unwrap();
        assert_eq!(op.calculate(6.0, 0.0).unwrap_err(), "error: division by zero");
    }

    #[test]
    fn test_invalid_operator() {
        let op = parse_operator("%");
        assert_eq!(op.unwrap_err(), "error: invalid operator");
    }
}
