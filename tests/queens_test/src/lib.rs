#[cfg(test)]
mod tests {
    use queens::*;

    #[test]
    fn test_position_is_valid() {
        assert!(ChessPosition::new(2, 4).is_some());
        assert!(ChessPosition::new(8, 2).is_none());
        assert!(ChessPosition::new(5, 8).is_none());
    }

    #[test]
    fn test_queen_valid_position() {
        Queen::new(ChessPosition::new(2, 4).unwrap());
    }

    #[test]
    fn test_can_not_attack() {
        [
            (2, 4),
            (6, 6),
            (1, 2),
            (0, 4),
            (5, 3),
            (0, 6),
            (3, 7),
            (2, 0),
        ]
        .map(|pos| Queen::new(ChessPosition::new(pos.0, pos.1).unwrap()))
        .as_chunks() // use array_chunks when stabilised
        .0
        .iter()
        .for_each(|&[w, b]| {
            assert!(!w.can_attack(b));
        });
    }

    #[test]
    fn test_same_rank() {
        [
            (2, 4),
            (2, 6),
            (1, 2),
            (1, 6),
            (4, 7),
            (4, 3),
            (5, 3),
            (5, 1),
        ]
        .map(|pos| Queen::new(ChessPosition::new(pos.0, pos.1).unwrap()))
        .as_chunks()
        .0
        .iter()
        .for_each(|&[w, b]| {
            assert!(w.can_attack(b));
        });
    }

    #[test]
    fn test_same_file() {
        [
            (4, 5),
            (3, 5),
            (2, 2),
            (3, 2),
            (2, 6),
            (1, 6),
            (2, 7),
            (5, 7),
        ]
        .map(|pos| Queen::new(ChessPosition::new(pos.0, pos.1).unwrap()))
        .as_chunks()
        .0
        .iter()
        .for_each(|&[w, b]| {
            assert!(w.can_attack(b));
        });
    }

    #[test]
    fn test_same_diagonal() {
        [
            (2, 2),
            (0, 4),
            (2, 2),
            (3, 1),
            (2, 2),
            (1, 1),
            (2, 2),
            (5, 5),
        ]
        .map(|pos| Queen::new(ChessPosition::new(pos.0, pos.1).unwrap()))
        .as_chunks()
        .0
        .iter()
        .for_each(|&[w, b]| {
            assert!(w.can_attack(b));
        });
    }
}
