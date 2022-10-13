#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    pub validation: bool,
    pub expected: String,
}

impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError {
            validation,
            expected,
        }
    }
}

pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    if ciphered == "" || original == "" {
        return None;
    }
    match decode(original) == ciphered {
        true => Some(Ok(true)),
        false => Some(Err(CipherError::new(
            decode(original) == ciphered,
            decode(original),
        ))),
    }
}

fn decode(original: &str) -> String {
    original
        .chars()
        .map(|letter| match letter.is_ascii_alphabetic() {
            true => {
                match letter.is_uppercase() {
                    true => (((25 - (letter as u32 - 'A' as u32)) + 'A' as u32) as u8 as char)
                        .to_string(),
                    false => (((25 - (letter as u32 - 'a' as u32)) + 'a' as u32) as u8 as char)
                        .to_string(),
                }
            }
            false => letter.to_string(),
        })
        .collect::<Vec<String>>()
        .join("")
}
