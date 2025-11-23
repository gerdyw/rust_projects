use crate::model::MoveDirection;

pub enum GameCommand {
    MoveInDirection(MoveDirection),
    MoveForward,
    SwapDirection,
    MoveToNextEmptyCell,
    MoveToNextWord,
    EnterChar(char),
    DeleteChar,
    Quit,
}
