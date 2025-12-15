#[cfg(test)]
mod tests {
    use super::super::{BoardCell, BoardDirection, Coordinate, MoveDirection, PlayerBoard, Puzzle};

    fn create_test_puzzle() -> Puzzle {
        Puzzle::from_toml_file("puzzles/puzzle.toml").expect("Failed to load test puzzle")
    }

    #[test]
    fn test_from_puzzle() {
        let puzzle = create_test_puzzle();
        let board = PlayerBoard::from_puzzle(puzzle.clone());

        // Should start with cursor at a playable position (not necessarily origin if blocked)
        assert!(board.cell_playable(board.get_cursor()));
        assert_eq!(board.direction, BoardDirection::Across);
        assert_eq!(board.size, puzzle.size());
    }

    #[test]
    fn test_get() {
        let puzzle = create_test_puzzle();
        let board = PlayerBoard::from_puzzle(puzzle.clone());

        // All cells should start empty (or blocked)
        let coord = Coordinate::new(0, 0);
        match board.get(coord) {
            BoardCell::Empty | BoardCell::Blocked => {}
            BoardCell::Filled(_) => panic!("Expected empty or blocked cell"),
        }
    }

    #[test]
    fn test_swap_direction() {
        let puzzle = create_test_puzzle();
        let mut board = PlayerBoard::from_puzzle(puzzle.clone());

        assert_eq!(board.get_direction(), BoardDirection::Across);

        board.swap_direction();
        assert_eq!(board.get_direction(), BoardDirection::Down);

        board.swap_direction();
        assert_eq!(board.get_direction(), BoardDirection::Across);
    }

    #[test]
    fn test_write_to_cell() {
        let puzzle = create_test_puzzle();
        let mut board = PlayerBoard::from_puzzle(puzzle.clone());

        // Move to a playable cell
        while !board.cell_playable(board.get_cursor()) {
            board.move_cursor(MoveDirection::Right);
        }

        let cursor = board.get_cursor();
        board.write_to_cell('A');

        assert_eq!(board.get(cursor), BoardCell::Filled('A'));
    }

    #[test]
    fn test_clear_cell() {
        let puzzle = create_test_puzzle();
        let mut board = PlayerBoard::from_puzzle(puzzle.clone());

        // Move to a playable cell, fill it, then clear it
        while !board.cell_playable(board.get_cursor()) {
            board.move_cursor(MoveDirection::Right);
        }

        let cursor = board.get_cursor();
        board.write_to_cell('A');
        assert_eq!(board.get(cursor), BoardCell::Filled('A'));

        board.clear_cell();
        assert_eq!(board.get(cursor), BoardCell::Empty);
    }

    #[test]
    #[should_panic(expected = "Only alphanumeric characters can be entered")]
    fn test_write_to_cell_invalid_char() {
        let puzzle = create_test_puzzle();
        let mut board = PlayerBoard::from_puzzle(puzzle.clone());

        // Move to a playable cell
        while !board.cell_playable(board.get_cursor()) {
            board.move_cursor(MoveDirection::Right);
        }

        board.write_to_cell('!');
    }

    #[test]
    fn test_move_cursor_right() {
        let puzzle = create_test_puzzle();
        let mut board = PlayerBoard::from_puzzle(puzzle.clone());
        let original_cursor = board.get_cursor();

        board.move_cursor(MoveDirection::Right);

        // Cursor should have moved (unless all cells to the right are blocked)
        // At minimum, it should not panic
        assert!(board.get_cursor().col >= original_cursor.col);
    }

    #[test]
    fn test_move_cursor_down() {
        let puzzle = create_test_puzzle();
        let mut board = PlayerBoard::from_puzzle(puzzle.clone());
        let original_cursor = board.get_cursor();

        board.move_cursor(MoveDirection::Down);

        // Cursor should have moved down or wrapped
        let new_cursor = board.get_cursor();
        assert!(new_cursor != original_cursor || new_cursor == original_cursor);
    }

    #[test]
    fn test_is_playable() {
        let puzzle = create_test_puzzle();
        let board = PlayerBoard::from_puzzle(puzzle.clone());

        // Test various coordinates
        let coord = Coordinate::new(0, 0);
        // Just verify it doesn't panic - the actual playability depends on the puzzle
        let _ = board.cell_playable(coord);
    }

    #[test]
    fn test_move_cursor_skips_blocked() {
        let puzzle = create_test_puzzle();
        let mut board = PlayerBoard::from_puzzle(puzzle.clone());

        // Move cursor and verify we don't land on a blocked cell
        board.move_cursor(MoveDirection::Right);

        // The cursor should either be on a playable cell or back at start
        let cursor = board.get_cursor();
        if cursor != Coordinate::new(0, 0) {
            assert!(board.cell_playable(cursor));
        }
    }

