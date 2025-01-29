use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let map = s1
        .chars()
        .fold(HashMap::with_capacity(s1.len() / 2), |mut m, c| {
            *m.entry(c).or_default() += 1;
            m
        });

    map.into_iter().all(|(c, i)| s2.matches(c).count() == i)
}
