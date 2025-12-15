use crate::{
    debug_log,
    model::{BoardDirection, Coordinate, CoordinateIter, Word},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct WordIter {
    start_pos: Coordinate,
    current_pos: Coordinate,
    word_start: Coordinate,
    word_end: Coordinate,
    direction: BoardDirection,
    running: bool,
}

impl WordIter {
    pub fn new(word: &Word) -> Self {
        WordIter {
            start_pos: word.start_pos,
            current_pos: word.start_pos,
            word_start: word.start_pos,
            word_end: word.end_pos,
            direction: word.direction,
            running: true,
        }
    }

    pub fn start_at(&mut self, coord: Coordinate) -> Self {
        self.start_pos = coord;
        self.current_pos = coord;
        self.running = true;

        *self
    }

    pub fn from_start(&mut self) -> Self {
        self.current_pos = self.word_start;
        *self
    }
}

impl Iterator for WordIter {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        // wrap around word if at the end
        if !self.running {
            return None;
        }

        let result = self.current_pos;

        debug_log::debug_log(format!(
            "WordIter next: current_pos={:?}, word_end={:?}, start_pos={:?}",
            self.current_pos, self.word_end, self.start_pos
        ));

        self.current_pos = match self.direction {
            BoardDirection::Across => self.current_pos.right(),
            BoardDirection::Down => self.current_pos.down(),
        };

        if self.current_pos > self.word_end {
            self.current_pos = self.word_start;
        }

        if self.current_pos == self.start_pos {
            self.running = false;
        }

        Some(result)
    }
}
