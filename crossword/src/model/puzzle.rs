use crate::model::{BoardDirection, MoveDirection, PuzzleCell, Word};

use super::{Coordinate, Grid};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{error::Error, fmt::Display, fs};

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
    pub fn new(
        grid: Grid<PuzzleCell>,
        words: PuzzleWords,
        clue_numbers: Vec<(Coordinate, usize)>,
    ) -> Self {
        Puzzle {
            grid,
            words,
            clue_numbers,
        }
    }

    /// Parse an IPUZ JSON string and return a `Puzzle` ready to use by the app.
    pub fn parse_ipuz_to_puzzle(input: &str) -> Result<Puzzle, Box<dyn Error>> {
        let v: Value = serde_json::from_str(input)?;

        // get dimensions
        let width = v
            .get("dimensions")
            .and_then(|d| d.get("width"))
            .and_then(|w| w.as_u64())
            .map(|n| n as usize)
            .or_else(|| v.get("width").and_then(|w| w.as_u64()).map(|n| n as usize))
            .ok_or("missing width in IPUZ")?;

        // block character (default '#')
        let block_char = v
            .get("block")
            .and_then(|b| b.as_str())
            .and_then(|s| s.chars().next())
            .unwrap_or('#');

        // choose source for letters: prefer "solution", then "puzzle"
        let grid_source = v
            .get("solution")
            .or_else(|| v.get("puzzle"))
            .ok_or("no 'solution' or 'puzzle' array found in IPUZ")?;

        // Build Vec<Vec<PuzzleCell>>
        let mut rows: Vec<Vec<PuzzleCell>> = Vec::new();

        if let Some(arr) = grid_source.as_array() {
            for row_val in arr {
                // Each row might be an array of strings/objects or a string.
                if let Some(row_arr) = row_val.as_array() {
                    let mut row_cells = Vec::with_capacity(width);
                    for cell_val in row_arr {
                        let cell = if let Some(s) = cell_val.as_str() {
                            // string element like "A" or "#"
                            let ch = s.chars().next().unwrap_or(' ');
                            if ch == block_char {
                                PuzzleCell::Blocked
                            } else {
                                PuzzleCell::Fillable(ch)
                            }
                        } else if let Some(_) = cell_val.as_object() {
                            // object variant: prefer a "solution" string, otherwise treat as blocked
                            if let Some(sol) = cell_val.get("solution").and_then(|x| x.as_str()) {
                                let ch = sol.chars().next().unwrap_or(' ');
                                if ch == block_char {
                                    PuzzleCell::Blocked
                                } else {
                                    PuzzleCell::Fillable(ch)
                                }
                            } else {
                                // no explicit solution -> treat as blocked (safer than inventing letters)
                                PuzzleCell::Blocked
                            }
                        } else {
                            // unexpected cell format -> blocked
                            PuzzleCell::Blocked
                        };
                        row_cells.push(cell);
                    }
                    rows.push(row_cells);
                } else if let Some(s) = row_val.as_str() {
                    // a single string row, interpret characters and spaces; ignore whitespace
                    let mut row_cells = Vec::with_capacity(width);
                    for ch in s.chars().filter(|c| !c.is_whitespace()) {
                        if ch == block_char {
                            row_cells.push(PuzzleCell::Blocked);
                        } else {
                            row_cells.push(PuzzleCell::Fillable(ch));
                        }
                    }
                    // pad/truncate to width
                    row_cells.resize_with(width, || PuzzleCell::Blocked);
                    rows.push(row_cells);
                } else {
                    return Err("unsupported row format in IPUZ".into());
                }
            }
        } else {
            return Err("expected solution/puzzle to be an array of rows".into());
        }

        // convert rows into Grid<PuzzleCell>
        let grid = Grid::from_vec(rows, width as usize);

        // Extract clues: IPUZ example stores clues under "clues" -> "Across"/"Down" as arrays of [number, text]
        let extract = |dir: &str| -> Vec<String> {
            v.get("clues")
                .and_then(|c| c.get(dir))
                .and_then(|a| a.as_array())
                .map(|arr| {
                    arr.iter()
                        .map(|item| {
                            if let Some(s) = item
                                .as_array()
                                .and_then(|a| a.get(1))
                                .and_then(|x| x.as_str())
                            {
                                s.to_string()
                            } else if let Some(s) = item.as_str() {
                                s.to_string()
                            } else {
                                item.to_string()
                            }
                        })
                        .collect()
                })
                .unwrap_or_default()
        };

        let across = extract("Across");
        let down = extract("Down");

        // Build words & clue numbers using existing helper
        let all_words = Self::find_words(&grid, across.clone(), down.clone());

        let mut across_words = Vec::new();
        let mut down_words = Vec::new();
        let mut clue_numbers = Vec::new();

        for w in all_words {
            clue_numbers.push((w.start_pos, w.clue_number));
            match w.direction {
                BoardDirection::Across => across_words.push(w),
                BoardDirection::Down => down_words.push(w),
            }
        }

        clue_numbers.sort_by(|a, b| a.1.cmp(&b.1));
        clue_numbers.dedup_by(|a, b| a.1 == b.1);

        Ok(Puzzle::new(
            grid,
            PuzzleWords {
                across: across_words,
                down: down_words,
            },
            clue_numbers,
        ))
    }

    pub fn from_toml_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;
        Self::from_toml_string(contents)
    }

    pub fn from_toml_string(puzzle_data: String) -> Result<Self, Box<dyn Error>> {
        let puzzle_file: PuzzleFile = toml::from_str(&puzzle_data)?;
        let mut grid_vec = Vec::new();

        for line in puzzle_file.grid.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                let row: Vec<PuzzleCell> = trimmed
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .map(|ch| match ch {
                        '.' => PuzzleCell::Blocked,
                        c => PuzzleCell::Fillable(c),
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
