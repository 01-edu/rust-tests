pub fn next_prime(nbr: u64) -> u64 {
    (nbr.max(2)..).find(|&n| (2..).take_while(|d| d * d <= n).all(|d| n % d != 0)).unwrap()
}
