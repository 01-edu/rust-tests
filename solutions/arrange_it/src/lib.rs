use itertools::Itertools;

pub fn arrange_phrase(phrase: &str) -> String {
    let indexes = phrase
        .matches(|c: char| c.is_ascii_digit())
        .map(|c| c.parse::<u32>().unwrap());

    let stripped = phrase.replace(|c: char| c.is_ascii_digit(), "");

    stripped
        .split_ascii_whitespace()
        .zip(indexes)
        .sorted_unstable_by_key(|&(_, i)| i)
        .map(|(w, _)| w)
        .collect::<Vec<_>>()
        .join(" ")
}
