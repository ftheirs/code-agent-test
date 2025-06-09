use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_add() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("calculator")?;
    cmd.arg("2").arg("+").arg("3");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Result: 5"));

    Ok(())
}

#[test]
fn test_subtract() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("calculator")?;
    cmd.arg("5").arg("-").arg("2");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Result: 3"));

    Ok(())
}

#[test]
fn test_multiply() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("calculator")?;
    cmd.arg("4").arg("*").arg("6");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Result: 24"));

    Ok(())
}

#[test]
fn test_divide() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("calculator")?;
    cmd.arg("10").arg("/").arg("2");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Result: 5"));

    Ok(())
}

#[test]
fn test_divide_by_zero() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("calculator")?;
    cmd.arg("10").arg("/").arg("0");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: Division by zero"));

    Ok(())
}

#[test]
fn test_invalid_operator() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("calculator")?;
    cmd.arg("5").arg("%").arg("2");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: Invalid operator"));

    Ok(())
}

#[test]
fn test_invalid_number() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("calculator")?;
    cmd.arg("abc").arg("+").arg("2");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: Invalid first number"));

    Ok(())
}
