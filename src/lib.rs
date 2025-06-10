struct Fibonacci {
    a: u32,
    b: u32,
    limit: Option<u32>,
    count: u32,
}

impl Fibonacci {
    fn new(limit: Option<u32>) -> Self {
        Fibonacci {
            a: 0,
            b: 1,
            limit,
            count: 0,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.limit {
            Some(limit) if self.count >= limit => return None,
            _ => (),
        }

        let next_val = self.a;
        self.a = self.b;
        self.b = next_val.checked_add(self.b)?;
        self.count += 1;
        Some(next_val)
    }
}

#[cfg(test)]
mod tests {
    use super::Fibonacci;

    #[test]
    fn test_fibonacci_sequence() {
        let mut fib = Fibonacci::new(Some(10));
        let expected = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
        for &val in expected.iter() {
            assert_eq!(fib.next().unwrap(), val);
        }
        assert_eq!(fib.next(), None);
    }

    #[test]
    fn test_fibonacci_limit() {
        let mut fib = Fibonacci::new(Some(5));
        let mut count = 0;
        while fib.next().is_some() {
            count += 1;
        }
        assert_eq!(count, 5);
    }

    #[test]
    fn test_fibonacci_no_limit() {
        let mut fib = Fibonacci::new(None);
        // Check the first few values without a limit
        assert_eq!(fib.next().unwrap(), 0);
        assert_eq!(fib.next().unwrap(), 1);
        assert_eq!(fib.next().unwrap(), 1);
        assert_eq!(fib.next().unwrap(), 2);
        assert_eq!(fib.next().unwrap(), 3);
    }

    #[test]
    fn test_fibonacci_limit_0() {
        let mut fib = Fibonacci::new(Some(0));
        assert_eq!(fib.next(), None);
    }

    #[test]
    fn test_fibonacci_limit_1() {
        let mut fib = Fibonacci::new(Some(1));
        assert_eq!(fib.next().unwrap(), 0);
        assert_eq!(fib.next(), None);
    }
}
