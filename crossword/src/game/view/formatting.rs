use std::fmt::Display;

use crate::game::model::BoardCell;

impl Display for BoardCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardCell::Filled(ch) => write!(f, " {} ", ch.to_uppercase()),
            BoardCell::Empty => write!(f, "   "),
            BoardCell::Blocked => write!(f, "███"),
        }
    }
}
