#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use calculator::parse_operator;
    use input_parser::parse_input;

    #[test]
    fn test_valid_input() {
        // Simulate command-line arguments
        std::env::set_var("x", "5");
        std::env::set_var("y", "3");
        std::env::set_var("operation", "+");

        let result = parse_input();
        assert!(result.is_ok());
        let input = result.unwrap();

        assert_eq!(input.x, 5.0);
        assert_eq!(input.y, 3.0);
        assert_eq!(input.operation, "+");

        // Clean up environment variables
        std::env::remove_var("x");
        std::env::remove_var("y");
        std::env::remove_var("operation");
    }

    #[test]
    fn test_invalid_number_input() {
        // Simulate command-line arguments
        std::env::set_var("x", "abc");
        std::env::set_var("y", "3");
        std::env::set_var("operation", "+");

        let result = parse_input();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "error: invalid number");

        // Clean up environment variables
        std::env::remove_var("x");
        std::env::remove_var("y");
        std::env::remove_var("operation");
    }

    // Add more tests for invalid input (e.g., missing arguments)
}
