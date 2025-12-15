#[cfg(test)]
mod tests {
    use super::super::{BoardDirection, Coordinate, Word};

    #[test]
    fn test_new() {
        let word = Word::new(
            "Test".to_string(),
            1,
            BoardDirection::Across,
            Coordinate::new(0, 0),
            Coordinate::new(4, 0),
            "Test clue".to_string(),
        );

        assert_eq!(word.text, "Test");
        assert_eq!(word.clue_number, 1);
        assert_eq!(word.start_pos, Coordinate::new(0, 0));
        assert_eq!(word.end_pos, Coordinate::new(4, 0));
        assert_eq!(word.clue, "Test clue");
    }

    #[test]
    fn test_length_across() {
        let word = Word::new(
            "Test".to_string(),
            1,
            BoardDirection::Across,
            Coordinate::new(0, 0),
            Coordinate::new(4, 0),
            "Test".to_string(),
        );

        assert_eq!(word.length(), 5);
    }

    #[test]
    fn test_length_down() {
        let word = Word::new(
            "Test".to_string(),
            1,
            BoardDirection::Down,
            Coordinate::new(2, 1),
            Coordinate::new(2, 5),
            "Test".to_string(),
        );

        assert_eq!(word.length(), 5);
    }

    #[test]
    fn test_length_single_cell() {
        let word = Word::new(
            "I".to_string(),
            1,
            BoardDirection::Across,
            Coordinate::new(3, 3),
            Coordinate::new(3, 3),
            "I".to_string(),
        );

        assert_eq!(word.length(), 1);
    }

    #[test]
    fn test_is_across() {
        let word = Word::new(
            "Across".to_string(),
            1,
            BoardDirection::Across,
            Coordinate::new(0, 2),
            Coordinate::new(4, 2),
            "Across".to_string(),
        );

        assert!(word.is_across());
        assert!(!word.is_down());
    }

    #[test]
    fn test_is_down() {
        let word = Word::new(
            "Down".to_string(),
            1,
            BoardDirection::Down,
            Coordinate::new(3, 0),
            Coordinate::new(3, 4),
            "Down".to_string(),
        );

        assert!(word.is_down());
        assert!(!word.is_across());
    }

    #[test]
    fn test_contains_across() {
        let word = Word::new(
            "Test".to_string(),
            1,
            BoardDirection::Across,
            Coordinate::new(1, 2),
            Coordinate::new(5, 2),
            "Test".to_string(),
        );

        assert!(word.contains(&Coordinate::new(1, 2))); // Start
        assert!(word.contains(&Coordinate::new(3, 2))); // Middle
        assert!(word.contains(&Coordinate::new(5, 2))); // End
        assert!(!word.contains(&Coordinate::new(0, 2))); // Before
        assert!(!word.contains(&Coordinate::new(6, 2))); // After
        assert!(!word.contains(&Coordinate::new(3, 1))); // Wrong row
    }

    #[test]
    fn test_contains_down() {
        let word = Word::new(
            "Test".to_string(),
            1,
            BoardDirection::Down,
            Coordinate::new(2, 1),
            Coordinate::new(2, 5),
            "Test".to_string(),
        );

        assert!(word.contains(&Coordinate::new(2, 1))); // Start
        assert!(word.contains(&Coordinate::new(2, 3))); // Middle
        assert!(word.contains(&Coordinate::new(2, 5))); // End
        assert!(!word.contains(&Coordinate::new(2, 0))); // Before
        assert!(!word.contains(&Coordinate::new(2, 6))); // After
        assert!(!word.contains(&Coordinate::new(3, 3))); // Wrong col
    }

    #[test]
    fn test_display() {
        let word = Word::new(
            "Mona Lisa".to_string(),
            5,
            BoardDirection::Across,
            Coordinate::new(2, 3),
            Coordinate::new(6, 3),
            "Famous painting".to_string(),
        );

        let display = format!("{}", word);
        assert!(display.contains("5."));
        assert!(display.contains("Famous painting"));
    }

    #[test]
    fn test_cell_iter_across() {
        let word = Word::new(
            "CAT".to_string(),
            1,
            BoardDirection::Across,
            Coordinate::new(1, 2),
            Coordinate::new(3, 2),
            "Feline".to_string(),
        );

        let cells: Vec<Coordinate> = word.cell_iter().collect();
        assert_eq!(cells.len(), 3);
        assert_eq!(cells[0], Coordinate::new(1, 2));
        assert_eq!(cells[1], Coordinate::new(2, 2));
        assert_eq!(cells[2], Coordinate::new(3, 2));
    }

    #[test]
    fn test_cell_iter_down() {
        let word = Word::new(
            "DOG".to_string(),
            2,
            BoardDirection::Down,
            Coordinate::new(3, 1),
            Coordinate::new(3, 3),
            "Canine".to_string(),
        );

        let cells: Vec<Coordinate> = word.cell_iter().collect();
        assert_eq!(cells.len(), 3);
        assert_eq!(cells[0], Coordinate::new(3, 1));
        assert_eq!(cells[1], Coordinate::new(3, 2));
        assert_eq!(cells[2], Coordinate::new(3, 3));
    }

    #[test]
    fn test_word_iter_from_start() {
        let word = Word::new(
            "TEST".to_string(),
            1,
            BoardDirection::Across,
            Coordinate::new(0, 0),
            Coordinate::new(3, 0),
            "Test clue".to_string(),
        );

        let cells: Vec<Coordinate> = word
            .current_word_iter(Some(Coordinate::new(0, 0)))
            .collect();
        assert_eq!(cells.len(), 4);
        assert_eq!(cells[0], Coordinate::new(0, 0));
    }

    #[test]
    fn test_word_iter_from_middle() {
        let word = Word::new(
            "TEST".to_string(),
            1,
            BoardDirection::Across,
            Coordinate::new(0, 0),
            Coordinate::new(3, 0),
            "Test clue".to_string(),
        );

        let cells: Vec<Coordinate> = word
            .current_word_iter(Some(Coordinate::new(2, 0)))
            .collect();
        // Should start from position 2 and continue to end, then wrap to beginning
        assert!(cells.len() > 0);
        assert_eq!(cells[0], Coordinate::new(2, 0));
    }

    #[test]
    fn test_word_iter_none_start() {
        let word = Word::new(
            "TEST".to_string(),
            1,
            BoardDirection::Across,
            Coordinate::new(0, 0),
            Coordinate::new(3, 0),
            "Test clue".to_string(),
        );

        let cells: Vec<Coordinate> = word.current_word_iter(None).collect();
        // Should start from beginning
        assert_eq!(cells.len(), 4);
        assert_eq!(cells[0], Coordinate::new(0, 0));
    }
}
