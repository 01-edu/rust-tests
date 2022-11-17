#[derive(PartialEq, Eq, Debug)]
pub enum PrimeErr {
    Even,
    Divider(u32),
}

pub fn prime_checker(nb: u32) -> Option<Result<u32, PrimeErr>> {
    if let 0 = nb {
        return None;
    }
    if let 1 = nb {
        return None;
    }
    if let 2 = nb {
        return Some(Ok(nb));
    }
    if nb % 2 == 0 {
        return Some(Err(PrimeErr::Even));
    } else {
        let mut divider = 3;
        while divider < nb {
            if nb % divider == 0 {
                return Some(Err(PrimeErr::Divider(divider)));
            } else {
                divider += 1;
            }
        }
    }
    Some(Ok(nb))
}
