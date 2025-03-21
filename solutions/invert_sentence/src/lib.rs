// This exercise is quite difficult and should come later in the piscine.

pub fn invert_sentence(string: &str) -> String {
    let words = string.split_ascii_whitespace();

    let positions = string
        .split_ascii_whitespace()
        .map(|s| (s.as_ptr() as usize - string.as_ptr() as usize, s.len()))
        .map(|(i, l)| i..=l + i - 1)
        .rev();

    let mut acc = string.to_owned();

    positions
        .zip(words)
        .for_each(|(r, s)| acc.replace_range(r, s));

    acc
}
