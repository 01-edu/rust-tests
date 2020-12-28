/*
## cipher

### Instructions

The Atbash cipher is a encryption method in which each letter of a word is replaced with its mirror letter in the alphabet

Your objective is to create a function called `cipher` this must return a Result wrapped in an Option, this result should return either a boolean
or an Error being the structure `CipherError`. This structure should be the error type for the function `cipher`

This function should compare the original string wih the ciphered string. returning true if the cipher is correct otherwise the error type
CipherErr with the a true or false if it is validated and the proper atbash cipher.

### Example

```rust
fn main() {
    println!("{:?}", cipher("1Hello 2world!", "1Svool 2dliow!"));
    // Some(Ok(true))
    println!("{:?}", cipher("1Hello 2world!", "svool"));
    // Some(Err(CipherError { validation: false, expected: "1Svool 2dliow!" }))
    println!("{:?}", cipher("", "svool"));
    // None
}
```
*/
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    pub validation: bool,
    pub expected: String,
}

impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError { validation, expected }
    }
}

impl fmt::Display for CipherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.validation {
            write!(f, "cipher validation : {}\ncorrect: {}", &self.validation, &self.expected)
        } else {
            write!(f, "cipher validation : {}\nexpected: {}", &self.validation, &self.expected)
        }
    }
}

pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    if ciphered == "" || original == "" {
        return None;
    }
    match decode(original) == ciphered {
        true => Some(Ok(true)),
        false => Some(Err(CipherError::new(decode(original) == ciphered, decode(original)))),
    }
}

fn decode(original: &str) -> String {
    original.chars()
        .map(| letter | {
            match letter.is_ascii_alphabetic() {
                true => match letter.is_uppercase() {
                    true => (((25 - (letter as u32 - 'A' as u32)) + 'A' as u32) as u8 as char).to_string(),
                    false => (((25 - (letter as u32 - 'a' as u32)) + 'a' as u32) as u8 as char).to_string(),
                },
                false => letter.to_string(),
            }
        }).collect::<Vec<String>>().join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cipher() {
        assert_eq!(cipher("1Hello 2world!", "1Svool 2dliow!"), Some(Ok(true)));
        assert_eq!(cipher("", "1Svool 2dliow!"), None);
        assert_eq!(cipher("1Hello 2world!", ""), None);
        assert_eq!(cipher("1Hello 2world!", "1svool 2dliow!"), Some(Err(CipherError { validation: false, expected: String::from("1Svool 2dliow!") })));
        assert_eq!(cipher("asdasd", "zhwzhw"), Some(Ok(true)));
        assert_eq!(cipher("asdasd", "lkdas"), Some(Err(CipherError { validation: false, expected: String::from("zhwzhw") })));
        assert_eq!(cipher("3(/&%fsd 32das", "3(/&%uhw 32wzh"), Some(Ok(true)));
        assert_eq!(cipher("3(/&%sd 32das", "3(/&%uhw 32wzh"), Some(Err(CipherError { validation: false, expected: String::from("3(/&%hw 32wzh") })));
    }
}