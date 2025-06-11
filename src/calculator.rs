se crate::errors::CalcError;

pub fn calculate(num1: f64, operator: char, num2: f64) -> Result<f64, CalcError> {
    match operator {
        '+' => Ok(num1 + num2),
        '-' => Ok(num1 - num2),
        '*' => Ok(num1 * num2),
        '/' => {
            if num2 == 0.0 {
                Err(CalcError::DivisionByZero)
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err(CalcError::UnsupportedOperator(operator)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::CalcError;

    #[test]
    fn test_addition() {
        assert_eq!(calculate(5.0, '+', 3.0).unwrap(), 8.0);
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(calculate(10.0, '-', 4.0).unwrap(), 6.0);
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(calculate(7.0, '*', 2.0).unwrap(), 14.0);
    }

    #[test]
    fn test_division() {
        assert_eq!(calculate(9.0, '/', 3.0).unwrap(), 3.0);
    }

    #[test]
    fn test_division_by_zero() {
        match calculate(5.0, '/', 0.0) {
            Err(CalcError::DivisionByZero) => assert!(true),
            _ => panic!("Expected DivisionByZero error"),
        }
    }

    #[test]
    fn test_unsupported_operator() {
        match calculate(5.0, '%', 2.0) {
            Err(CalcError::UnsupportedOperator(op)) => assert_eq!(op, '%'),
            _ => panic!("Expected UnsupportedOperator error"),
        }
    }
}
