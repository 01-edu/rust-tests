#[derive(Debug, Clone, Copy)]
pub struct ChessPosition {
    pub rank: usize,
    pub file: usize,
}

impl ChessPosition {
    pub const fn new(rank: usize, file: usize) -> Option<Self> {
        if matches!(rank, 0..8) && matches!(file, 0..8) {
            Some(Self { rank, file })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Queen {
    pub position: ChessPosition,
}

impl Queen {
    pub const fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub const fn can_attack(self, other: Self) -> bool {
        self.can_see_rank(other) || self.can_see_file(other) || self.can_see_diagonal(other)
    }

    pub const fn can_see_rank(self, other: Self) -> bool {
        self.position.rank == other.position.rank
    }

    pub const fn can_see_file(self, other: Self) -> bool {
        self.position.file == other.position.file
    }

    pub const fn can_see_diagonal(self, other: Self) -> bool {
        let rank_diff = self.position.rank.abs_diff(other.position.rank);
        let file_diff = self.position.file.abs_diff(other.position.file);

        rank_diff == file_diff
    }
}
