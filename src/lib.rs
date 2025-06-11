ub fn generic_search<T: PartialOrd + Sized>(list: &[T], target: T) -> Option<usize> {
    if list.is_empty() {
        return None;
    }

    let mut low = 0;
    let mut high = list.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        if list[mid] == target {
            return Some(mid);
        } else if list[mid] < target {
            low = mid + 1;
        } else {
            if mid == 0 { // Avoid underflow if target is smaller than the smallest element
                return None;
            }
            high = mid - 1;
        }
    }
    None
}
