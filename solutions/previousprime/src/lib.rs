pub fn prev_prime(nbr: usize) -> usize {
    if nbr <= 2 {
        return 0;
    }

    let start = if nbr.is_multiple_of(2) {
        nbr - 1
    } else {
        nbr - 2
    };

    (1..=start)
        .rev()
        .step_by(2)
        .find(|&n| is_prime(n))
        .unwrap_or(2)
}

fn is_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n.is_multiple_of(2) || n.is_multiple_of(3) {
        return false;
    }

    if n.is_multiple_of(5) || n.is_multiple_of(7) || n.is_multiple_of(11) || n.is_multiple_of(13) {
        return n <= 13;
    }

    let sqrt_n = (n as f64).sqrt() as usize;
    let mut d = 17;
    let mut step = 2;

    while d <= sqrt_n {
        if n % d == 0 {
            return false;
        }
        d += step;
        step = 6 - step;
    }
    true
}
