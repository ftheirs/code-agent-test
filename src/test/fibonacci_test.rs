#[cfg(test)]
mod tests {
    #[test]
    fn test_fibonacci_sequence() {
        // Test case 1: Generate the first 10 Fibonacci numbers
        let fibonacci: Vec<u64> = fibonacci::Fibonacci::new().take(10).collect();
        assert_eq!(fibonacci, vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55]);

        // Test case 2: Generate the first 5 Fibonacci numbers
        let fibonacci: Vec<u64> = fibonacci::Fibonacci::new().take(5).collect();
        assert_eq!(fibonacci, vec![1, 1, 2, 3, 5]);

        // Test case 3: Generate an empty Fibonacci sequence
        let fibonacci: Vec<u64> = fibonacci::Fibonacci::new().take(0).collect();
        assert_eq!(fibonacci, vec![]);
    }

    #[test]
    fn test_fibonacci_sequence_with_limit() {
        // Test case 4: Generate the first 10 Fibonacci numbers with a limit
        let fibonacci: Vec<u64> = fibonacci::Fibonacci::new().take(10).collect();
        assert_eq!(fibonacci, vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55]);

        // Test case 5: Generate the first 5 Fibonacci numbers with a limit
        let fibonacci: Vec<u64> = fibonacci::Fibonacci::new().take(5).collect();
        assert_eq!(fibonacci, vec![1, 1, 2, 3, 5]);
    }

    #[test]
    fn test_fibonacci_zero_limit() {
        // Test case 6: Generate an empty Fibonacci sequence with a zero limit
        let fibonacci: Vec<u64> = fibonacci::Fibonacci::new().take(0).collect();
        assert_eq!(fibonacci, vec![]);
    }
}
