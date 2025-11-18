#[cfg(test)]
mod tests {
    use super::super::{Coordinate, Grid, Puzzle};

    fn create_simple_puzzle() -> Puzzle {
        let grid_vec = vec![
            vec![Some('C'), Some('A'), Some('T')],
            vec![Some('A'), None, Some('O')],
            vec![Some('R'), Some('A'), Some('T')],
        ];
        let grid = Grid::from_vec(grid_vec, 3);
        
        let across_clues = vec![
            "Feline".to_string(),
            "Rodent".to_string(),
        ];
        let down_clues = vec![
            "Automobile".to_string(),
            "Past tense of eat".to_string(),
            "Rodent".to_string(),
        ];
        
        let (across_words, down_words) = Puzzle::find_words(&grid, across_clues, down_clues);
        
        Puzzle {
            grid,
            across_words,
            down_words,
        }
    }

    #[test]
    fn test_new() {
        let puzzle = Puzzle::new(5);
        assert_eq!(puzzle.size(), 5);
        assert_eq!(puzzle.across_words.len(), 0);
        assert_eq!(puzzle.down_words.len(), 0);
    }

    #[test]
    fn test_size() {
        let puzzle = create_simple_puzzle();
        assert_eq!(puzzle.size(), 3);
    }

    #[test]
    fn test_get() {
        let puzzle = create_simple_puzzle();
        
        assert_eq!(puzzle.get(Coordinate::new(0, 0)), Some('C'));
        assert_eq!(puzzle.get(Coordinate::new(1, 0)), Some('A'));
        assert_eq!(puzzle.get(Coordinate::new(2, 0)), Some('T'));
        assert_eq!(puzzle.get(Coordinate::new(1, 1)), None); // Blocked
    }

    #[test]
    fn test_get_out_of_bounds() {
        let puzzle = create_simple_puzzle();
        
        assert_eq!(puzzle.get(Coordinate::new(-1, 0)), None);
        assert_eq!(puzzle.get(Coordinate::new(3, 0)), None);
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
        assert_eq!(puzzle.across_words.len(), 2);
        
        let cat_word = &puzzle.across_words[0];
        assert_eq!(cat_word.start_pos, Coordinate::new(0, 0));
        assert_eq!(cat_word.end_pos, Coordinate::new(2, 0));
        assert_eq!(cat_word.clue, "Feline");
        
        let rat_word = &puzzle.across_words[1];
        assert_eq!(rat_word.start_pos, Coordinate::new(0, 2));
        assert_eq!(rat_word.end_pos, Coordinate::new(2, 2));
        assert_eq!(rat_word.clue, "Rodent");
    }

    #[test]
    fn test_find_words_down() {
        let puzzle = create_simple_puzzle();
        
        // Should find CAR and AT (not TOT since middle is blocked)
        assert_eq!(puzzle.down_words.len(), 2);
        
        let car_word = &puzzle.down_words[0];
        assert_eq!(car_word.start_pos, Coordinate::new(0, 0));
        assert_eq!(car_word.end_pos, Coordinate::new(0, 2));
        assert_eq!(car_word.clue, "Automobile");
    }

    #[test]
    fn test_find_words_clue_numbers() {
        let puzzle = create_simple_puzzle();
        
        // Words starting at same position should share clue number
        let cat_across = &puzzle.across_words[0];
        let car_down = &puzzle.down_words[0];
        assert_eq!(cat_across.clue_number, car_down.clue_number);
        
        // Next words should have different numbers
        assert_ne!(cat_across.clue_number, puzzle.across_words[1].clue_number);
    }

    #[test]
    fn test_from_file() {
        // This would test loading from an actual file
        // For now, just test that it doesn't panic with valid structure
        let result = Puzzle::from_file("puzzles/puzzle.toml");
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
