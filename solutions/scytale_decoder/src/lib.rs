pub fn scytale_decoder(s: String, letters_per_turn: u32) -> Option<String> {
    if s.is_empty() || letters_per_turn == 0 {
        None
    } else {
        let mut decoded_s = "".to_string();
        let mut start = 0;

        while decoded_s.len() < s.len() {
            decoded_s.push(s.chars().nth(start).unwrap());

            let mut next = start + letters_per_turn as usize;
            while next < s.len() {
                decoded_s.push(s.chars().nth(next).unwrap());
                next += letters_per_turn as usize;
            }
            start += 1;
        }
        Some(decoded_s)
    }
}
