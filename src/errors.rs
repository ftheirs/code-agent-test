se std::fmt;

#[derive(Debug, PartialEq)]
pub enum CalcError {
    DivisionByZero,
    UnsupportedOperator(char),
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalcError::DivisionByZero => write!(f, "Division by zero is not allowed"),
            CalcError::UnsupportedOperator(op) => write!(f, "Unsupported operator: '{}'", op),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_division_by_zero() {
        assert_eq!(format!("{}", CalcError::DivisionByZero), "Division by zero is not allowed");
    }

    #[test]
    fn test_display_unsupported_operator() {
        assert_eq!(format!("{}", CalcError::UnsupportedOperator('^')), "Unsupported operator: '^'");
    }
}
