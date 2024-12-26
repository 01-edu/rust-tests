pub fn capitalize_first(input: &str) -> String {
    if input.is_empty() {
        String::new()
    } else {
        format!(
            "{}{}",
            input.chars().next().unwrap().to_ascii_uppercase(),
            &input[1..]
        )
    }
}

pub fn title_case(input: &str) -> String {
    input
        .split_inclusive(|c: char| c.is_ascii_whitespace())
        .map(capitalize_first)
        .collect::<Vec<_>>()
        .concat()
}

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                c.to_ascii_lowercase()
            } else {
                c.to_ascii_uppercase()
            }
        })
        .collect()
}
