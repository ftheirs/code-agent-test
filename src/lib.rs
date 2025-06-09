#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(calculate(2.0, 3.0, Operator::Add).unwrap(), 5.0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(calculate(5.0, 2.0, Operator::Subtract).unwrap(), 3.0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(calculate(2.0, 3.0, Operator::Multiply).unwrap(), 6.0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(calculate(6.0, 2.0, Operator::Divide).unwrap(), 3.0);
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(calculate(6.0, 0.0, Operator::Divide).unwrap_err(), "Error: Division by zero");
    }

}

use clap::ValueEnum;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub fn calculate(num1: f64, num2: f64, operator: Operator) -> Result<f64, String> {
    match operator {
        Operator::Add => Ok(num1 + num2),
        Operator::Subtract => Ok(num1 - num2),
        Operator::Multiply => Ok(num1 * num2),
        Operator::Divide => {
            if num2 == 0.0 {
                Err("Error: Division by zero".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
    }
}
