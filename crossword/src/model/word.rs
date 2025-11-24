use std::fmt::Display;

use crate::model::{BoardDirection, CoordinateIter, WordIter};

use super::Coordinate;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Word {
    pub text: String,
    pub clue_number: usize,
    pub direction: BoardDirection,
    pub start_pos: Coordinate,
    pub end_pos: Coordinate,
    pub clue: String,
}

impl Word {
    pub fn new(
        text: String,
        clue_number: usize,
        direction: BoardDirection,
        start_pos: Coordinate,
        end_pos: Coordinate,
        clue: String,
    ) -> Self {
        Word {
            text,
            clue_number,
            direction,
            start_pos,
            end_pos,
            clue,
        }
    }

    pub fn length(&self) -> usize {
        if self.is_across() {
            (self.end_pos.col - self.start_pos.col + 1) as usize
        } else {
            (self.end_pos.row - self.start_pos.row + 1) as usize
        }
    }

    pub fn is_across(&self) -> bool {
        self.start_pos.row == self.end_pos.row
    }

    pub fn is_down(&self) -> bool {
        self.start_pos.col == self.end_pos.col
    }

    pub fn contains(&self, coord: &Coordinate) -> bool {
        if self.is_across() {
            coord.row == self.start_pos.row
                && coord.col >= self.start_pos.col
                && coord.col <= self.end_pos.col
        } else {
            coord.col == self.start_pos.col
                && coord.row >= self.start_pos.row
                && coord.row <= self.end_pos.row
        }
    }

    pub fn cell_iter(&self) -> CoordinateIter {
        CoordinateIter::new(self.start_pos, self.end_pos)
    }

    pub fn word_iter(&self, start: Option<Coordinate>) -> WordIter {
        WordIter::new(self, start)
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}. ({}, {}) to ({}, {}): {}",
            self.clue_number,
            self.start_pos.col,
            self.start_pos.row,
            self.end_pos.col,
            self.end_pos.row,
            self.clue
        )
    }
}

#[cfg(test)]
#[path = "word_tests.rs"]
mod word_tests;
