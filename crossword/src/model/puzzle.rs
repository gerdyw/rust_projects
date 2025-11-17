use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct PuzzleFile {
    size: usize,
    grid: String,
    across: Vec<String>,
    down: Vec<String>,
}

pub struct Puzzle {
    pub size: usize,
    pub grid: Vec<Vec<Option<char>>>,
    pub across_clues: Vec<String>,
    pub down_clues: Vec<String>,
}

impl Puzzle {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(path)?;
        let puzzle_file: PuzzleFile = toml::from_str(&contents)?;

        let mut grid = Vec::new();
        for line in puzzle_file.grid.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                let row: Vec<Option<char>> = trimmed
                    .chars()
                    .map(|ch| match ch {
                        ' ' => None,
                        c => Some(c),
                    })
                    .collect();
                grid.push(row);
            }
        }

        Ok(Puzzle {
            size: puzzle_file.size,
            grid,
            across_clues: puzzle_file.across,
            down_clues: puzzle_file.down,
        })
    }

    pub fn new(size: usize) -> Self {
        Puzzle {
            size,
            grid: vec![vec![None; size]; size],
            across_clues: Vec::new(),
            down_clues: Vec::new(),
        }
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Top border
        write!(f, "┌")?;
        for i in 0..self.size {
            write!(f, "─")?;
            if i < self.size - 1 {
                write!(f, "┬")?;
            }
        }
        writeln!(f, "┐")?;

        for (row_idx, row) in self.grid.iter().enumerate() {
            write!(f, "│")?;
            for (col_idx, &cell) in row.iter().enumerate() {
                match cell {
                    Some(ch) => write!(f, "{}", ch)?,
                    None => write!(f, "█")?,
                }
                if col_idx < self.size - 1 {
                    write!(f, "│")?;
                }
            }
            writeln!(f, "│")?;

            // Middle border
            if row_idx < self.size - 1 {
                write!(f, "├")?;
                for i in 0..self.size {
                    write!(f, "─")?;
                    if i < self.size - 1 {
                        write!(f, "┼")?;
                    }
                }
                writeln!(f, "┤")?;
            }
        }

        // Bottom borde
        write!(f, "└")?;
        for i in 0..self.size {
            write!(f, "─")?;
            if i < self.size - 1 {
                write!(f, "┴")?;
            }
        }
        writeln!(f, "┘")?;
        Ok(())
    }
}
