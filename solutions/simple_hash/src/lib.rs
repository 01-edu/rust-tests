use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut hash = HashMap::with_capacity(words.len() / 2);

    words
        .iter()
        .copied()
        .for_each(|w| *hash.entry(w).or_default() += 1);

    hash
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}
