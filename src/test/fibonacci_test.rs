#[test]
fn test_fibonacci_sequence() {
    let mut fibonacci = fibonacci::Fibonacci::new();
    assert_eq!(fibonacci.next(), Some(1));
    assert_eq!(fibonacci.next(), Some(1));
    assert_eq!(fibonacci.next(), Some(2));
    assert_eq!(fibonacci.next(), Some(3));
    assert_eq!(fibonacci.next(), Some(5));
    assert_eq!(fibonacci.next(), Some(8));
}

#[test]
fn test_fibonacci_limit() {
    let mut fibonacci = fibonacci::Fibonacci::new();
    let mut last = 0;
    while let Some(x) = fibonacci.next() {
        last = x;
    }
    assert_eq!(last, 286577670);
}
