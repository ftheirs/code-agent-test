/// Performs a binary search on a sorted list.
///
/// # Arguments
///
/// * `list` - A sorted list of elements.
/// * `target` - The element to search for.
///
/// # Returns
///
/// The index of the target element if found, otherwise `None`.
///
/// # Examples
///
/// ```
/// let list = [2, 4, 6, 8, 10];
/// let target = 6;
/// let result = binary_search(&list, &target);
/// assert_eq!(result, Some(2));
///
/// let list = [2, 4, 6, 8, 10];
/// let target = 7;
/// let result = binary_search(&list, &target);
/// assert_eq!(result, None);
/// ```
pub fn binary_search<T: Ord>(list: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = list.len();

    while low < high {
        let mid = (low + high) / 2;

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
    fn test_binary_search_found() {
        let list = [2, 4, 6, 8, 10];
        let target = 6;
        let result = binary_search(&list, &target);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_binary_search_not_found() {
        let list = [2, 4, 6, 8, 10];
        let target = 7;
        let result = binary_search(&list, &target);
        assert_eq!(result, None);
    }

    #[test]
    fn test_binary_search_empty_list() {
        let list: [i32; 0] = [];
        let target = 7;
        let result = binary_search(&list, &target);
        assert_eq!(result, None);
    }
}
