pub fn binary_search<T: Ord>(slice: &[T], target: &T) -> Option<usize> {
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
    use super::*;

    #[test]
    fn test_binary_search() {
        let arr = [2, 5, 7, 8, 11, 12];
        assert_eq!(binary_search(&arr, &13), None);
        assert_eq!(binary_search(&arr, &12), Some(5));
        assert_eq!(binary_search(&arr, &0), None);
        assert_eq!(binary_search(&arr, &8), Some(3));
    }
}
