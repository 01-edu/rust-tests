use std::iter;

#[inline]
pub fn prev_prime(nbr: usize) -> usize {
    if nbr > 2 {
        iter::once(2)
            .chain((3..nbr).step_by(2))
            .rfind(|&n| is_prime(n))
            .unwrap_or_default()
    } else {
        Default::default()
    }
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
    let mut d = 5;
    while d * d <= n {
        if n.is_multiple_of(d) || n.is_multiple_of(d + 2) {
            return false;
        }
        d += 6;
    }
    true
}
