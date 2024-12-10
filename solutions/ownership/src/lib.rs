pub fn first_subword(s: String) -> String {
    let pos = s
        .char_indices()
        .position(|(i, c)| i != 0 && (c.is_ascii_uppercase() || c == '_'))
        .unwrap_or(s.len());

    s[0..pos].to_owned()
}
