#[test]
fn test_addition() {
    assert_eq!(calculate(1.0, "+", 2.0), 3.0);
}

#[test]
fn test_subtraction() {
    assert_eq!(calculate(5.0, "-", 2.0), 3.0);
}

#[test]
fn test_multiplication() {
    assert_eq!(calculate(3.0, "*", 2.0), 6.0);
}

#[test]
fn test_division() {
    assert_eq!(calculate(6.0, "/", 2.0), 3.0);
}

#[test]
#[should_panic]
fn test_invalid_operator() {
    calculate(1.0, "%", 2.0);
}

#[test]
#[should_panic]
fn test_division_by_zero() {
    calculate(1.0, "/", 0.0);
}

use calculator::calculate;
