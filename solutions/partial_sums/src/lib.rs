pub fn parts_sums(ls: &[u64]) -> Vec<u64> {
    let mut res = vec![];
    let mut sum: u64 = ls.iter().sum();
    res.push(sum);
    for nbr in ls.iter().rev() {
        sum -= nbr;
        res.push(sum);
    }
    res
}
