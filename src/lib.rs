//! A simple Fibonacci sequence generator.

/// A struct representing the Fibonacci sequence.
#[derive(Debug)]
pub struct Fibonacci {
    a: u32,
    b: u32,
}

impl Fibonacci {
    /// Creates a new Fibonacci sequence starting with 0 and 1.
    pub fn new() -> Self {
        Fibonacci {
            a: 0,
            b: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // Calculate the next Fibonacci number.
        let next = self.a.checked_add(self.b)?;

        // Update the state of the Fibonacci sequence.
        self.a = self.b;
        self.b = next;

        Some(self.a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let mut fib = Fibonacci::new();
        assert_eq!(fib.next(), Some(0));
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
    fn test_fibonacci_limit() {
        let mut fib = Fibonacci::new();
        let mut count = 0;
        while let Some(_) = fib.next() {
            count += 1;
        }
        assert_eq!(count, 47); // The 47th Fibonacci number is the largest that fits in a u32
    }
}
