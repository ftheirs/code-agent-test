#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_addition() {
        let output = Command::new("cargo")
            .args(["run", "--", "2", "+", "3"])
            .output()
            .expect("Failed to execute command");
        assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "5");
    }

    #[test]
    fn test_subtraction() {
        let output = Command::new("cargo")
            .args(["run", "--", "5", "-", "2"])
            .output()
            .expect("Failed to execute command");
        assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "3");
    }

    #[test]
    fn test_multiplication() {
        let output = Command::new("cargo")
            .args(["run", "--", "4", "*", "6"])
            .output()
            .expect("Failed to execute command");
        assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "24");
    }

    #[test]
    fn test_division() {
        let output = Command::new("cargo")
            .args(["run", "--", "8", "/", "2"])
            .output()
            .expect("Failed to execute command");
        assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "4");
    }

    #[test]
    fn test_division_by_zero() {
        let output = Command::new("cargo")
            .args(["run", "--", "5", "/", "0"])
            .output()
            .expect("Failed to execute command");
        assert_eq!(String::from_utf8_lossy(&output.stderr).trim(), "Error: Division by zero");
    }

    #[test]
    fn test_invalid_input() {
        let output = Command::new("cargo")
            .args(["run", "--", "a", "+", "2"])
            .output()
            .expect("Failed to execute command");
        assert_eq!(String::from_utf8_lossy(&output.stderr).trim(), "Invalid number1: a");
    }

    #[test]
    fn test_invalid_operator() {
        let output = Command::new("cargo")
            .args(["run", "--", "2", "%", "2"])
            .output()
            .expect("Failed to execute command");
        assert_eq!(String::from_utf8_lossy(&output.stderr).trim(), "Invalid operator: %");
    }

    #[test]
    fn test_incorrect_number_of_arguments() {
        let output = Command::new("cargo")
            .args(["run", "--", "2", "+"])
            .output()
            .expect("Failed to execute command");
        assert_eq!(String::from_utf8_lossy(&output.stderr).trim(), "Usage: calculator <number1> <operator> <number2>");
    }
}
