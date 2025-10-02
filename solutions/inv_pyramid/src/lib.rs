#[inline]
pub fn inv_pyramid(st: String, max: usize) -> Vec<String> {
    (1..=max)
        .chain((1..max).rev())
        .map(|i| format!("{:>1$}", st.repeat(i), 2 * i))
        .collect()
}
