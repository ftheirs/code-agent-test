pub fn parse_arguments(args: Vec<String>) -> Result<(f64, char, f64), String> {
    if args.len() != 4 {
        return Err("Invalid number of arguments".to_string());
    }

    let num1 = match args[1].parse::<f64>() {
        Ok(n) => n,
        Err(_) => return Err("Invalid number format for number1".to_string()),
    };

    let operator = match args[2].len() {
        1 => args[2].chars().next().unwrap(),
        _ => return Err("Invalid operator format".to_string()),
    };

    if !['+', '-', '*', '/'].contains(&operator) {
        return Err("Invalid operator".to_string());
    }

    let num2 = match args[3].parse::<f64>() {
        Ok(n) => n,
        Err(_) => return Err("Invalid number format for number2".to_string()),
    };

    Ok((num1, operator, num2))
}


#[cfg(test)]
mod tests {
    use super::parse_arguments;

    #[test]
    fn test_valid_arguments() {
        let args = vec!["calculator".to_string(), "2".to_string(), "+".to_string(), "3".to_string()];
        let result = parse_arguments(args).unwrap();
        assert_eq!(result, (2.0, '+', 3.0));
    }

    #[test]
    fn test_invalid_number_format() {
        let args = vec!["calculator".to_string(), "a".to_string(), "+".to_string(), "3".to_string()];
        let result = parse_arguments(args);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_operator() {
        let args = vec!["calculator".to_string(), "2".to_string(), "%".to_string(), "3".to_string()];
        let result = parse_arguments(args);
        assert!(result.is_err());
    }

    #[test]
    fn test_incorrect_argument_count() {
        let args = vec!["calculator".to_string(), "2".to_string(), "+".to_string()];
        let result = parse_arguments(args);
        assert!(result.is_err());
    }
}