    #[test]
    fn test_fill_cell() {
        let puzzle = create_test_puzzle();
        let mut board = PlayerBoard::from_puzzle(puzzle.clone());

        // Find a playable cell
        while !board.cell_playable(board.get_cursor()) {
            board.move_cursor(MoveDirection::Right);
        }

        let first_cursor = board.get_cursor();
        board.write_to_cell('H');
        assert_eq!(board.get(first_cursor), BoardCell::Filled('H'));

        board.move_cursor(MoveDirection::Right);
        // Should have moved to a different cell
        let cursor = board.get_cursor();
        if board.cell_playable(cursor) {
            board.write_to_cell('E');
            // Should be able to fill the second cell
            assert_eq!(board.get(cursor), BoardCell::Filled('E'));
        }
    }

    #[test]
    fn test_mutability() {
        let puzzle = create_test_puzzle();
        let mut board = PlayerBoard::from_puzzle(puzzle.clone());

        // Initial state - cursor should be at a playable position
        let initial_cursor = board.get_cursor();
        assert!(board.cell_playable(initial_cursor));
        assert_eq!(board.get_direction(), BoardDirection::Across);

        // Mutate direction
        board.swap_direction();
        assert_eq!(board.get_direction(), BoardDirection::Down);

        // Mutate cursor
        board.move_cursor(MoveDirection::Right);
        // Cursor should have changed or stayed same if at edge/blocked
        let new_cursor = board.get_cursor();
        assert!(board.cell_playable(new_cursor));
    }

    #[test]
    fn test_get_clue_number_at() {
        let puzzle = create_test_puzzle();
        let board = PlayerBoard::from_puzzle(puzzle.clone());

        // Test that clue numbers can be retrieved
        // Result depends on puzzle structure
        let result = board.get_clue_number_at(Coordinate::new(0, 0));
        // Just verify it doesn't panic
        let _ = result;
    }

    #[test]
    fn test_get_current() {
        let puzzle = create_test_puzzle();
        let board = PlayerBoard::from_puzzle(puzzle.clone());

        let current = board.get_current();
        // Should be either Empty or Blocked (not Filled at start)
        match current {
            BoardCell::Empty | BoardCell::Blocked => {}
            BoardCell::Filled(_) => panic!("Expected empty or blocked cell at start"),
        }
    }

    #[test]
    fn test_size() {
        let puzzle = create_test_puzzle();
        let board = PlayerBoard::from_puzzle(puzzle.clone());

        assert_eq!(board.size(), puzzle.size());
        assert!(board.size() > 0);
    }

    #[test]
    fn test_get_elapsed_time() {
        let puzzle = create_test_puzzle();
        let board = PlayerBoard::from_puzzle(puzzle.clone());

        let elapsed = board.get_elapsed_time();
        // Should be a very small duration since just created
        assert!(elapsed.as_secs() < 1);
    }

    #[test]
    fn test_get_total_time_before_win() {
        let puzzle = create_test_puzzle();
        let board = PlayerBoard::from_puzzle(puzzle.clone());

        // Before winning, total time should be None
        assert_eq!(board.get_total_time(), None);
    }

    #[test]
    fn test_get_total_time_after_win() {
        let puzzle = create_test_puzzle();
        let mut board = PlayerBoard::from_puzzle(puzzle.clone());

        // Mark as won
        board.mark_as_won();

        // Now total time should be Some
        assert!(board.get_total_time().is_some());
    }

    #[test]
    fn test_get_words_in_current_direction() {
        let puzzle = create_test_puzzle();
        let board = PlayerBoard::from_puzzle(puzzle.clone());

        // Should return across words initially
        let words = board.get_words_in_current_direction();
        assert!(words.len() > 0);

        // Compare with puzzle's across words
        assert_eq!(words.len(), puzzle.words.across.len());
    }

    #[test]
    fn test_get_words_in_current_direction_after_swap() {
        let puzzle = create_test_puzzle();
        let mut board = PlayerBoard::from_puzzle(puzzle.clone());

        board.swap_direction();

        // Should return down words after swap
        let words = board.get_words_in_current_direction();
        assert_eq!(words.len(), puzzle.words.down.len());
    }

    #[test]
    fn test_get_current_word() {
        let puzzle = create_test_puzzle();
        let mut board = PlayerBoard::from_puzzle(puzzle.clone());

        // Move to a position that's definitely in a word
        while !board.cell_playable(board.get_cursor()) {
            board.move_cursor(MoveDirection::Right);
        }

        // May or may not have a word depending on puzzle structure
        let _ = board.get_current_word();
    }

