use std::fmt::Display;

use crate::model::{
    common::{BoardDirection, Coordinate, Grid, MoveDirection},
    puzzle::Puzzle,
};

use super::common::Edge;

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
            grid: Grid::from_vec(grid, size),
            pos: Coordinate::new(0, 0),
            direction: BoardDirection::default(),
        }
    }

    pub fn is_start_of_word(&self) -> bool {
        match self.direction {
            BoardDirection::Across => {
                // Check if at the start of a row or the previous tile is blocked
                // Also ensure the next tile exists and is not blocked (no single-letter words)
                let on_left = self.pos.is_on_edge(Edge::Left);
                let left_blocked = self
                    .grid
                    .is_val(self.pos.move_direction_wrapped(MoveDirection::Left), Tile::Blocked);
                let next_exists = self.grid.is_on_edge(coord, edge)
            }
            BoardDirection::Down => {
                // Check if at the top of a column or the tile above is blocked
                // Also ensure the tile below exists and is not blocked (no single-letter words)
                let at_start = self.pos.row == 0
                    || *self.grid.get(Coordinate::new(
                        self.pos.col,
                        self.pos.row - 1,
                        self.pos.size,
                    )) == Tile::Blocked;

                let next_exists = self.pos.row + 1 < self.size
                    && *self.grid.get(Coordinate::new(
                        self.pos.col,
                        self.pos.row + 1,
                        self.pos.size,
                    )) != Tile::Blocked;

                at_start && next_exists
            }
        }
    }

    pub fn is_end_of_word(&self) -> bool {
        match self.direction {
            BoardDirection::Across => {
                // Check if at the end of a row or the next tile is blocked
                if self.pos.col + 1 >= self.size {
                    return true;
                }
                let right_coord = Coordinate::new(self.pos.col + 1, self.pos.row, self.pos.size);
                *self.grid.get(right_coord) == Tile::Blocked
            }
            BoardDirection::Down => {
                // Check if at the bottom of a column or the tile below is blocked
                if self.pos.row + 1 >= self.size {
                    return true;
                }
                let below_coord = Coordinate::new(self.pos.col, self.pos.row + 1, self.pos.size);
                *self.grid.get(below_coord) == Tile::Blocked
            }
        }
    }

    pub fn move_cursor(self, direction: MoveDirection) -> Self {
        let mut new_pos = self.pos;
        loop {
            new_pos = new_pos.move_direction_wrapped(direction);
            if *self.grid.get(new_pos) != Tile::Blocked {
                break;
            }
        }

        Board {
            pos: new_pos,
            ..self
        }
    }

    pub fn move_backward(self) -> Self {
        match self.direction {
            BoardDirection::Across => self.move_cursor(MoveDirection::Left),
            BoardDirection::Down => self.move_cursor(MoveDirection::Up),
        }
    }

    pub fn move_forward(self) -> Self {
        match self.direction {
            BoardDirection::Across => self.move_cursor(MoveDirection::Right),
            BoardDirection::Down => self.move_cursor(MoveDirection::Down),
        }
    }

    pub fn move_to_next_word_start(self) -> Self {
        let mut new_board = self;
        while !new_board.is_start_of_word() {
            new_board = new_board.move_forward();
        }
        new_board
    }

    pub fn swap_direction(self) -> Self {
        Board {
            direction: self.direction.swap(),
            ..self
        }
    }

    pub fn enter_char(self, ch: char) -> Self {
        Board {
            grid: self.grid.set(self.pos, Tile::Filled(ch)),
            ..self
        }
    }

    pub fn delete_char(self) -> Self {
        Board {
            grid: self.grid.set(self.pos, Tile::Empty),
            ..self
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
