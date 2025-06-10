use fibonacci_iterator::Fibonacci;

#[test]
fn test_fibonacci_initial_values() {
    let mut fib = Fibonacci::new();
    assert_eq!(fib.next(), Some(0));
    assert_eq!(fib.next(), Some(1));
}

#[test]
fn test_fibonacci_sequence_progression() {
    let mut fib = Fibonacci::new();
    assert_eq!(fib.next(), Some(0));
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(2));
    assert_eq!(fib.next(), Some(3));
    assert_eq!(fib.next(), Some(5));
    assert_eq!(fib.next(), Some(8));
}

#[test]
fn test_fibonacci_overflow_or_limit() {
    // This test assumes a u64 implementation and aims to reach an overflow.
    // The exact number of iterations will depend on the `Fibonacci` struct's
    // internal type (e.g., u64) and its overflow handling.
    // We will iterate until `None` is returned, signifying overflow or limit.
    let mut fib = Fibonacci::new();
    let mut last_value = 0;
    let mut current_value = 0;
    let mut count = 0;

    // Iterate until None is returned, or a reasonable upper bound for testing
    // to prevent infinite loops in case of incorrect implementation.
    // For u64, the sequence will overflow after about 93 iterations.
    for i in 0..100 { // Max 100 iterations to prevent infinite loop for faulty implementation
        match fib.next() {
            Some(val) => {
                // Assert that the sequence is increasing (after the first two terms)
                if i >= 2 {
                    assert!(val > current_value, "Fibonacci sequence should be increasing");
                }
                last_value = current_value;
                current_value = val;
            },
            None => {
                // If None is returned, it means overflow or limit was reached.
                // This is the expected behavior for this test.
                println!("Fibonacci sequence ended after {} iterations due to overflow/limit.", count);
                return; // Test passed, overflow/limit correctly handled.
            }
        }
        count += 1;
    }

    // If the loop finishes without returning None, it means the overflow/limit
    // condition was not met within 100 iterations, which is unexpected for u64.
    panic!("Fibonacci iterator did not return None within 100 iterations, suggesting no overflow/limit handling or unexpected large type.");
}
