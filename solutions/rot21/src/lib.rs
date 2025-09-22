const ROT: u8 = 21;
const ALPHABET_SIZE: u8 = 26;

#[inline]
pub fn rot21(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if !c.is_ascii_alphabetic() {
                c
            } else {
                let init = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                ((c as u8 - init + ROT) % ALPHABET_SIZE + init) as char
            }
        })
        .collect()
}
