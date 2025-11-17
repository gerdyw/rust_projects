use crate::model::common::MoveDirection;

pub enum Message {
    MoveCursor(MoveDirection),
    SwapDirection,
    EnterChar(char),
    DeleteChar,
    ResetPuzzle,
    Quit,
}
