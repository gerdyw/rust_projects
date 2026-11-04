#[cfg(test)]
mod tests {
    use super::super::LoopIter;

    #[test]
    fn test_new() {
        let elements = vec![1, 2, 3, 4];
        let iter = LoopIter::new(elements.clone(), 0);
        let result: Vec<i32> = iter.collect();

        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_start_from_middle() {
        let elements = vec![1, 2, 3, 4];
        let iter = LoopIter::new(elements, 2);
        let result: Vec<i32> = iter.collect();

        // Start at index 2, loop around: 3, 4, 1, 2
        assert_eq!(result, vec![3, 4, 1, 2]);
    }

    #[test]
    fn test_start_from_end() {
        let elements = vec![1, 2, 3];
        let iter = LoopIter::new(elements, 2);
        let result: Vec<i32> = iter.collect();

        // Start at index 2 (last element), loop around: 3, 1, 2
        assert_eq!(result, vec![3, 1, 2]);
    }

    #[test]
    fn test_single_element() {
        let elements = vec!['A'];
        let iter = LoopIter::new(elements, 0);
        let result: Vec<char> = iter.collect();

        assert_eq!(result, vec!['A']);
    }

    #[test]
    fn test_empty_elements() {
        let elements: Vec<i32> = vec![];
        let iter = LoopIter::new(elements, 0);
        let result: Vec<i32> = iter.collect();

        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_start_at() {
        let elements = vec![10, 20, 30, 40];
        let mut iter = LoopIter::new(elements, 0);

        // Change start position to index 2
        iter.start_at(2);
        let result: Vec<i32> = iter.collect();

        assert_eq!(result, vec![30, 40, 10, 20]);
    }

    #[test]
    fn test_move_start_by_positive() {
        let elements = vec![1, 2, 3, 4, 5];
        let mut iter = LoopIter::new(elements, 0);

        // Move start by +2 positions
        iter.move_start_by(2);
        let result: Vec<i32> = iter.collect();

        assert_eq!(result, vec![3, 4, 5, 1, 2]);
    }

    #[test]
    fn test_move_start_by_negative() {
        let elements = vec![1, 2, 3, 4, 5];
        let mut iter = LoopIter::new(elements, 2);

        // Move start by -1 position
        iter.move_start_by(-1);
        let result: Vec<i32> = iter.collect();

        assert_eq!(result, vec![2, 3, 4, 5, 1]);
    }

    #[test]
    fn test_move_start_by_wraparound() {
        let elements = vec![1, 2, 3];
        let mut iter = LoopIter::new(elements, 0);

        // Move start by +5 (should wrap around)
        iter.move_start_by(5);
        let result: Vec<i32> = iter.collect();

        // 5 % 3 = 2, so should start at index 2
        assert_eq!(result, vec![3, 1, 2]);
    }

    #[test]
    fn test_double_ended_iterator_next_back() {
        let elements = vec![1, 2, 3, 4];
        let mut iter = LoopIter::new(elements, 0);

        // Get last element before start
        assert_eq!(iter.next_back(), Some(4));
        assert_eq!(iter.next_back(), Some(3));
    }

    #[test]
    fn test_double_ended_iterator_alternating() {
        let elements = vec![1, 2, 3, 4, 5];
        let mut iter = LoopIter::new(elements, 0);

        // next() returns current then increments
        assert_eq!(iter.next(), Some(1));
        // next_back() decrements then returns, but the current position was already moved by next()
        // Since current is now at 1, next_back decrements to 0, returns 1
        assert_eq!(iter.next_back(), Some(1));
    }

    #[test]
    fn test_clone() {
        let elements = vec![1, 2, 3];
        let iter = LoopIter::new(elements, 1);
        let cloned = iter.clone();

        let result1: Vec<i32> = iter.collect();
        let result2: Vec<i32> = cloned.collect();

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_strings() {
        let elements = vec!["first".to_string(), "second".to_string(), "third".to_string()];
        let iter = LoopIter::new(elements, 1);
        let result: Vec<String> = iter.collect();

        assert_eq!(result, vec!["second", "third", "first"]);
    }
}
