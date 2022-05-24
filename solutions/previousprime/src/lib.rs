pub fn prev_prime(nbr: u64) -> u64 {
    if nbr <= 2 {
        return 0;
    }

    let mut min_number = 2;

    for i in 2..nbr {
        if is_prime(i) {
            min_number = i;
        }
    }

    min_number
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
