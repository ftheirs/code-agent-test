use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn add() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("calculator")?;

    cmd.arg("2").arg("+").arg("3");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("5"));

    Ok(())
}

#[test]
fn subtract() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("calculator")?;

    cmd.arg("5").arg("-").arg("2");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("3"));

    Ok(())
}

#[test]
fn multiply() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("calculator")?;

    cmd.arg("4").arg("*").arg("6");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("24"));

    Ok(())
}

#[test]
fn divide() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("calculator")?;

    cmd.arg("10").arg("/").arg("2");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("5"));

    Ok(())
}

#[test]
fn divide_by_zero() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("calculator")?;

    cmd.arg("5").arg("/").arg("0");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: Division by zero"));

    Ok(())
}

#[test]
fn invalid_number() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("calculator")?;

    cmd.arg("a").arg("+").arg("3");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: Invalid first number"));

    Ok(())
}
