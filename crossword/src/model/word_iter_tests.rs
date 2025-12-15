#[cfg(test)]
mod tests {
    use super::super::{BoardDirection, Coordinate, Word, WordIter};

    fn create_test_word_across() -> Word {
        Word::new(
            "TEST".to_string(),
            1,
            BoardDirection::Across,
            Coordinate::new(0, 0),
            Coordinate::new(3, 0),
            "Test word".to_string(),
            vec![],
        )
    }

    fn create_test_word_down() -> Word {
        Word::new(
            "DOWN".to_string(),
            2,
            BoardDirection::Down,
            Coordinate::new(2, 1),
            Coordinate::new(2, 4),
            "Down word".to_string(),
            vec![],
        )
    }

    #[test]
    fn test_new() {
        let word = create_test_word_across();
        let iter = WordIter::new(&word);

        // Verify iterator is created with correct starting position
        let coords: Vec<Coordinate> = iter.collect();
        assert_eq!(coords.len(), 4);
        assert_eq!(coords[0], Coordinate::new(0, 0));
    }

    #[test]
    fn test_iterate_across_word() {
        let word = create_test_word_across();
        let iter = WordIter::new(&word);

        let coords: Vec<Coordinate> = iter.collect();
        assert_eq!(coords.len(), 4);
        assert_eq!(coords[0], Coordinate::new(0, 0));
        assert_eq!(coords[1], Coordinate::new(1, 0));
        assert_eq!(coords[2], Coordinate::new(2, 0));
        assert_eq!(coords[3], Coordinate::new(3, 0));
    }

    #[test]
    fn test_iterate_down_word() {
        let word = create_test_word_down();
        let iter = WordIter::new(&word);

        let coords: Vec<Coordinate> = iter.collect();
        assert_eq!(coords.len(), 4);
        assert_eq!(coords[0], Coordinate::new(2, 1));
        assert_eq!(coords[1], Coordinate::new(2, 2));
        assert_eq!(coords[2], Coordinate::new(2, 3));
        assert_eq!(coords[3], Coordinate::new(2, 4));
    }

    #[test]
    fn test_start_at() {
        let word = create_test_word_across();
        let mut iter = WordIter::new(&word);

        // Start from middle of word
        iter.start_at(Coordinate::new(2, 0));

        let coords: Vec<Coordinate> = iter.collect();
        // Should start at (2,0), go to (3,0), wrap to (0,0), (1,0), then stop
        assert_eq!(coords.len(), 4);
        assert_eq!(coords[0], Coordinate::new(2, 0));
        assert_eq!(coords[1], Coordinate::new(3, 0));
        assert_eq!(coords[2], Coordinate::new(0, 0));
        assert_eq!(coords[3], Coordinate::new(1, 0));
    }

    #[test]
    fn test_from_start() {
        let word = create_test_word_across();
        let mut iter = WordIter::new(&word);

        // Move to different position first
        iter.start_at(Coordinate::new(2, 0));
        // Then reset current to word start (but start_pos is still (2,0))
        iter.from_start();

        let coords: Vec<Coordinate> = iter.collect();
        // Will go from (0,0) until it hits start_pos at (2,0)
        assert_eq!(coords.len(), 2);
        assert_eq!(coords[0], Coordinate::new(0, 0));
        assert_eq!(coords[1], Coordinate::new(1, 0));
    }

    #[test]
    fn test_single_char_word() {
        let word = Word::new(
            "I".to_string(),
            1,
            BoardDirection::Across,
            Coordinate::new(5, 5),
            Coordinate::new(5, 5),
            "Single".to_string(),
            vec![],
        );

        let iter = WordIter::new(&word);
        let coords: Vec<Coordinate> = iter.collect();

        assert_eq!(coords.len(), 1);
        assert_eq!(coords[0], Coordinate::new(5, 5));
    }

    #[test]
    fn test_wrapping_behavior() {
        let word = create_test_word_across();
        let mut iter = WordIter::new(&word);

        // Start from last position
        iter.start_at(Coordinate::new(3, 0));

        let coords: Vec<Coordinate> = iter.collect();
        // Should go: (3,0) -> wraps to (0,0) -> (1,0) -> (2,0) -> back to (3,0) stop
        assert_eq!(coords.len(), 4);
        assert_eq!(coords[0], Coordinate::new(3, 0));
        assert_eq!(coords[1], Coordinate::new(0, 0));
    }

    #[test]
    fn test_equality() {
        let word = create_test_word_across();
        let iter1 = WordIter::new(&word);
        let iter2 = WordIter::new(&word);

        assert_eq!(iter1, iter2);
    }

    #[test]
    fn test_clone() {
        let word = create_test_word_across();
        let iter = WordIter::new(&word);
        let cloned = iter.clone();

        assert_eq!(iter, cloned);
    }
}
