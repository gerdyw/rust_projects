use crate::model::common::MoveDirection;

pub enum Message {
    MoveInDirection(MoveDirection),
    MoveForward,
    SwapDirection,
    MoveToNextWord,
    EnterChar(char),
    DeleteChar,
    ResetPuzzle,
    Quit,
}
