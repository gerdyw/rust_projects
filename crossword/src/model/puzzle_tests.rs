#[cfg(test)]
mod tests {
    use crate::{
        execution::puzzle_parser::from_toml_file,
        model::{BoardDirection, PuzzleCell::*, Word},
    };

    use super::super::{Coordinate, Grid, Puzzle, PuzzleWords};

    fn create_simple_puzzle() -> Puzzle {
        let grid_vec = vec![
            vec![Fillable('C'), Fillable('A'), Fillable('T')],
            vec![Fillable('A'), Blocked, Fillable('O')],
            vec![Fillable('R'), Fillable('A'), Fillable('T')],
        ];
        let grid = Grid::from_vec(grid_vec, 3, 3);

        let across_clues = vec!["Feline".to_string(), "Rodent".to_string()];
        let down_clues = vec!["Automobile".to_string(), "Child".to_string()];
        let clue_numbers = vec![
            (Coordinate::new(0, 0), 1),
            (Coordinate::new(2, 0), 2),
            (Coordinate::new(0, 2), 3),
        ];

        let across = vec![
            Word::new(
                "CAT".to_string(),
                1,
                BoardDirection::Across,
                Coordinate::new(0, 0),
                Coordinate::new(2, 0),
                across_clues[0].clone(),
                vec![],
            ),
            Word::new(
                "RAT".to_string(),
                2,
                BoardDirection::Across,
                Coordinate::new(0, 2),
                Coordinate::new(2, 2),
                across_clues[1].clone(),
                vec![],
            ),
        ];

        let down = vec![
            Word::new(
                "CAR".to_string(),
                1,
                BoardDirection::Down,
                Coordinate::new(0, 0),
                Coordinate::new(0, 2),
                down_clues[0].clone(),
                vec![],
            ),
            Word::new(
                "TOT".to_string(),
                2,
                BoardDirection::Down,
                Coordinate::new(2, 0),
                Coordinate::new(2, 2),
                down_clues[1].clone(),
                vec![],
            ),
        ];

        Puzzle::new(grid, PuzzleWords { across, down }, clue_numbers)
    }

    #[test]
    fn test_size() {
        let puzzle = create_simple_puzzle();
        assert_eq!(puzzle.size(), 3);
    }

    #[test]
    fn test_get() {
        let puzzle = create_simple_puzzle();

        assert_eq!(puzzle.get(Coordinate::new(0, 0)), Fillable('C'));
        assert_eq!(puzzle.get(Coordinate::new(1, 0)), Fillable('A'));
        assert_eq!(puzzle.get(Coordinate::new(2, 0)), Fillable('T'));
        assert_eq!(puzzle.get(Coordinate::new(1, 1)), Blocked); // Blocked
    }

    #[test]
    fn test_get_out_of_bounds() {
        let puzzle = create_simple_puzzle();

        assert_eq!(puzzle.get(Coordinate::new(-1, 0)), Blocked);
        assert_eq!(puzzle.get(Coordinate::new(3, 0)), Blocked);
    }

    #[test]
    fn test_is_blocked() {
        let puzzle = create_simple_puzzle();

        assert!(!puzzle.is_blocked(Coordinate::new(0, 0)));
        assert!(puzzle.is_blocked(Coordinate::new(1, 1)));
        assert!(!puzzle.is_blocked(Coordinate::new(2, 2)));
    }

    #[test]
    fn test_is_fillable() {
        let puzzle = create_simple_puzzle();

        assert!(puzzle.is_fillable(Coordinate::new(0, 0)));
        assert!(!puzzle.is_fillable(Coordinate::new(1, 1)));
        assert!(puzzle.is_fillable(Coordinate::new(2, 2)));
    }

    #[test]
    fn test_find_words_across() {
        let puzzle = create_simple_puzzle();

        // Should find CAT and RAT
        assert_eq!(puzzle.words.across.len(), 2);

        let cat_word = &puzzle.words.across[0];
        assert_eq!(cat_word.start_pos, Coordinate::new(0, 0));
        assert_eq!(cat_word.end_pos, Coordinate::new(2, 0));
        assert_eq!(cat_word.clue, "Feline");

        let rat_word = &puzzle.words.across[1];
        assert_eq!(rat_word.start_pos, Coordinate::new(0, 2));
        assert_eq!(rat_word.end_pos, Coordinate::new(2, 2));
        assert_eq!(rat_word.clue, "Rodent");
    }

    #[test]
    fn test_find_words_down() {
        let puzzle = create_simple_puzzle();

        // Should find CAR and AT (not TOT since middle is blocked)
        assert_eq!(puzzle.words.down.len(), 2);

        let car_word = &puzzle.words.down[0];
        assert_eq!(car_word.start_pos, Coordinate::new(0, 0));
        assert_eq!(car_word.end_pos, Coordinate::new(0, 2));
        assert_eq!(car_word.clue, "Automobile");
    }

    #[test]
    fn test_find_words_clue_numbers() {
        let puzzle = create_simple_puzzle();

        // Words starting at same position should share clue number
        let cat_across = &puzzle.words.across[0];
        let car_down = &puzzle.words.down[0];
        assert_eq!(cat_across.clue_number, car_down.clue_number);

        // Next words should have different numbers
        assert_ne!(cat_across.clue_number, puzzle.words.across[1].clue_number);
    }

    #[test]
    fn test_from_file() {
        // This would test loading from an actual file
        // For now, just test that it doesn't panic with valid structure
        let result = from_toml_file("puzzles/puzzle.toml");
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_includes_grid() {
        let puzzle = create_simple_puzzle();
        let display = format!("{}", puzzle);

        // Should contain box drawing characters
        assert!(display.contains("┌"));
        assert!(display.contains("┐"));
        assert!(display.contains("└"));
        assert!(display.contains("┘"));
    }

    #[test]
    fn test_display_includes_clues() {
        let puzzle = create_simple_puzzle();
        let display = format!("{}", puzzle);

        assert!(display.contains("Across:"));
        assert!(display.contains("Down:"));
        assert!(display.contains("Feline"));
        assert!(display.contains("Automobile"));
    }
}
