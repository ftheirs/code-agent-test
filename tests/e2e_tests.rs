se std::process::Command;

#[test]
fn test_addition() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("10")
        .arg("+")
        .arg("5")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "15");
}

#[test]
fn test_subtraction() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("10")
        .arg("-")
        .arg("5")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "5");
}

#[test]
fn test_multiplication() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("10")
        .arg("*")
        .arg("5")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "50");
}

#[test]
fn test_division() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("10")
        .arg("/")
        .arg("5")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "2");
}

#[test]
fn test_division_by_zero() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("10")
        .arg("/")
        .arg("0")
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success()); // Expecting a non-zero exit code for error
    assert!(String::from_utf8_lossy(&output.stderr).contains("Error: Division by zero is not allowed."));
}

#[test]
fn test_invalid_number_of_arguments() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("10")
        .arg("+")
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("Usage: <number1> <operator> <number2>"));
}

#[test]
fn test_non_numeric_argument() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("abc")
        .arg("+")
        .arg("5")
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("Error: Invalid number provided. Please ensure both numbers are valid."));
}

#[test]
fn test_unsupported_operator() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("10")
        .arg("%")
        .arg("5")
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("Error: Unsupported operator. Supported operators are +, -, *, /."));
}
