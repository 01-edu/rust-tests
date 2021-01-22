pub fn scytale_cipher(s: String, i: u32) -> String {
    if i as usize >= s.chars().count() || i == 1 {
        return s.to_string();
    }

    let width = (s.chars().count() as f64 / i as f64).ceil() as usize;
    let mut table = vec![vec![' '; width]; i as usize];

    for (pos, element) in s.chars().enumerate() {
        let col = pos % i as usize;
        let row = pos / i as usize;

        table[col][row] = element;
    }
    table
        .iter()
        .flatten()
        .collect::<String>()
        .trim_end()
        .to_string()
}
