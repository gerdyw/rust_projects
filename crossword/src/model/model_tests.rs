#[cfg(test)]
mod tests {
    use crate::model::{
        board::{Board, Tile},
        common::*,
        message::Message,
        Model, RunningState,
    };

    fn create_simple_board() -> Board {
        let grid = vec![
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
        ];
        Board {
            size: 3,
            grid: Grid::from_vec(grid),
            pos: Coordinate::new(0, 0, 3),
            direction: BoardDirection::Across,
        }
    }

    #[test]
    fn test_handle_swap_direction() {
        let board = create_simple_board();
        let model = Model {
            puzzle: crate::model::puzzle::Puzzle {
                size: 3,
                grid: vec![vec![Some('A'); 3]; 3],
                across_clues: vec![],
                down_clues: vec![],
            },
            player_board: board,
            running_state: RunningState::Running,
        };
        
        let original_direction = model.player_board.direction;
        let (new_model, msg) = model.handle_swap_direction();
        assert!(msg.is_none());
        assert_ne!(new_model.player_board.direction, original_direction);
    }

    #[test]
    fn test_update_quit_message() {
        let board = create_simple_board();
        let model = Model {
            puzzle: crate::model::puzzle::Puzzle {
                size: 3,
                grid: vec![vec![Some('A'); 3]; 3],
                across_clues: vec![],
                down_clues: vec![],
            },
            player_board: board,
            running_state: RunningState::Running,
        };
        
        let (new_model, msg) = model.update(Message::Quit);
        assert_eq!(new_model.running_state, RunningState::Quit);
        assert!(msg.is_none());
    }
}
