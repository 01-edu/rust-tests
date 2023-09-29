pub fn is_luhn_formula(code: &str) -> bool {
    if code.trim().len() <= 1 {
        return false;
    }
    code.as_bytes()
        .iter()
        .rev()
        .filter(|&c| *c != b' ')
        .enumerate()
        .try_fold(0, |acc, (i, &c)| {
            if c.is_ascii_digit() {
                Some(
                    acc + match (i % 2 == 1, c - b'0') {
                        (true, nb) if nb > 4 => nb * 2 - 9,
                        (true, nb) => nb * 2,
                        (false, nb) => nb,
                    },
                )
            } else {
                None
            }
        })
        .map_or(false, |checksum| checksum % 10 == 0)
}
