pub fn binary_search<T: Ord>(slice: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = slice.len();

    while low < high {
        let mid = low + (high - low) / 2;

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
        let arr = [2, 5, 7, 8, 11, 12];
        assert_eq!(binary_search(&arr, &11), Some(4));
    }

    #[test]
    fn test_not_found() {
        let arr = [2, 5, 7, 8, 11, 12];
        assert_eq!(binary_search(&arr, &13), None);
    }

    #[test]
    fn test_empty_slice() {
        let arr: [i32; 0] = [];
        assert_eq!(binary_search(&arr, &13), None);
    }
}
