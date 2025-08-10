pub fn prev_prime(nbr: u64) -> u64 {
    (2..nbr)
        .rev()
        .find(|&n| (2..).take_while(|d| d * d <= n).all(|d| n % d != 0))
        .unwrap_or(0)
}
