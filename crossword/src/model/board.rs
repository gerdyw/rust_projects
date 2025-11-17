use std::fmt::Display;

use crate::model::{
    common::{BoardDirection, Coordinate, Grid, MoveDirection},
    puzzle::Puzzle,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Filled(char),
    Empty,
    Blocked,
}

#[derive(Clone)]
pub struct Board {
    pub size: usize,
    pub grid: Grid<Tile>,
    pub pos: Coordinate,
    pub direction: BoardDirection,
}

impl Board {
    pub fn from_puzzle(puzzle: &Puzzle) -> Self {
        let size = puzzle.size;

        let grid = puzzle
            .grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&cell| match cell {
                        Some(_) => Tile::Empty,
                        None => Tile::Blocked,
                    })
                    .collect()
            })
            .collect();

        Board {
            size,
            grid: Grid::from_vec(grid),
            pos: Coordinate::new(0, 0, size),
            direction: BoardDirection::default(),
        }
    }

    pub fn move_cursor(self, direction: MoveDirection) -> Self {
        let mut new_pos = self.pos;
        loop {
            new_pos = new_pos.move_direction(direction);
            if *self.grid.get(new_pos) != Tile::Blocked {
                break;
            }
        }

        Board {
            pos: new_pos,
            grid: self.grid,
            size: self.size,
            direction: self.direction,
        }
    }

    pub fn move_backward(self) -> Self {
        match self.direction {
            BoardDirection::Across => self.move_cursor(MoveDirection::Left),
            BoardDirection::Down => self.move_cursor(MoveDirection::Up),
        }
    }

    pub fn swap_direction(self) -> Self {
        Board {
            direction: self.direction.swap(),
            grid: self.grid,
            size: self.size,
            pos: self.pos,
        }
    }

    pub fn enter_char(self, ch: char) -> Self {
        Board {
            grid: self.grid.set(self.pos, Tile::Filled(ch)),
            size: self.size,
            pos: self.pos,
            direction: self.direction,
        }
    }

    pub fn delete_char(self) -> Self {
        Board {
            grid: self.grid.set(self.pos, Tile::Empty),
            size: self.size,
            pos: self.pos,
            direction: self.direction,
        }
    }
}

impl Display for Board {
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

        for (row_idx, row) in self.grid.vec().iter().enumerate() {
            write!(f, "│")?;
            for (col_idx, &tile) in row.iter().enumerate() {
                match tile {
                    Tile::Filled(ch) => write!(f, "{}", ch)?,
                    Tile::Empty => write!(f, " ")?,
                    Tile::Blocked => write!(f, "█")?,
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

        // Bottom border
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

#[cfg(test)]
#[path = "board_tests.rs"]
mod board_tests;
