ub fn generic_search<T: PartialOrd + Sized>(list: &[T], target: T) -> Option<usize> {
    let mut low = 0;
    let mut high = list.len();

    while low < high {
        let mid = low + (high - low) / 2;
        if list[mid] == target {
            return Some(mid);
        } else if list[mid] < target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    None
}