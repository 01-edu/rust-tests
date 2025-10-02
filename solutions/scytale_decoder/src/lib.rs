#[inline]
pub fn scytale_decoder(s: String, letters_per_turn: usize) -> Option<String> {
    if s.is_empty() || letters_per_turn == 0 {
        None
    } else {
        Some(
            (0..letters_per_turn)
                .flat_map(|i| {
                    (i..s.len())
                        .step_by(letters_per_turn)
                        .map(|j| s.chars().nth(j).unwrap())
                })
                .collect(),
        )
    }
}
