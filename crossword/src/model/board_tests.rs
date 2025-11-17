#[cfg(test)]
mod tests {
    use crate::model::{
        board::{Board, Tile::*},
        common::*,
    };

    fn create_test_board() -> Board {
        let grid = vec![
            vec![Empty, Empty, Blocked],
            vec![Empty, Empty, Empty],
            vec![Blocked, Empty, Empty],
        ];
        Board {
            size: 3,
            grid: Grid::from_vec(grid),
            pos: Coordinate::new(0, 0, 3),
            direction: BoardDirection::Across,
        }
    }

    #[test]
    fn test_enter_char() {
        let board = create_test_board();
        let new_board = board.enter_char('A');
        assert_eq!(*new_board.grid.get(new_board.pos), Filled('A'));
    }

    #[test]
    fn test_delete_char() {
        let board = create_test_board();
        let board_with_char = board.enter_char('A');
        let cleared_board = board_with_char.delete_char();
        assert_eq!(*cleared_board.grid.get(cleared_board.pos), Empty);
    }

    #[test]
    fn test_swap_direction() {
        let board = create_test_board();
        assert_eq!(board.direction, BoardDirection::Across);

        let swapped = board.swap_direction();
        assert_eq!(swapped.direction, BoardDirection::Down);

        let swapped_back = swapped.swap_direction();
        assert_eq!(swapped_back.direction, BoardDirection::Across);
    }

    #[test]
    fn test_move_cursor_right() {
        let board = create_test_board();
        let moved = board.move_cursor(MoveDirection::Right);
        assert_eq!(moved.pos.col, 1);
        assert_eq!(moved.pos.row, 0);
    }

    #[test]
    fn test_move_cursor_skips_blocked() {
        let grid = vec![
            vec![Empty, Empty, Blocked],
            vec![Empty, Empty, Empty],
            vec![Blocked, Empty, Empty],
        ];
        // Start at position (1,0), moving right should skip blocked cell at (2,0)
        // and wrap to (0,1), then skip blocked at (0,2) wrapping to (1,2)
        let board = Board {
            size: 3,
            grid: Grid::from_vec(grid),
            pos: Coordinate::new(1, 0, 3),
            direction: BoardDirection::Across,
        };
        let moved = board.move_cursor(MoveDirection::Right);
        // From (1,0) -> (2,0) is blocked -> (0,1) is empty
        assert_eq!(moved.pos.col, 0);
        assert_eq!(moved.pos.row, 1);
    }

    #[test]
    fn test_move_backward_across() {
        let board = create_test_board();
        let board = Board {
            pos: Coordinate::new(1, 1, 3),
            direction: BoardDirection::Across,
            ..board
        };
        let moved = board.move_backward();
        assert_eq!(moved.pos.col, 0);
        assert_eq!(moved.pos.row, 1);
    }

    #[test]
    fn test_move_backward_down() {
        let board = create_test_board();
        let board = Board {
            pos: Coordinate::new(1, 1, 3),
            direction: BoardDirection::Down,
            ..board
        };
        let moved = board.move_backward();
        assert_eq!(moved.pos.col, 1);
        assert_eq!(moved.pos.row, 0);
    }

    #[test]
    fn test_enter_char_immutability() {
        let board = create_test_board();
        let original_pos = board.pos;
        let _new_board = board.clone().enter_char('A');
        // Original board should be unchanged
        assert_eq!(*board.grid.get(original_pos), Empty);
    }

    #[test]
    fn test_multiple_operations() {
        let board = create_test_board();
        let result = board
            .enter_char('A')
            .move_cursor(MoveDirection::Right)
            .enter_char('B')
            .swap_direction();

        assert_eq!(result.direction, BoardDirection::Down);
        assert_eq!(*result.grid.get(Coordinate::new(0, 0, 3)), Filled('A'));
        assert_eq!(*result.grid.get(Coordinate::new(1, 0, 3)), Filled('B'));
    }
}
