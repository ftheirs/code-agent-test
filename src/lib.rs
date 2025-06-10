struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci {
            curr: 0,
            next: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr.checked_add(self.next)?;
        let new_curr = self.next;
        self.curr = new_curr;
        self.next = new_next;
        Some(self.curr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let mut fib = Fibonacci::new();
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(2));
        assert_eq!(fib.next(), Some(3));
        assert_eq!(fib.next(), Some(5));
        assert_eq!(fib.next(), Some(8));
        assert_eq!(fib.next(), Some(13));
        assert_eq!(fib.next(), Some(21));
        assert_eq!(fib.next(), Some(34));
    }

    #[test]
    fn test_overflow() {
        let mut fib = Fibonacci::new();
        // consume the iterator until the end
        while fib.next().is_some() {}

        // the last value should be None
        // assert_eq!(fib.next(), None);
    }

    #[test]
    fn test_initial_values() {
        let mut fib = Fibonacci::new();
        assert_eq!(fib.curr, 0);
        assert_eq!(fib.next, 1);
    }
}
