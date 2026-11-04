use std::time::{Duration, Instant};

use crate::{
    debug_log,
    game::model::{
        BoardCell, BoardDirection, Coordinate, Grid, LoopIter, MoveDirection, Puzzle, PuzzleCell,
        Word, WordIter,
    },
};

pub struct PlayerBoard {
    puzzle: Puzzle,
    size: usize,
    grid: Grid<BoardCell>,
    cursor: Coordinate,
    direction: BoardDirection,
    empty_cells: usize,
    start_time: Instant,
    end_time: Option<Instant>,
}

impl PlayerBoard {
    pub fn from_puzzle(puzzle: Puzzle) -> Self {
        let width = puzzle.width();
        let height = puzzle.height();

        let grid = puzzle
            .grid
            .vec()
            .iter()
            .map(|row| row.iter().map(|&cell| cell.into()).collect())
            .collect();

        let empty_cells = puzzle
            .grid
            .vec()
            .iter()
            .flatten()
            .filter(|&&cell| matches!(cell, PuzzleCell::Fillable(_)))
            .count();

        let start_time = Instant::now();

        let mut board = PlayerBoard {
            puzzle: puzzle,
            size: width.max(height), // For backward compatibility; prefer width/height
            grid: Grid::from_vec(grid, width, height),
            cursor: Coordinate::new(0, 0),
            direction: BoardDirection::Across,
            empty_cells,
            start_time,
            end_time: None,
        };

        if !board.cell_playable(board.cursor) {
            board.move_to_next_empty_cell();
        }

        board
    }

    pub fn get(&self, coord: Coordinate) -> BoardCell {
        self.grid.get(coord).unwrap_or(BoardCell::Blocked)
    }

    pub fn get_clue_number_at(&self, coord: Coordinate) -> Option<usize> {
        self.puzzle.get_clue_number_at(coord)
    }

    pub fn get_current(&self) -> BoardCell {
        self.get(self.cursor)
    }

    pub fn get_cursor(&self) -> Coordinate {
        self.cursor
    }

    pub fn get_direction(&self) -> BoardDirection {
        self.direction
    }

