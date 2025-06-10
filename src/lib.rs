[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2.0, 3.0), 5.0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5.0, 2.0), 3.0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2.0, 3.0), 6.0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(6.0, 2.0).unwrap(), 3.0);
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(divide(6.0, 0.0).is_err());
    }

    #[test]
    fn test_parse_arguments_valid() {
        let args = vec!["calculator".to_string(), "2.0".to_string(), "+".to_string(), "3.0".to_string()];
        let (num1, operator, num2) = parse_arguments(args).unwrap();
        assert_eq!(num1, 2.0);
        assert_eq!(operator, '+');
        assert_eq!(num2, 3.0);
    }

    #[test]
    fn test_parse_arguments_invalid_number() {
        let args = vec!["calculator".to_string(), "a".to_string(), "+".to_string(), "3.0".to_string()];
        assert!(parse_arguments(args).is_err());
    }

    #[test]
    fn test_parse_arguments_invalid_operator() {
        let args = vec!["calculator".to_string(), "2.0".to_string(), "%".to_string(), "3.0".to_string()];
        assert!(parse_arguments(args).is_err());
    }

    #[test]
    fn test_parse_arguments_missing_arguments() {
        let args = vec!["calculator".to_string(), "2.0".to_string(), "+".to_string()];
        assert!(parse_arguments(args).is_err());
    }
}

fn add(x: f64, y: f64) -> f64 {
    x + y
}

fn subtract(x: f64, y: f64) -> f64 {
    x - y
}

fn multiply(x: f64, y: f64) -> f64 {
    x * y
}

fn divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(x / y)
    }
}

fn parse_arguments(args: Vec<String>) -> Result<(f64, char, f64), String> {
    if args.len() != 4 {
        return Err("Incorrect number of arguments".to_string());
    }

    let num1 = match args[1].parse::<f64>() {
        Ok(n) => n,
        Err(_) => return Err("Invalid number1".to_string()),
    };

    let operator = args[2].chars().next().unwrap();
    if operator != '+' && operator != '-' && operator != '*' && operator != '/' {
        return Err("Invalid operator".to_string());
    }

    let num2 = match args[3].parse::<f64>() {
        Ok(n) => n,
        Err(_) => return Err("Invalid number2".to_string()),
    };

    Ok((num1, operator, num2))
}
