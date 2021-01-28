/*
Given the position of two queens on a chess board, indicate whether or not they are positioned so that they can attack each other.

In the game of chess, a queen can attack pieces which are on the same row, column, or diagonal.

A chessboard can be represented by an 8 by 8 array.

So if you're told the white queen is at (2, 3) and the black queen at (5, 6), then you'd know you've got a set-up like so:

_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ W _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ B _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _


You'd also be able to answer whether the queens can attack each other.
In this case, that answer would be yes, they can, because both pieces share a diagonal.


fn main() {
    let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());
    let black_queen = Queen::new(ChessPosition::new(0, 4).unwrap());

    println!(
        "Is it possible for the queens to attack each other? {}",
        white_queen.can_attack(&black_queen)
    );

    let white_queen = Queen::new(ChessPosition::new(1, 2).unwrap());
    let black_queen = Queen::new(ChessPosition::new(0, 4).unwrap());

    println!(
        "Is it possible for the queens to attack each other? {}",
        white_queen.can_attack(&black_queen)
    );
}
*/

#[derive(Debug)]
pub struct ChessPosition {
    pub rank: i32,
    pub file: i32,
}

#[derive(Debug)]
pub struct Queen {
    pub position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank > 7 || file < 0 || file > 7 {
            None
        } else {
            Some(Self { rank, file })
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let self_pos = &self.position;
        let other_pos = &other.position;

        self_pos.rank == other_pos.rank
            || self_pos.file == other_pos.file
            || Queen::are_on_diagonal(self_pos, other_pos)
    }

    pub fn are_on_diagonal(self_pos: &ChessPosition, other_pos: &ChessPosition) -> bool {
        let rank_diff = (self_pos.rank - other_pos.rank).abs();
        let file_diff = (self_pos.file - other_pos.file).abs();

        rank_diff == file_diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_is_valid() {
        assert!(ChessPosition::new(2, 4).is_some());
        assert!(ChessPosition::new(-1, 2).is_none());
        assert!(ChessPosition::new(8, 2).is_none());
        assert!(ChessPosition::new(5, -1).is_none());
        assert!(ChessPosition::new(5, 8).is_none());
    }

    #[test]
    fn test_queen_valid_position() {
        Queen::new(ChessPosition::new(2, 4).unwrap());
    }

    #[test]
    fn test_can_not_attack() {
        let white_queen = Queen::new(ChessPosition::new(2, 4).unwrap());
        let black_queen = Queen::new(ChessPosition::new(6, 6).unwrap());
        let white_queen2 = Queen::new(ChessPosition::new(1, 2).unwrap());
        let black_queen2 = Queen::new(ChessPosition::new(0, 4).unwrap());
        let white_queen3 = Queen::new(ChessPosition::new(5, 3).unwrap());
        let black_queen3 = Queen::new(ChessPosition::new(0, 6).unwrap());
        let white_queen4 = Queen::new(ChessPosition::new(3, 7).unwrap());
        let black_queen4 = Queen::new(ChessPosition::new(2, 0).unwrap());

        assert!(!white_queen.can_attack(&black_queen));
        assert!(!white_queen2.can_attack(&black_queen2));
        assert!(!white_queen3.can_attack(&black_queen3));
        assert!(!white_queen4.can_attack(&black_queen4));
    }

    #[test]
    fn test_same_rank() {
        let white_queen = Queen::new(ChessPosition::new(2, 4).unwrap());
        let black_queen = Queen::new(ChessPosition::new(2, 6).unwrap());
        let white_queen2 = Queen::new(ChessPosition::new(1, 2).unwrap());
        let black_queen2 = Queen::new(ChessPosition::new(1, 6).unwrap());
        let white_queen3 = Queen::new(ChessPosition::new(4, 7).unwrap());
        let black_queen3 = Queen::new(ChessPosition::new(4, 3).unwrap());
        let white_queen4 = Queen::new(ChessPosition::new(5, 3).unwrap());
        let black_queen4 = Queen::new(ChessPosition::new(5, 1).unwrap());

        assert!(white_queen.can_attack(&black_queen));
        assert!(white_queen2.can_attack(&black_queen2));
        assert!(white_queen3.can_attack(&black_queen3));
        assert!(white_queen4.can_attack(&black_queen4));
    }

    #[test]
    fn test_same_file() {
        let white_queen = Queen::new(ChessPosition::new(4, 5).unwrap());
        let black_queen = Queen::new(ChessPosition::new(3, 5).unwrap());
        let white_queen2 = Queen::new(ChessPosition::new(2, 2).unwrap());
        let black_queen2 = Queen::new(ChessPosition::new(3, 2).unwrap());
        let white_queen3 = Queen::new(ChessPosition::new(2, 6).unwrap());
        let black_queen3 = Queen::new(ChessPosition::new(1, 6).unwrap());
        let white_queen4 = Queen::new(ChessPosition::new(2, 7).unwrap());
        let black_queen4 = Queen::new(ChessPosition::new(5, 7).unwrap());

        assert!(white_queen.can_attack(&black_queen));
        assert!(white_queen2.can_attack(&black_queen2));
        assert!(white_queen3.can_attack(&black_queen3));
        assert!(white_queen4.can_attack(&black_queen4));
    }

    #[test]
    fn test_same_diagonal() {
        let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());
        let black_queen = Queen::new(ChessPosition::new(0, 4).unwrap());
        let white_queen2 = Queen::new(ChessPosition::new(2, 2).unwrap());
        let black_queen2 = Queen::new(ChessPosition::new(3, 1).unwrap());
        let white_queen3 = Queen::new(ChessPosition::new(2, 2).unwrap());
        let black_queen3 = Queen::new(ChessPosition::new(1, 1).unwrap());
        let white_queen4 = Queen::new(ChessPosition::new(2, 2).unwrap());
        let black_queen4 = Queen::new(ChessPosition::new(5, 5).unwrap());

        assert!(white_queen.can_attack(&black_queen));
        assert!(white_queen2.can_attack(&black_queen2));
        assert!(white_queen3.can_attack(&black_queen3));
        assert!(white_queen4.can_attack(&black_queen4));
    }
}
