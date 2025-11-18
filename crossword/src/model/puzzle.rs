use crate::model::Word;

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
pub struct Puzzle {
    pub grid: Grid<Option<char>>,
    pub across_words: Vec<Word>,
    pub down_words: Vec<Word>,
}

impl Puzzle {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(path)?;
        let puzzle_file: PuzzleFile = toml::from_str(&contents)?;

        let mut grid_vec = Vec::new();
        for line in puzzle_file.grid.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                let row: Vec<Option<char>> = trimmed
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .map(|ch| match ch {
                        '.' => None, // Use '.' for blocked cells in file
                        c => Some(c),
                    })
                    .collect();
                grid_vec.push(row);
            }
        }

        let grid = Grid::from_vec(grid_vec, puzzle_file.size);
        let (across_words, down_words) =
            Self::find_words(&grid, puzzle_file.across, puzzle_file.down);

        Ok(Puzzle {
            grid,
            across_words,
            down_words,
        })
    }

    pub fn new(size: usize) -> Self {
        Puzzle {
            grid: Grid::new(size, None),
            across_words: Vec::new(),
            down_words: Vec::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.grid.size()
    }

    pub fn get(&self, coord: Coordinate) -> Option<char> {
        self.grid.get(coord).and_then(|&cell| cell)
    }

    pub fn get_answer(&self, coord: Coordinate) -> Option<char> {
        self.get(coord)
    }

    pub fn is_blocked(&self, coord: Coordinate) -> bool {
        matches!(self.get(coord), None) && coord.is_valid(self.size(), self.size())
    }

    pub fn is_fillable(&self, coord: Coordinate) -> bool {
        self.get(coord).is_some()
    }

    fn find_words(
        grid: &Grid<Option<char>>,
        across_clues: Vec<String>,
        down_clues: Vec<String>,
    ) -> (Vec<Word>, Vec<Word>) {
        let mut across_words = Vec::new();
        let mut down_words = Vec::new();
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

                across_words.push(Word::new(clue_number, start_pos, end_pos, clue));
                prev_clue_number = clue_number;
            }

            if Self::is_down_word_start(&grid, &start_pos)
                && let Some(end_pos) = Self::find_down_word_end(&grid, &start_pos)
            {
                let clue = down_iter.next().expect("Not enough down clues").to_owned();

                down_words.push(Word::new(clue_number, start_pos, end_pos, clue));
                prev_clue_number = clue_number;
            }
        }
        (across_words, down_words)
    }

    fn is_across_word_start(grid: &Grid<Option<char>>, coord: &Coordinate) -> bool {
        let coord = *coord;
        let left = coord.left();
        let right = coord.right();
        // Helper to get char from grid
        let get = |c: Coordinate| grid.get(c).and_then(|&cell| cell);

        // Current cell is fillable, left is blocked/edge, right is fillable
        get(coord).is_some() && get(left).is_none() && get(right).is_some()
    }

    fn is_down_word_start(grid: &Grid<Option<char>>, coord: &Coordinate) -> bool {
        let coord = *coord;
        let up = coord.up();
        let down = coord.down();
        // Helper to get char from grid
        let get = |c: Coordinate| grid.get(c).and_then(|&cell| cell);

        // Current cell is fillable, up is blocked/edge, down is fillable
        get(coord).is_some() && get(up).is_none() && get(down).is_some()
    }

    fn find_across_word_end(grid: &Grid<Option<char>>, start: &Coordinate) -> Option<Coordinate> {
        if !Self::is_across_word_start(grid, start) {
            return None;
        }
        let get = |c: Coordinate| grid.get(c).and_then(|&cell| cell);

        let mut end = *start;
        while get(end.right()).is_some() {
            end = end.right();
        }
        Some(end)
    }

    fn find_down_word_end(grid: &Grid<Option<char>>, start: &Coordinate) -> Option<Coordinate> {
        if !Self::is_down_word_start(grid, start) {
            return None;
        }
        let get = |c: Coordinate| grid.get(c).and_then(|&cell| cell);

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
                    Some(Some(ch)) => write!(f, "{}", ch)?,
                    Some(None) => write!(f, "█")?,
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
        if !self.across_words.is_empty() {
            writeln!(f, "\nAcross:")?;
            for word in &self.across_words {
                writeln!(f, "  {}", word)?;
            }
        }

        // Down clues
        if !self.down_words.is_empty() {
            writeln!(f, "\nDown:")?;
            for word in &self.down_words {
                writeln!(f, "  {}", word)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "puzzle_tests.rs"]
mod puzzle_tests;
