pub fn invert_sentence(string: &str) -> String {
    let mut words: Vec<&str> = Vec::new();
    let mut word_start = None;

    for (i, c) in string.char_indices() {
        if c.is_whitespace() {
            if let Some(start) = word_start {
                words.push(&string[start..i]);
            }
            words.push(" "); // Preserve spaces
            word_start = None;
        } else {
            if word_start.is_none() {
                word_start = Some(i);
            }
        }
    }

    if let Some(start) = word_start {
        words.push(&string[start..]);
    }

    words.reverse();
    words.join("")
}
