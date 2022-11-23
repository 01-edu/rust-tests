#[derive(PartialEq, Eq, Debug)]
pub enum PrimeErr {
    Even,
    Divider(u32),
}

pub fn prime_checker(nb: u32) -> Option<Result<u32, PrimeErr>> {
    if nb == 0 || nb == 1 {
        return None;
    } else if nb == 2 {
        return Some(Ok(nb));
    } else if nb % 2 == 0 {
        return Some(Err(PrimeErr::Even));
    } else {
        let mut divider = 3;
        let max_divide = ((nb as f64).sqrt() + 1.0) as u32;
        while divider < max_divide {
            if nb % divider == 0 {
                return Some(Err(PrimeErr::Divider(divider)));
            } else {
                divider += 2;
            }
        }
    }
    Some(Ok(nb))
}
