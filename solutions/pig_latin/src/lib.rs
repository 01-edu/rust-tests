/*
## pig_latin

### Instructions

Write a function that transforms a string passed as argument in its `Pig Latin` version.

The rules used by Pig Latin are the following:

- If a word begins with a vowel, just add "ay" to the end.
- If it begins with a consonant, then we take all consonants before the first vowel and we put them on the end of the word and add "ay" at the end.
- If a word starts with a consonant followed by "qu", move it to the end of the word, and then add an "ay" at the end.

### Notions

- https://doc.rust-lang.org/book/ch18-00-patterns.html

### Expected functions

```rust
pub fn pig_latin(text: &str) -> String {}
```

### Usage

Here is a program to test your function.

```rust
fn main() {
    println!("{}", pig_latin(&String::from("igloo")));
    println!("{}", pig_latin(&String::from("apple")));
    println!("{}", pig_latin(&String::from("hello")));
    println!("{}", pig_latin(&String::from("square")));
    println!("{}", pig_latin(&String::from("xenon")));
    println!("{}", pig_latin(&String::from("chair")));
}
```
*/

pub fn pig_latin(text: &str) -> String {
    text.split_whitespace()
        .map(|s| match (&s[0..1], &s[1..2], &s[2..3]) {
            ("a", _, _)
            | ("e", _, _)
            | ("u", _, _)
            | ("i", _, _)
            | ("o", _, _)
            | ("y", "t", _)
            | ("x", "r", _) => s.to_owned() + "ay",

            ("t", "h", "r") | ("s", "c", "h") | (_, "q", "u") => {
                format!("{}{}ay", &s[3..], &s[0..3])
            }

            ("c", "h", _) | ("t", "h", _) | ("q", "u", _) => format!("{}{}ay", &s[2..], &s[0..2]),

            _ => format!("{}{}ay", &s[1..], &s[0..1]),
        })
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_beginning_with_vowel() {
        assert_eq!(pig_latin(&String::from("apple")), "appleay");
        assert_eq!(pig_latin(&String::from("ear")), "earay");
        assert_eq!(pig_latin(&String::from("igloo")), "iglooay");
        assert_eq!(pig_latin(&String::from("object")), "objectay");
        assert_eq!(pig_latin(&String::from("under")), "underay");
    }

    #[test]

    fn test_word_beginning_with_consonant() {
        assert_eq!(pig_latin(&String::from("queen")), "eenquay");
        assert_eq!(pig_latin(&String::from("square")), "aresquay");
        assert_eq!(pig_latin(&String::from("equal")), "equalay");
        assert_eq!(pig_latin(&String::from("pig")), "igpay");
        assert_eq!(pig_latin(&String::from("koala")), "oalakay");
        assert_eq!(pig_latin(&String::from("yellow")), "ellowyay");
        assert_eq!(pig_latin(&String::from("xenon")), "enonxay");
        assert_eq!(pig_latin(&String::from("qat")), "atqay");
        assert_eq!(pig_latin(&String::from("chair")), "airchay");
        assert_eq!(pig_latin(&String::from("therapy")), "erapythay");
        assert_eq!(pig_latin(&String::from("thrush")), "ushthray");
        assert_eq!(pig_latin(&String::from("school")), "oolschay");
    }
}
