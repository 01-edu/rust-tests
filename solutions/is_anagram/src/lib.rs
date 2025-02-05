pub fn is_anagram(s1: &str, s2: &str) -> bool {
    // Early return if the lengths of the strings are different
    if s1.len() != s2.len() {
        return false;
    }

    let mut char_count = std::collections::HashMap::new();

    // Count character occurrences in the first string
    for c in s1.to_lowercase().chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }

    // Decrement character occurrences for the second string
    for c in s2.to_lowercase().chars() {
        if let Some(count) = char_count.get_mut(&c) {
            *count -= 1;
            if *count < 0 {
                return false; // Character count mismatch
            }
        } else {
            return false; // Character not found in the first string
        }
    }

    // Check if all character counts are zero (anagrams)
    char_count.values().all(|&count| count == 0)
}
