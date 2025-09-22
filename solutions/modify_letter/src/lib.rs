#[inline]
pub fn remove_letter_sensitive(s: &str, letter: char) -> String {
    s.replace(letter, "")
}

#[inline]
pub fn remove_letter_insensitive(s: &str, letter: char) -> String {
    s.replace(|c: char| c.eq_ignore_ascii_case(&letter), "")
}

#[inline]
pub fn swap_letter_case(s: &str, letter: char) -> String {
    s.chars()
        .map(|c| {
            if c.eq_ignore_ascii_case(&letter) {
                if c.is_lowercase() {
                    c.to_ascii_uppercase()
                } else {
                    c.to_ascii_lowercase()
                }
            } else {
                c
            }
        })
        .collect()
}
