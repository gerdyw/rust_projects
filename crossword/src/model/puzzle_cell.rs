#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PuzzleCell {
    Blocked,
    Fillable(char),
}
