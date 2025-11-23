use crate::model::{BoardDirection, MoveDirection, PuzzleCell, Word};

use super::{Coordinate, Grid};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
struct PuzzleFile {
    size: usize,
    grid: String,
    across: Vec<String>,
    down: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct PuzzleWords {
    pub across: Vec<Word>,
    pub down: Vec<Word>,
}

#[derive(Clone, Debug)]
pub struct Puzzle {
    pub grid: Grid<PuzzleCell>,
    pub words: PuzzleWords,
    pub clue_numbers: Vec<(Coordinate, usize)>,
}

impl Puzzle {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(path)?;
        let puzzle_file: PuzzleFile = toml::from_str(&contents)?;
        let mut grid_vec = Vec::new();

        for line in puzzle_file.grid.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                let row: Vec<PuzzleCell> = trimmed
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .map(|ch| match ch {
                        '.' => None, // Use '.' for blocked cells in file
                        c => Some(c),
                    })
                    .map(|opt_char| match opt_char {
                        Some(c) => PuzzleCell::Fillable(c),
                        None => PuzzleCell::Blocked,
                    })
                    .collect();
                grid_vec.push(row);
            }
        }

        let grid = Grid::from_vec(grid_vec, puzzle_file.size);
        let words = Self::find_words(&grid, puzzle_file.across, puzzle_file.down);

        let mut across = Vec::new();
        let mut down = Vec::new();
        let mut clue_numbers = Vec::new();

        for word in words {
            clue_numbers.push((word.start_pos, word.clue_number));

            match word.direction {
                BoardDirection::Across => across.push(word),
                BoardDirection::Down => down.push(word),
            }
        }

        clue_numbers.sort_by(|a, b| a.1.cmp(&b.1));
        clue_numbers.dedup_by(|a, b| a.1 == b.1);

        Ok(Puzzle {
            grid,
            words: PuzzleWords { across, down },
            clue_numbers,
        })
    }

    pub fn size(&self) -> usize {
        self.grid.size()
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
        matches!(self.get(coord), PuzzleCell::Blocked) && coord.is_valid(self.size(), self.size())
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

    fn find_words(
        grid: &Grid<PuzzleCell>,
        across_clues: Vec<String>,
        down_clues: Vec<String>,
    ) -> Vec<Word> {
        let mut words = Vec::new();
        let mut across_iter = across_clues.iter();
        let mut down_iter = down_clues.iter();
        let mut prev_clue_number = 0;

        for (start_pos, _) in grid.iter() {
            let clue_number = prev_clue_number + 1;
            if Self::is_across_word_start(&grid, &start_pos)
                && let Some(end_pos) = Self::find_across_word_end(&grid, &start_pos)
            {
                let clue = across_iter
                    .next()
                    .expect("Not enough across clues")
                    .to_owned();

                let text = grid
                    .directional_iter(start_pos, MoveDirection::Right)
                    .take_while(|(coord, _)| coord <= &end_pos)
                    .map(|(_, cell)| match cell {
                        PuzzleCell::Fillable(ch) => ch,
                        PuzzleCell::Blocked => ' ', // Should not happen in a valid word
                    })
                    .collect::<String>();

                let word = Word::new(
                    text,
                    clue_number,
                    BoardDirection::Across,
                    start_pos,
                    end_pos,
                    clue,
                );
                words.push(word);
                prev_clue_number = clue_number;
            }

            if Self::is_down_word_start(&grid, &start_pos)
                && let Some(end_pos) = Self::find_down_word_end(&grid, &start_pos)
            {
                let clue = down_iter.next().expect("Not enough down clues").to_owned();

                let text = grid
                    .directional_iter(start_pos, MoveDirection::Right)
                    .take_while(|(coord, _)| coord <= &end_pos)
                    .map(|(_, cell)| match cell {
                        PuzzleCell::Fillable(ch) => ch,
                        PuzzleCell::Blocked => ' ', // Should not happen in a valid word
                    })
                    .collect();

                let word = Word::new(
                    text,
                    clue_number,
                    BoardDirection::Down,
                    start_pos,
                    end_pos,
                    clue,
                );

                words.push(word);
                prev_clue_number = clue_number;
            }
        }
        words
    }

    fn is_across_word_start(grid: &Grid<PuzzleCell>, coord: &Coordinate) -> bool {
        let coord = *coord;
        let left = coord.left();
        let right = coord.right();
        // Helper to get char from grid
        let get = |c: Coordinate| {
            grid.get(c).and_then(|cell| match cell {
                PuzzleCell::Fillable(ch) => Some(ch),
                PuzzleCell::Blocked => None,
            })
        };

        // Current cell is fillable, left is blocked/edge, right is fillable
        get(coord).is_some() && get(left).is_none() && get(right).is_some()
    }

    fn is_down_word_start(grid: &Grid<PuzzleCell>, coord: &Coordinate) -> bool {
        let coord = *coord;
        let up = coord.up();
        let down = coord.down();
        // Helper to get char from grid
        let get = |c: Coordinate| {
            grid.get(c).and_then(|cell| match cell {
                PuzzleCell::Fillable(ch) => Some(ch),
                PuzzleCell::Blocked => None,
            })
        };

        // Current cell is fillable, up is blocked/edge, down is fillable
        get(coord).is_some() && get(up).is_none() && get(down).is_some()
    }

    fn find_across_word_end(grid: &Grid<PuzzleCell>, start: &Coordinate) -> Option<Coordinate> {
        if !Self::is_across_word_start(grid, start) {
            return None;
        }
        let get = |c: Coordinate| {
            grid.get(c).and_then(|cell| match cell {
                PuzzleCell::Fillable(ch) => Some(ch),
                PuzzleCell::Blocked => None,
            })
        };

        let mut end = *start;
        while get(end.right()).is_some() {
            end = end.right();
        }
        Some(end)
    }

    fn find_down_word_end(grid: &Grid<PuzzleCell>, start: &Coordinate) -> Option<Coordinate> {
        if !Self::is_down_word_start(grid, start) {
            return None;
        }
        let get = |c: Coordinate| {
            grid.get(c).and_then(|cell| match cell {
                PuzzleCell::Fillable(ch) => Some(ch),
                PuzzleCell::Blocked => None,
            })
        };

        let mut end = *start;
        while get(end.down()).is_some() {
            end = end.down();
        }
        Some(end)
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
