#[inline]
pub fn first_fifty_even_square() -> Vec<i32> {
    (1..)
        .filter(|x| x % 2 == 0)
        .take(50)
        .map(|x| x * x)
        .collect()
}
