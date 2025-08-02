const VECTORS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn solve_board(minefield: &[&str]) -> Vec<String> {
    let mut copy = minefield
        .iter()
        .map(|c| c.as_bytes().to_owned())
        .collect::<Vec<_>>();

    minefield
        .iter()
        .enumerate()
        .flat_map(|(y, l)| l.chars().enumerate().map(move |(x, c)| ((y, x), c)))
        .filter(|&(_, c)| c == '*')
        .for_each(|(p, _)| {
            VECTORS
                .into_iter()
                .filter_map(|v| {
                    Some((
                        usize::try_from(p.0 as isize + v.0).ok()?,
                        usize::try_from(p.1 as isize + v.1).ok()?,
                    ))
                })
                .for_each(|(y, x)| {
                    if let Some(c) = copy.get_mut(y).and_then(|y| y.get_mut(x)) {
                        if *c != b'*' {
                            if *c == b' ' {
                                *c = b'1';
                            } else {
                                *c += 1;
                            }
                        }
                    }
                })
        });

    copy.into_iter()
        .map(|v| String::from_utf8(v).unwrap())
        .collect()
}