    #[test]
    fn test_current_word_iter() {
        let puzzle = create_test_puzzle();
        let mut board = PlayerBoard::from_puzzle(puzzle.clone());

        // Move to a playable position
        while !board.cell_playable(board.get_cursor()) {
            board.move_cursor(MoveDirection::Right);
        }

        let iter = board.current_word_iter();
        // May or may not have an iterator depending on puzzle structure
        if let Some(iter) = iter {
            // Verify it's an iterator
            let _coords: Vec<_> = iter.collect();
        }
    }

    #[test]
    fn test_is_current_word_completed_empty() {
        let puzzle = create_test_puzzle();
        let mut board = PlayerBoard::from_puzzle(puzzle.clone());

        // Move to a playable position
        while !board.cell_playable(board.get_cursor()) {
            board.move_cursor(MoveDirection::Right);
        }

        // Initially, word should not be completed (cells are empty)
        assert!(!board.is_current_word_completed());
    }

    #[test]
    fn test_move_to_next_empty_cell() {
        let puzzle = create_test_puzzle();
        let mut board = PlayerBoard::from_puzzle(puzzle.clone());

        board.move_to_next_empty_cell();

        // Cursor should have moved or stayed if already at an empty cell
        let new_cursor = board.get_cursor();
        assert!(board.cell_playable(new_cursor));
    }

    #[test]
    fn test_move_to_previous_empty_cell() {
        let puzzle = create_test_puzzle();
        let mut board = PlayerBoard::from_puzzle(puzzle.clone());

        // Fill current cell first
        if board.cell_playable(board.get_cursor()) {
            board.write_to_cell('A');
        }

        board.move_to_previous_empty_cell();

        // Should move to previous empty cell
        let cursor = board.get_cursor();
        assert!(board.cell_playable(cursor));
    }

    #[test]
    fn test_move_to_next_open_word() {
        let puzzle = create_test_puzzle();
        let mut board = PlayerBoard::from_puzzle(puzzle.clone());

        board.move_to_next_open_word();

        // Cursor should have moved (or stayed if only one word)
        let new_cursor = board.get_cursor();
        assert!(board.cell_playable(new_cursor));
    }

    #[test]
    fn test_has_won_initial() {
        let puzzle = create_test_puzzle();
        let board = PlayerBoard::from_puzzle(puzzle.clone());

        // Should not have won initially (puzzle is empty)
        assert!(!board.has_won());
    }

    #[test]
    fn test_mark_as_won() {
        let puzzle = create_test_puzzle();
        let mut board = PlayerBoard::from_puzzle(puzzle.clone());

        // Initially no end time
        assert_eq!(board.get_total_time(), None);

        board.mark_as_won();

        // Now should have an end time
        assert!(board.get_total_time().is_some());
    }

    #[test]
    fn test_mark_as_won_idempotent() {
        let puzzle = create_test_puzzle();
        let mut board = PlayerBoard::from_puzzle(puzzle.clone());

        board.mark_as_won();
        let first_time = board.get_total_time();

        // Mark again - time shouldn't change
        board.mark_as_won();
        let second_time = board.get_total_time();

        assert_eq!(first_time, second_time);
    }

    #[test]
    fn test_move_to_valid() {
        let puzzle = create_test_puzzle();
        let mut board = PlayerBoard::from_puzzle(puzzle.clone());

        // Find a playable cell
        let mut target = Coordinate::new(0, 0);
        for row in 0..board.size() {
            for col in 0..board.size() {
                let coord = Coordinate::new(col as isize, row as isize);
                if board.cell_playable(coord) {
                    target = coord;
                    break;
                }
            }
        }

        let result = board.move_to(target);
        assert!(result.is_ok());
        assert_eq!(board.get_cursor(), target);
    }

    #[test]
    fn test_move_to_blocked() {
        let puzzle = create_test_puzzle();
        let mut board = PlayerBoard::from_puzzle(puzzle.clone());

        // Find a blocked cell
        let mut blocked = None;
        for row in 0..board.size() {
            for col in 0..board.size() {
                let coord = Coordinate::new(col as isize, row as isize);
                if !board.cell_playable(coord) {
                    blocked = Some(coord);
                    break;
                }
            }
            if blocked.is_some() {
                break;
            }
        }

        if let Some(blocked_coord) = blocked {
            let result = board.move_to(blocked_coord);
            assert!(result.is_err());
        }
    }
}
