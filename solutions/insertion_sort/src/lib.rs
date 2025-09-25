pub fn insertion_sort(slice: &mut [i32], steps: usize) {
    for i in 1..=steps.min(slice.len().saturating_sub(1)) {
        let mut j = i;

        while j > 0 && slice[j] < slice[j - 1] {
            slice.swap(j, j - 1);
            j -= 1;
        }
    }
}
