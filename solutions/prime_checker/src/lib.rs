#[derive(PartialEq, Eq, Debug)]
pub enum PrimeErr {
    Even,
    Divider(usize),
}

pub fn prime_checker(n: usize) -> Option<Result<usize, PrimeErr>> {
    if n < 2 {
        return None;
    }
    if n <= 3 {
        return Some(Ok(n));
    }
    if n.is_multiple_of(2) {
        return Some(Err(PrimeErr::Even));
    }
    if n.is_multiple_of(3) {
        return Some(Err(PrimeErr::Divider(3)));
    }

    let sqrt_n = (n as f64).sqrt() as usize;
    let mut d = 5;
    let mut step = 2;

    while d <= sqrt_n {
        if n.is_multiple_of(d) {
            return Some(Err(PrimeErr::Divider(d)));
        }
        d += step;
        step = 6 - step;
    }

    Some(Ok(n))
}
