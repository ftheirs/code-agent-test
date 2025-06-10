import subprocess
import pytest


def test_addition():
    result = subprocess.run(['cargo', 'run', '1', '+', '2'], capture_output=True, text=True)
    assert result.returncode == 0
    assert result.stdout.strip() == '3.0'


def test_subtraction():
    result = subprocess.run(['cargo', 'run', '5', '-', '3'], capture_output=True, text=True)
    assert result.returncode == 0
    assert result.stdout.strip() == '2.0'


def test_multiplication():
    result = subprocess.run(['cargo', 'run', '4', '*', '6'], capture_output=True, text=True)
    assert result.returncode == 0
    assert result.stdout.strip() == '24.0'


def test_division():
    result = subprocess.run(['cargo', 'run', '10', '/', '2'], capture_output=True, text=True)
    assert result.returncode == 0
    assert result.stdout.strip() == '5.0'


def test_division_by_zero():
    result = subprocess.run(['cargo', 'run', '5', '/', '0'], capture_output=True, text=True)
    assert result.returncode != 0
    assert 'Error: Division by zero' in result.stderr


def test_invalid_operator():
    result = subprocess.run(['cargo', 'run', '2', '$', '3'], capture_output=True, text=True)
    assert result.returncode != 0
    assert 'Error: Invalid operator' in result.stderr


def test_invalid_number_input():
    result = subprocess.run(['cargo', 'run', 'a', '+', '2'], capture_output=True, text=True)
    assert result.returncode != 0
    assert 'Error: Invalid number input' in result.stderr


def test_missing_arguments():
    result = subprocess.run(['cargo', 'run', '1', '+'], capture_output=True, text=True)
    assert result.returncode != 0
    assert 'Error: Not enough arguments' in result.stderr
