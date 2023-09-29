
pub fn binary_search(sorted_list: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high;
    let len = sorted_list.len();

    if len == 0 {
        return None;
    } else {
        high = len - 1;
    }

    while low <= high {
        let mid = (low + high) / 2;

        if sorted_list[mid] == target {
            return Some(mid);
        }

        if sorted_list[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    None
}