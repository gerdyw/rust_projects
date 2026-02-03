use crate::game::model::{Coordinate, MoveDirection};

#[derive(Debug)]
pub enum CellCondition {
    IsEmpty,
    IsFilled,
    IsPlayable,
}

#[derive(Debug)]
pub enum MoveCondition {
    ToCellThat(CellCondition),
    ToNextWord,
    ToSameWordCellThat(CellCondition),
}

#[derive(Debug)]
pub enum GameCommand {
    MoveInDirection(MoveDirection),
    MoveTo(Coordinate),
    MoveForward,
    SwapDirection,
    MoveToNextEmptyCell,
    MoveToNextOpenWord,
    MoveToPreviousOpenWord,
    // MoveToNextPlayableCell,
    // MoveToCellThat(CellCondition),
    EnterChar(char),
    DeleteChar,
    Quit,
}
