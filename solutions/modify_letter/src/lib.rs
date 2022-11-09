pub fn remove_letter_sensitive(s: &str, letter: char) -> String {
    s.replace(letter, "")
}

pub fn remove_letter_insensitive(s: &str, letter: char) -> String {
    let lower_letter = letter.to_lowercase().to_string();
    let upper_letter = letter.to_uppercase().to_string();
    s.replace(&upper_letter, &lower_letter)
        .replace(&lower_letter, "")
}

pub fn swap_letter_case(s: &str, letter: char) -> String {
    s.split("")
        .into_iter()
        .map(|s| {
            s.chars()
                .filter_map(|c| {
                    if c.to_string() == letter.to_lowercase().to_string() {
                        c.to_uppercase().next()
                    } else if c.to_string() == letter.to_uppercase().to_string() {
                        c.to_lowercase().next()
                    } else {
                        Some(c)
                    }
                })
                .collect()
        })
        .collect::<Vec<String>>()
        .join("")
}
