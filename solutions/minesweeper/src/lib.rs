const OFFSET: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub fn solve_board(minefield: &[&str]) -> Vec<String> {
    let mut field: Vec<Vec<u8>> = minefield
        .iter()
        .map(|line| line.as_bytes().to_owned())
        .collect();

    let x_max = field.first().unwrap_or(&vec![]).len();
    let y_max = field.len();

    minefield.iter().enumerate().for_each(|(y, line)| {
        line.as_bytes()
            .iter()
            .enumerate()
            .filter(|(_, &c)| c == b'*')
            .for_each(|(x, _)| {
                OFFSET
                    .iter()
                    .filter(|(x_off, y_off)| {
                        !(*x_off == -1 && x == 0)
                            && *x_off + (x as i32) < x_max as i32
                            && !(*y_off == -1 && y == 0)
                            && *y_off + (y as i32) < y_max as i32
                    })
                    .map(|&(x_off, y_off)| {
                        (((x as i32) + x_off) as usize, ((y as i32) + y_off) as usize)
                    })
                    .for_each(|(x, y)| {
                        let curr_val = field[y][x];

                        if curr_val != b'*' {
                            if curr_val == b' ' {
                                field[y][x] = b'1';
                            } else {
                                field[y][x] = curr_val + 1;
                            }
                        }
                    })
            })
    });

    field
        .into_iter()
        .map(|line| unsafe { String::from_utf8_unchecked(line) })
        .collect()
}
