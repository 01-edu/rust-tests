pub fn search(array: &[i32], key: i32) -> Option<usize> {
    array.iter().copied().rposition(|e| e == key)
}
