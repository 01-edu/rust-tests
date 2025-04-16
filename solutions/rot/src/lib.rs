const ALPHABET_LEN: i8 = (b'Z' - b'A' + 1) as _;

pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| {
            if !c.is_ascii_alphabetic() {
                c
            } else {
                let lower_bound = if c.is_ascii_lowercase() { b'a' } else { b'A' };

                (((c as u8 - lower_bound) as i8 + key).rem_euclid(ALPHABET_LEN) as u8 + lower_bound)
                    as _
            }
        })
        .collect()
}