    pub fn get_elapsed_time(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn get_total_time(&self) -> Option<Duration> {
        self.end_time.map(|end| end.duration_since(self.start_time))
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn width(&self) -> usize {
        self.grid.width()
    }

    pub fn height(&self) -> usize {
        self.grid.height()
    }

    pub fn get_words_in_current_direction(&self) -> Vec<&Word> {
        self.puzzle.words_in_direction(self.direction)
    }

    pub fn get_current_word(&self) -> Option<&Word> {
        self.puzzle.get_word_at(self.cursor, self.direction)
    }

    pub fn get_current_word_index(&self) -> Option<usize> {
        let words = self.get_words_in_current_direction();
        self.get_current_word().and_then(|current_word| {
            words
                .iter()
                .position(|word| word.start_pos == current_word.start_pos)
        })
    }

    pub fn current_word_iter(&self) -> Option<WordIter> {
        self.get_current_word()
            .map(|word| word.word_iter().start_at(self.cursor))
    }

    pub fn is_current_word_completed(&self) -> bool {
        if let Some(word) = self.get_current_word() {
            word.cell_iter()
                .all(|coord| matches!(self.get(coord), BoardCell::Filled(_)))
        } else {
            false
        }
    }

    pub fn swap_direction(&mut self) {
        self.direction = self.direction.swap();
    }

    pub fn move_cursor(&mut self, move_direction: MoveDirection) {
        let new_pos = self
            .grid
            .directional_iter(self.cursor, move_direction, true)
            .skip(1)
            .find(|(_, cell)| !cell.is_blocked())
            .map(|(coord, _)| coord)
            .expect("should have moved");

        self.cursor = new_pos;
    }

    pub fn move_to_next_empty_cell(&mut self) {
        let move_direction = self.direction.into();
        self.move_to_cell_that(|cell| cell == BoardCell::Empty, move_direction);
    }

    pub fn move_to_next_open_word(&mut self) {
        let word_info = self
            .words_iter()
            .skip(1)
            .find_map(|word| {
                word.word_iter()
                    .find(|coord| self.get(*coord) == BoardCell::Empty)
                    .map(|coord| (coord, word.direction))
            })
            .or_else(|| {
                self.words_iter()
                    .nth(1)
                    .and_then(|word| Some((word.start_pos, word.direction)))
            });

        if let Some((coord, direction)) = word_info {
            self.move_to(coord)
                .expect("move_to_next_open_word outside bounds");
            self.direction = direction;
            return;
        }
    }

    pub(crate) fn move_to_previous_open_word(&mut self) {
        let word_info = self
            .words_iter()
            .rev()
            .find_map(|word| {
                word.word_iter()
                    .find(|coord| self.get(*coord).is_empty())
                    .map(|coord| (coord, word.direction))
            })
            .or_else(|| {
                self.words_iter()
                    .nth(1)
                    .and_then(|word| Some((word.start_pos, word.direction)))
            });

        debug_log::debug_log(format!(
            "move_to_previous_open_word found word_info: {:?}",
            word_info
        ));

        if let Some((coord, direction)) = word_info {
            self.move_to(coord)
                .expect("move_to_next_open_word outside bounds");
            self.direction = direction;
        }
    }

    pub fn move_to_previous_cell(&mut self) {
        if let Some(word) = self.get_current_word()
            && self.cursor == word.start_pos
        {
            let word_info = self
                .words_iter()
                .rev()
                .find_map(|word| word.word_iter().last().map(|coord| (coord, word.direction)));

            debug_log::debug_log(format!(
                "move_to_previous_cell found word_info: {:?}",
                word_info
            ));

            if let Some((coord, direction)) = word_info {
                self.move_to(coord)
                    .expect("move_to_previous_cell outside bounds");
                self.direction = direction;
            }
        } else {
            let move_dir: MoveDirection = self.direction.into();
            self.move_cursor(move_dir.reverse())
        }
    }

    pub fn write_to_cell(&mut self, c: char) {
        assert!(
            c.is_alphanumeric(),
            "Only alphanumeric characters can be entered"
        );
        assert!(
            self.cell_playable(self.cursor),
            "Cannot fill a blocked cell at {:?}",
            self.cursor
        );

        let current_cell = self.get(self.cursor);

        self.grid.set(self.cursor, BoardCell::Filled(c));
        if current_cell == BoardCell::Empty {
            self.empty_cells = self.empty_cells.saturating_sub(1);
        }
    }

    pub fn clear_cell(&mut self) {
        let current_cell = self.get(self.cursor);
        self.grid.set(self.cursor, BoardCell::Empty);
        if current_cell != BoardCell::Empty {
            self.empty_cells = self.empty_cells.saturating_add(1);
        }
    }

    pub fn cell_playable(&self, coord: Coordinate) -> bool {
        self.get(coord) != BoardCell::Blocked
    }

    pub fn cell_empty(&self, coord: Coordinate) -> bool {
        self.get(coord).is_empty()
    }

    pub fn has_won(&self) -> bool {
        self.grid.coord_iter().all(|coord| self.cell_correct(coord))
    }

    pub fn mark_as_won(&mut self) {
        if self.end_time.is_none() {
            self.end_time = Some(Instant::now());
        }
    }

    pub fn move_to(&mut self, coord: Coordinate) -> Result<(), String> {
        if self.cell_playable(coord) {
            self.cursor = coord;
            Ok(())
        } else {
            Err(format!("Cannot move to blocked cell at {:?}", coord))
        }
    }

    pub fn words_iter(&self) -> LoopIter<Word> {
        let start_index = self.get_current_word_index().unwrap_or(0)
            + match self.direction {
                BoardDirection::Across => 0,
                BoardDirection::Down => self.puzzle.across_words().len(),
            };

        let words = self.puzzle.all_words().to_owned();

        LoopIter::new(words, start_index)
    }

    fn move_to_cell_that<F>(&mut self, condition: F, move_direction: MoveDirection)
    where
        F: Fn(BoardCell) -> bool,
    {
        let new_pos = self
            .grid
            .directional_iter(self.cursor, move_direction, true)
            .find(|(_, cell)| condition(*cell))
            .map(|(coord, _)| coord)
            .unwrap_or(self.cursor);

        self.cursor = new_pos;
    }

    fn cell_correct(&self, coord: Coordinate) -> bool {
        let player_cell = self.get(coord);
        let puzzle_cell = self.puzzle.get(coord);

        match (player_cell, puzzle_cell) {
            (BoardCell::Filled(c), PuzzleCell::Fillable(correct_char)) => {
                c.to_ascii_lowercase() == correct_char.to_ascii_lowercase()
            }
            (BoardCell::Blocked, PuzzleCell::Blocked) => true,
            (BoardCell::Empty, PuzzleCell::Fillable(_)) => false,
            _ => false,
        }
    }

    pub(crate) fn move_forward(&mut self) {
        if self
            .grid
            .get(self.cursor.move_direction(self.direction.into()))
            .map(|cell| !cell.is_blocked())
            .unwrap_or(false)
        {
            self.move_cursor(self.direction.into());
        } else {
            let word = self
                .words_iter()
                .skip(1)
                .find(|word| word.word_iter().any(|coord| self.get(coord).is_empty()));

            // let coord =
            //     &word.and_then(|word| word.word_iter().find(|coord| self.get(*coord).is_empty()));

            // if let Some(coord) = coord {
            //     self.direction = word.expect("word should exist if coord exists").direction;
            //     self.move_to(*coord).expect("move_forward outside bounds");
            // }

            if let Some(word) = word
                && let Some(coord) = word.word_iter().find(|coord| self.get(*coord).is_empty())
            {
                self.direction = word.direction;
                self.move_to(coord).expect("move_forward outside bounds");
            }
        }
    }
}

#[cfg(test)]
#[path = "player_board_tests.rs"]
mod player_board_tests;
