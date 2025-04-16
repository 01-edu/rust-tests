pub fn is_luhn_formula(code: &str) -> bool {
    let it = code.chars().filter(|c| !c.is_ascii_whitespace());
    if it.clone().count() <= 1 {
        false
    } else {
        it.rev()
            .enumerate()
            .try_fold(0, |acc, (i, c)| {
                c.to_digit(10).map(|c| {
                    acc + if i % 2 == 1 {
                        if c > 4 {
                            c * 2 - 9
                        } else {
                            c * 2
                        }
                    } else {
                        c
                    }
                })
            })
            .map_or(false, |n| n % 10 == 0)
    }
}
