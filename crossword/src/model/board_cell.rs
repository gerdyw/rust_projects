use crate::model::PuzzleCell;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoardCell {
    Filled(char),
    Empty,
    Blocked,
}

impl From<Option<char>> for BoardCell {
    fn from(opt: Option<char>) -> Self {
        match opt {
            Some(_) => BoardCell::Empty,
            None => BoardCell::Blocked,
        }
    }
}

impl From<PuzzleCell> for BoardCell {
    fn from(cell: PuzzleCell) -> Self {
        match cell {
            PuzzleCell::Fillable(_) => BoardCell::Empty,
            PuzzleCell::Blocked => BoardCell::Blocked,
        }
    }
}
