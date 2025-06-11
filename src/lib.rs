pub fn binary_search<T: Ord>(list: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = list.len();

    while low < high {
        let mid = low + (high - low) / 2;

        match list[mid].cmp(target) {
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
        let list = [2, 5, 7, 8, 11, 12];
        assert_eq!(binary_search(&list, &13), None);
        assert_eq!(binary_search(&list, &12), Some(5));
        assert_eq!(binary_search(&list, &0), None);
        assert_eq!(binary_search(&list, &2), Some(0));
    }

    #[test]
    fn test_empty_list() {
        let list: [i32; 0] = [];
        assert_eq!(binary_search(&list, &1), None);
    }

    #[test]
    fn test_string_list() {
        let list = ["apple", "banana", "cherry"];
        assert_eq!(binary_search(&list, &"banana"), Some(1));
        assert_eq!(binary_search(&list, &"grape"), None);
    }
}