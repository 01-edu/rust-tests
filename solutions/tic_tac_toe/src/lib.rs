const PLAYERS: &[char] = &['X', 'O'];

pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    PLAYERS
        .iter()
        .copied()
        .find(|&p| horizontal(p, table) || vertical(p, table) || diagonals(p, table))
        .map(|w| format!("player {w} won"))
        .unwrap_or_else(|| "tie".to_owned())
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    let is_pyramid = (0..table.len()).map(|i| table[i][i]).all(|c| c == player);

    let is_inverted_pyramid = (0..table.len())
        .map(|i| table[i][table.len() - 1 - i])
        .all(|c| c == player);

    is_pyramid || is_inverted_pyramid
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    table
        .into_iter()
        .any(|r| r.into_iter().all(|c| c == player))
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    (0..table.len()).any(|i| {
        table
            .into_iter()
            .flatten()
            .skip(i)
            .step_by(table.len())
            .all(|c| c == player)
    })
}
