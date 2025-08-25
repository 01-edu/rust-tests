pub fn scytale_decoder(s: String, letters_per_turn: u32) -> Option<String> {
    let k = letters_per_turn as usize;
    if s.is_empty() || k == 0 {
        return None;
    }
    let mut res = vec![String::new(); k];
    for (i, c) in s.chars().enumerate() {
        res[i % k].push(c);
    }
    Some(res.concat())
}
