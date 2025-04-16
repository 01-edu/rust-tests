const VOWELS: &str = "aeiou";

pub fn pig_latin(text: &str) -> String {
    let first_vowel = text
        .find(|c: char| VOWELS.contains(c.to_ascii_lowercase()))
        .unwrap_or(text.len());

    let (start, rem) = text.split_at(
        if first_vowel != 0 && text.len() >= 3 && &text[1..3] == "qu" {
            3
        } else {
            first_vowel
        },
    );

    format!("{rem}{start}ay")
}
