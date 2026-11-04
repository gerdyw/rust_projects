#[derive(Clone, Debug)]
pub struct LoopIter<T> {
    elements: Vec<T>,
    start_index: usize,
    current_index: usize,
    has_started: bool,
}

impl<T> LoopIter<T> {
    pub fn new(elements: Vec<T>, start_index: usize) -> Self {
        LoopIter {
            elements,
            start_index,
            current_index: start_index,
            has_started: false,
        }
    }

    pub fn start_at(&mut self, index: usize) {
        self.start_index = index;
        self.current_index = index;
        self.has_started = false;
    }

    pub fn move_start_by(&mut self, offset: isize) {
        let len = self.elements.len() as isize;
        let new_index = (self.start_index as isize + offset).rem_euclid(len);
        self.start_index = new_index as usize;
        self.current_index = self.start_index;
        self.has_started = false;
    }

    fn increment_index(&mut self) {
        self.current_index = (self.current_index + 1) % self.elements.len();
    }

    fn decrement_index(&mut self) {
        if self.current_index == 0 {
            self.current_index = self.elements.len() - 1;
        } else {
            self.current_index -= 1;
        }
    }
}

impl<T: Clone> Iterator for LoopIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.elements.is_empty() {
            return None;
        }

        // If we've looped back to start after having started, we're done
        if self.has_started && self.current_index == self.start_index {
            return None;
        }

        self.has_started = true;

        // Return current element
        let result = self.elements[self.current_index].clone();

        // Move to next position
        self.increment_index();

        Some(result)
    }
}

impl<T: Clone> DoubleEndedIterator for LoopIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.elements.is_empty() {
            return None;
        }

        // If we've looped back to start after having started, we're done
        if self.has_started && self.current_index == self.start_index {
            return None;
        }

        self.has_started = true;

        self.decrement_index();

        // Return current element
        Some(self.elements[self.current_index].clone())
    }
}

#[cfg(test)]
#[path = "loop_iter_tests.rs"]
mod loop_iter_tests;
