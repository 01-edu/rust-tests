pub fn num_to_ordinal(x: u32) -> String {
    format!(
        "{}{}",
        x,
        if matches!(x % 100, 11..=13) {
            "th"
        } else {
            match x % 10 {
                1 => "st",
                2 => "nd",
                3 => "rd",
                _ => "th",
            }
        }
    )
}
