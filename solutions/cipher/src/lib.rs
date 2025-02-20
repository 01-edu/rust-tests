#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let decoded = decode(original);

    if decoded == ciphered {
        Ok(())
    } else {
        Err(CipherError { expected: decoded })
    }
}

const CHARS: u8 = 'z' as u8 - 'a' as u8;

fn decode(original: &str) -> String {
    original
        .chars()
        .map(|l| {
            if l.is_ascii_alphabetic() {
                let offset = (if l.is_ascii_uppercase() { 'A' } else { 'a' }) as u8;
                ((CHARS - (l as u8 - offset)) + offset) as char
            } else {
                l
            }
        })
        .collect()
}
