use std::process::Command;

#[test]
fn test_fibonacci_cli_integration() {
    // Build the project
    let build_status = Command::new("cargo")
        .arg("build")
        .status()
        .expect("Failed to build project");
    assert!(build_status.success(), "Cargo build failed");

    // Run the compiled binary (assuming it's in the target/debug directory)
    let output = Command::new("target/debug/fibonacci_sequence") // Replace with your actual binary name
        .output()
        .expect("Failed to execute command");

    // Assert that the command executed successfully
    assert!(output.status.success(), "Command execution failed");

    // Optionally, assert on the output of the command
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Output: {}", stdout);
    // Add assertions based on expected output here
    // Example: assert!(stdout.contains("Fibonacci"));
}
