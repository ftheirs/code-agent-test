// Binary search implementation in Rust

// Function to perform binary search on a sorted slice
fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2; // Prevent potential overflow

        match arr[mid].cmp(target) {
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
            std::cmp::Ordering::Equal => return Some(mid),
        }
    }

    None // Target not found
}

// Example usage
#[cfg(test)]
mod tests {
    use super::binary_search;

    #[test]
    fn test_integer_search() {
        let sorted_integers = [2, 4, 6, 8, 10];
        assert_eq!(binary_search(&sorted_integers, &6), Some(2));
        assert_eq!(binary_search(&sorted_integers, &5), None);
    }

    #[test]
    fn test_float_search() {
        let sorted_floats = [2.0, 4.0, 6.0, 8.0, 10.0];
        assert_eq!(binary_search(&sorted_floats, &6.0), Some(2));
        assert_eq!(binary_search(&sorted_floats, &5.0), None);
    }

    #[test]
    fn test_string_search() {
        let sorted_strings = ["apple", "banana", "cherry", "date"];
        assert_eq!(binary_search(&sorted_strings, &"cherry"), Some(2));
        assert_eq!(binary_search(&sorted_strings, &"grape"), None);
    }

    #[test]
    fn test_custom_struct_search() {
        #[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
        struct Point { x: i32, y: i32 }

        let sorted_points = [
            Point { x: 1, y: 1 },
            Point { x: 2, y: 2 },
            Point { x: 3, y: 3 },
        ];

        assert_eq!(binary_search(&sorted_points, &Point { x: 2, y: 2 }), Some(1));
        assert_eq!(binary_search(&sorted_points, &Point { x: 4, y: 4 }), None);
    }
}
