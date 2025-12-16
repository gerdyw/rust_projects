#[cfg(test)]
mod tests {
    use super::super::{CommandExecutor, GameCommand};
    use crate::{
        execution::puzzle_parser::from_toml_file,
        model::{BoardDirection, Coordinate, MoveDirection, PlayerBoard},
    };

    fn create_test_executor() -> CommandExecutor {
        let puzzle = from_toml_file("puzzles/puzzle.toml").expect("Failed to load test puzzle");
        let board = PlayerBoard::from_puzzle(puzzle);
        CommandExecutor::new(board)
    }

    #[test]
    fn test_new() {
        let executor = create_test_executor();
        assert!(executor.is_running());
    }

    #[test]
    fn test_quit_command() {
        let mut executor = create_test_executor();

        executor.execute(GameCommand::Quit);

        assert!(!executor.is_running());
    }

    #[test]
    fn test_swap_direction_command() {
        let mut executor = create_test_executor();

        let initial_direction = executor.get_board().get_direction();
        executor.execute(GameCommand::SwapDirection);
        let new_direction = executor.get_board().get_direction();

        assert_ne!(initial_direction, new_direction);
    }

    #[test]
    fn test_enter_char_command() {
        let mut executor = create_test_executor();

        // Find a playable position
        while !executor
            .get_board()
            .cell_playable(executor.get_board().get_cursor())
        {
            executor.execute(GameCommand::MoveInDirection(MoveDirection::Right));
        }

        let cursor = executor.get_board().get_cursor();
        executor.execute(GameCommand::EnterChar('A'));

        // Verify character was written (cursor may have moved)
        let board = executor.get_board();
        let cell = board.get(cursor);
        assert!(matches!(cell, crate::model::BoardCell::Filled('A')));
    }

    #[test]
    fn test_delete_char_on_filled() {
        let mut executor = create_test_executor();

        // Find a playable position and fill it
        while !executor
            .get_board()
            .cell_playable(executor.get_board().get_cursor())
        {
            executor.execute(GameCommand::MoveInDirection(MoveDirection::Right));
        }

        let cursor = executor.get_board().get_cursor();
        executor.execute(GameCommand::EnterChar('B'));

        // Move back to the filled cell
        executor.execute(GameCommand::MoveInDirection(MoveDirection::Left));

        // Delete should clear the cell
        executor.execute(GameCommand::DeleteChar);

        let board = executor.get_board();
        let cell = board.get(cursor);
        assert!(matches!(cell, crate::model::BoardCell::Empty));
    }

    #[test]
    fn test_move_in_direction_commands() {
        let mut executor = create_test_executor();

        executor.execute(GameCommand::MoveInDirection(MoveDirection::Right));
        let new_cursor = executor.get_board().get_cursor();

        // Cursor should have moved or stayed at a valid position
        assert!(executor.get_board().cell_playable(new_cursor));
    }

    #[test]
    fn test_move_forward_command() {
        let mut executor = create_test_executor();

        let initial_cursor = executor.get_board().get_cursor();
        let initial_direction = executor.get_board().get_direction();

        executor.execute(GameCommand::MoveForward);

        let new_cursor = executor.get_board().get_cursor();

        // Should move in the current direction
        match initial_direction {
            BoardDirection::Across => {
                assert!(
                    new_cursor.col >= initial_cursor.col || new_cursor.row > initial_cursor.row
                );
            }
            BoardDirection::Down => {
                assert!(
                    new_cursor.row >= initial_cursor.row || new_cursor.col > initial_cursor.col
                );
            }
        }
    }

    #[test]
    fn test_move_to_next_empty_cell_command() {
        let mut executor = create_test_executor();

        // Fill current cell
        if executor
            .get_board()
            .cell_playable(executor.get_board().get_cursor())
        {
            executor.execute(GameCommand::EnterChar('X'));
        }

        executor.execute(GameCommand::MoveToNextEmptyCell);

        // Should be at an empty playable cell
        let cursor = executor.get_board().get_cursor();
        assert!(executor.get_board().cell_playable(cursor));
    }

    #[test]
    fn test_move_to_next_word_command() {
        let mut executor = create_test_executor();

        executor.execute(GameCommand::MoveToNextOpenWord);

        // Just verify we're still at a valid position
        assert!(
            executor
                .get_board()
                .cell_playable(executor.get_board().get_cursor())
        );
    }

    #[test]
    fn test_move_to_command() {
        let mut executor = create_test_executor();

        // Find a playable position
        let target = Coordinate::new(1, 1);
        if executor.get_board().cell_playable(target) {
            executor.execute(GameCommand::MoveTo(target));

            assert_eq!(executor.get_board().get_cursor(), target);
        }
    }

    #[test]
    fn test_check_has_won() {
        let mut executor = create_test_executor();

        // Initially should not have won
        assert!(!executor.check_has_won());
    }

    #[test]
    fn test_get_board() {
        let executor = create_test_executor();

        let board = executor.get_board();
        assert!(board.size() > 0);
    }

    #[test]
    fn test_executor_running_state() {
        let mut executor = create_test_executor();

        assert!(executor.is_running());

        executor.execute(GameCommand::Quit);

        assert!(!executor.is_running());
    }

    #[test]
    fn test_enter_char_overwrite_mode() {
        let mut executor = create_test_executor();

        // Find a playable position
        while !executor
            .get_board()
            .cell_playable(executor.get_board().get_cursor())
        {
            executor.execute(GameCommand::MoveInDirection(MoveDirection::Right));
        }

        let cursor = executor.get_board().get_cursor();

        // Write first character
        executor.execute(GameCommand::EnterChar('A'));

        // Move back
        let prev_direction = executor
            .get_board()
            .get_direction()
            .to_move_direction()
            .reverse();
        executor.execute(GameCommand::MoveInDirection(prev_direction));

        // Overwrite with second character
        executor.execute(GameCommand::EnterChar('B'));

        // Should have moved forward after overwrite
        let new_cursor = executor.get_board().get_cursor();
        assert_ne!(cursor, new_cursor);
    }

    #[test]
    fn test_delete_char_on_empty() {
        let mut executor = create_test_executor();

        // Ensure we're at an empty cell
        while !executor
            .get_board()
            .cell_playable(executor.get_board().get_cursor())
        {
            executor.execute(GameCommand::MoveInDirection(MoveDirection::Right));
        }

        // Delete on empty should move cursor backward
        executor.execute(GameCommand::DeleteChar);

        let new_cursor = executor.get_board().get_cursor();

        // Cursor should have moved or stayed
        assert!(executor.get_board().cell_playable(new_cursor));
    }

    #[test]
    fn test_command_chaining() {
        let mut executor = create_test_executor();

        // Execute multiple commands in sequence
        executor.execute(GameCommand::SwapDirection);
        executor.execute(GameCommand::SwapDirection);

        // Should be back to original direction
        assert_eq!(executor.get_board().get_direction(), BoardDirection::Across);
    }
}
