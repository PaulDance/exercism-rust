#[derive(Debug, PartialEq)]
pub struct ChessPosition {
    rank: u8,
    file: u8,
}

impl ChessPosition {
    fn is_on_same_rank(&self, other: &Self) -> bool {
        self.rank == other.rank
    }

    fn is_on_same_file(&self, other: &Self) -> bool {
        self.file == other.file
    }

    fn is_on_same_diagonal(&self, other: &Self) -> bool {
        self.rank.max(other.rank) - self.rank.min(other.rank)
            == self.file.max(other.file) - self.file.min(other.file)
    }
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i8, file: i8) -> Option<Self> {
        if 0 <= rank && rank < 8 && 0 <= file && file < 8 {
            Some(Self {
                rank: rank as u8,
                file: file as u8,
            })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Self) -> bool {
        self.position.is_on_same_file(&other.position)
            || self.position.is_on_same_rank(&other.position)
            || self.position.is_on_same_diagonal(&other.position)
    }
}
