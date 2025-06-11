mport pytest
import subprocess
import sys

# Assuming the executable is located at target/debug/calculator relative to the project root
# Adjust the executable path if necessary based on the actual build location
if sys.platform == "win32":
    CALCULATOR_PATH = "../target/debug/calculator.exe"
else:
    CALCULATOR_PATH = "../target/debug/calculator"

def run_calculator(args):
    """Helper function to run the calculator executable."""
    result = subprocess.run(
        [CALCULATOR_PATH] + args,
        capture_output=True,
        text=True
    )
    return result

def test_addition_integers():
    result = run_calculator(["10", "+", "5"])
    assert result.returncode == 0
    assert result.stdout.strip() == "Result: 15"
    assert result.stderr == ""

def test_subtraction_integers():
    result = run_calculator(["10", "-", "5"])
    assert result.returncode == 0
    assert result.stdout.strip() == "Result: 5"
    assert result.stderr == ""

def test_multiplication_integers():
    result = run_calculator(["10", "*", "5"])
    assert result.returncode == 0
    assert result.stdout.strip() == "Result: 50"
    assert result.stderr == ""

def test_division_integers():
    result = run_calculator(["10", "/", "5"])
    assert result.returncode == 0
    assert result.stdout.strip() == "Result: 2"
    assert result.stderr == ""

def test_addition_floats():
    result = run_calculator(["10.5", "+", "5.2"])
    assert result.returncode == 0
    # Using startswith to handle potential floating point precision differences
    assert result.stdout.strip().startswith("Result: 15.7")
    assert result.stderr == ""

def test_subtraction_floats():
    result = run_calculator(["10.5", "-", "5.2"])
    assert result.returncode == 0
    assert result.stdout.strip().startswith("Result: 5.3")
    assert result.stderr == ""

def test_multiplication_floats():
    result = run_calculator(["10.5", "*", "5.2"])
    assert result.returncode == 0
    assert result.stdout.strip().startswith("Result: 54.6")
    assert result.stderr == ""

def test_division_floats():
    result = run_calculator(["10.5", "/", "5.2"])
    assert result.returncode == 0
    assert result.stdout.strip().startswith("Result: 2.019") # Check for a few decimal places
    assert result.stderr == ""

def test_mixed_types_addition():
    result = run_calculator(["10", "+", "5.5"])
    assert result.returncode == 0
    assert result.stdout.strip().startswith("Result: 15.5")
    assert result.stderr == ""

def test_mixed_types_multiplication():
    result = run_calculator(["10.5", "*", "5"])
    assert result.returncode == 0
    assert result.stdout.strip().startswith("Result: 52.5")
    assert result.stderr == ""

def test_division_by_zero():
    result = run_calculator(["10", "/", "0"])
    assert result.returncode != 0 # Expect a non-zero exit code
    assert "Error" in result.stderr # Expect an error message on stderr
    assert result.stdout == ""

def test_invalid_operator():
    result = run_calculator(["10", "%", "5"])
    assert result.returncode != 0 # Expect a non-zero exit code
    assert "Error" in result.stderr # Expect an error message on stderr
    assert result.stdout == ""

def test_not_enough_arguments():
    result = run_calculator(["10", "+"])
    assert result.returncode != 0 # Expect a non-zero exit code
    assert "Error" in result.stderr # Expect an error message on stderr
    assert result.stdout == ""

def test_too_many_arguments():
    result = run_calculator(["10", "+", "5", "extra"])
    assert result.returncode != 0 # Expect a non-zero exit code
    assert "Error" in result.stderr # Expect an error message on stderr
    assert result.stdout == ""

def test_invalid_number_format():
    result = run_calculator(["abc", "+", "5"])
    assert result.returncode != 0 # Expect a non-zero exit code
    assert "Error" in result.stderr # Expect an error message on stderr
    assert result.stdout == ""

def test_invalid_second_number_format():
    result = run_calculator(["10", "+", "def"])
    assert result.returncode != 0 # Expect a non-zero exit code
    assert "Error" in result.stderr # Expect an error message on stderr
    assert result.stdout == ""
