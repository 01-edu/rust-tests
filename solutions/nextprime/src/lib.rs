pub fn next_prime(nbr: u64) -> u64 {
    if is_prime(nbr) {
        return nbr;
    }
    next_prime(nbr + 1)
}

fn is_prime(nbr: u64) -> bool {
    if nbr > 0 {
        if nbr <= 1 {
            return false;
        }
        let mut d = 2;
        while d * d <= nbr {
            if nbr % d == 0 {
                return false;
            }
            d += 1;
        }
        return true;
    } else {
        return false;
    }
}
