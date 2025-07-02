pub fn is_pangram(s: &str) -> bool {
    ('a'..='z').all(|c| s.to_lowercase().contains(c))
}
