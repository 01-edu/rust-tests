#[inline]
pub fn get_products<const N: usize>(arr: [u32; N]) -> [u32; N] {
    std::array::from_fn(|i| {
        arr.into_iter()
            .enumerate()
            .filter(|&(j, _)| j != i)
            .map(|(_, v)| v)
            .product()
    })
}
