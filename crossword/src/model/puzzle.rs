use crate::model::{BoardDirection, PuzzleCell, Word};

use super::{Coordinate, Grid};
use std::{cmp::Ordering, fmt::Display};

#[derive(Clone, Debug)]
pub struct PuzzleWords {
    pub across: Vec<Word>,
    pub down: Vec<Word>,
}

#[derive(Clone, Debug)]
pub struct Puzzle {
    pub grid: Grid<PuzzleCell>,
    words: PuzzleWords,
    all_words: Vec<Word>,
    pub clue_numbers: Vec<(Coordinate, usize)>,
}

impl Puzzle {
    pub fn new(
        grid: Grid<PuzzleCell>,
        words: PuzzleWords,
        clue_numbers: Vec<(Coordinate, usize)>,
    ) -> Self {
        let mut all_words = words.across.clone();

        all_words.extend(words.down.clone());
        // sort acrosses always before downs, then by clue number
        all_words.sort_by(|a, b| match (a.direction, b.direction) {
            (BoardDirection::Down, BoardDirection::Across) => Ordering::Greater,
            (BoardDirection::Across, BoardDirection::Down) => Ordering::Less,
            _ => a.clue_number.cmp(&b.clue_number),
        });

        Puzzle {
            grid,
            words,
            clue_numbers,
            all_words,
        }
    }

    pub fn size(&self) -> usize {
        self.grid.size()
    }

    pub fn width(&self) -> usize {
        self.grid.width()
    }

    pub fn height(&self) -> usize {
        self.grid.height()
    }

    pub fn get(&self, coord: Coordinate) -> PuzzleCell {
        self.grid.get(coord).unwrap_or(PuzzleCell::Blocked)
    }

    pub fn get_answer(&self, coord: Coordinate) -> Option<char> {
        match self.get(coord) {
            PuzzleCell::Fillable(ch) => Some(ch),
            PuzzleCell::Blocked => None,
        }
    }

    pub fn is_blocked(&self, coord: Coordinate) -> bool {
        matches!(self.get(coord), PuzzleCell::Blocked)
            && coord.is_valid(self.width(), self.height())
    }

    pub fn is_fillable(&self, coord: Coordinate) -> bool {
        matches!(self.get(coord), PuzzleCell::Fillable(_))
    }

    pub fn get_word_at(&self, coord: Coordinate, direction: BoardDirection) -> Option<&Word> {
        let words = match direction {
            BoardDirection::Across => &self.words.across,
            BoardDirection::Down => &self.words.down,
        };

        words.iter().find(|w| w.contains(&coord))
    }

    pub fn get_clue_number_at(&self, coord: Coordinate) -> Option<usize> {
        for (position, clue_number) in &self.clue_numbers {
            if *position == coord {
                return Some(*clue_number);
            }
        }
        None
    }

    pub fn all_words(&self) -> &Vec<Word> {
        &self.all_words
    }

    pub fn words_in_direction(&self, direction: BoardDirection) -> Vec<&Word> {
        match direction {
            BoardDirection::Across => self
                .all_words
                .iter()
                .take_while(|w| w.direction == BoardDirection::Across)
                .collect(),
            BoardDirection::Down => self
                .all_words
                .iter()
                .skip_while(|w| w.direction == BoardDirection::Across)
                .collect(),
        }
    }

    pub fn across_words(&self) -> Vec<&Word> {
        self.words_in_direction(BoardDirection::Across)
    }

    pub fn down_words(&self) -> Vec<&Word> {
        self.words_in_direction(BoardDirection::Down)
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = self.size() as usize;

        // Top border
        write!(f, "┌")?;
        for i in 0..size {
            write!(f, "─")?;
            if i < size - 1 {
                write!(f, "┬")?;
            }
        }
        writeln!(f, "┐")?;

        for row in 0..size {
            write!(f, "│")?;
            for col in 0..size {
                let coord = Coordinate::new(col as isize, row as isize);
                match self.grid.get(coord) {
                    Some(PuzzleCell::Fillable(ch)) => write!(f, "{}", ch)?,
                    Some(PuzzleCell::Blocked) => write!(f, "█")?,
                    None => write!(f, " ")?,
                }
                if col < size - 1 {
                    write!(f, "│")?;
                }
            }
            writeln!(f, "│")?;

            // Middle border
            if row < size - 1 {
                write!(f, "├")?;
                for i in 0..size {
                    write!(f, "─")?;
                    if i < size - 1 {
                        write!(f, "┼")?;
                    }
                }
                writeln!(f, "┤")?;
            }
        }

        // Bottom border
        write!(f, "└")?;
        for i in 0..size {
            write!(f, "─")?;
            if i < size - 1 {
                write!(f, "┴")?;
            }
        }
        writeln!(f, "┘")?;

        // Across clues
        if !self.words.across.is_empty() {
            writeln!(f, "\nAcross:")?;
            for word in &self.words.across {
                writeln!(f, "  {}", word)?;
            }
        }

        // Down clues
        if !self.words.down.is_empty() {
            writeln!(f, "\nDown:")?;
            for word in &self.words.down {
                writeln!(f, "  {}", word)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "puzzle_tests.rs"]
mod puzzle_tests;
