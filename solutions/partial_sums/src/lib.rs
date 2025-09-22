use std::iter;

#[inline]
pub fn parts_sums(arr: &[u64]) -> Vec<u64> {
    iter::successors(Some(arr), |&v| {
        if v.is_empty() {
            None
        } else {
            Some(&v[0..v.len() - 1])
        }
    })
    .map(|v| v.iter().sum())
    .collect()
}
