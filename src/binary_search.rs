fn binary_search<T: Ord>(slice: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = slice.len();

    while low < high {
        let mid = (low + high) / 2;

        match slice[mid].cmp(target) {
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
            std::cmp::Ordering::Equal => return Some(mid),
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::binary_search;

    #[test]
    fn test_found() {
        let slice = [2, 4, 6, 8, 10];
        let target = 6;
        assert_eq!(binary_search(&slice, &target), Some(2));
    }

    #[test]
    fn test_not_found() {
        let slice = [2, 4, 6, 8, 10];
        let target = 5;
        assert_eq!(binary_search(&slice, &target), None);
    }

    #[test]
    fn test_empty_slice() {
        let slice: [i32; 0] = [];
        let target = 5;
        assert_eq!(binary_search(&slice, &target), None);
    }
}
