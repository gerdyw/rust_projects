#[cfg(test)]
mod tests {
    use super::super::{BoardDirection, MoveDirection};

    #[test]
    fn test_default() {
        let dir = BoardDirection::default();
        assert_eq!(dir, BoardDirection::Across);
    }

    #[test]
    fn test_swap_across_to_down() {
        let dir = BoardDirection::Across;
        assert_eq!(dir.swap(), BoardDirection::Down);
    }

    #[test]
    fn test_swap_down_to_across() {
        let dir = BoardDirection::Down;
        assert_eq!(dir.swap(), BoardDirection::Across);
    }

    #[test]
    fn test_to_move_direction_across() {
        let dir = BoardDirection::Across;
        assert_eq!(dir.to_move_direction(), MoveDirection::Right);
    }

    #[test]
    fn test_to_move_direction_down() {
        let dir = BoardDirection::Down;
        assert_eq!(dir.to_move_direction(), MoveDirection::Down);
    }

    #[test]
    fn test_from_move_direction_right() {
        let board_dir = BoardDirection::from(MoveDirection::Right);
        assert_eq!(board_dir, BoardDirection::Across);
    }

    #[test]
    fn test_from_move_direction_left() {
        let board_dir = BoardDirection::from(MoveDirection::Left);
        assert_eq!(board_dir, BoardDirection::Across);
    }

    #[test]
    fn test_from_move_direction_down() {
        let board_dir = BoardDirection::from(MoveDirection::Down);
        assert_eq!(board_dir, BoardDirection::Down);
    }

    #[test]
    fn test_from_move_direction_up() {
        let board_dir = BoardDirection::from(MoveDirection::Up);
        assert_eq!(board_dir, BoardDirection::Down);
    }

    #[test]
    fn test_into_move_direction_across() {
        let move_dir: MoveDirection = BoardDirection::Across.into();
        assert_eq!(move_dir, MoveDirection::Right);
    }

    #[test]
    fn test_into_move_direction_down() {
        let move_dir: MoveDirection = BoardDirection::Down.into();
        assert_eq!(move_dir, MoveDirection::Down);
    }

    #[test]
    fn test_display_across() {
        let dir = BoardDirection::Across;
        assert_eq!(format!("{}", dir), "Across");
    }

    #[test]
    fn test_display_down() {
        let dir = BoardDirection::Down;
        assert_eq!(format!("{}", dir), "Down");
    }

    #[test]
    fn test_equality() {
        assert_eq!(BoardDirection::Across, BoardDirection::Across);
        assert_eq!(BoardDirection::Down, BoardDirection::Down);
        assert_ne!(BoardDirection::Across, BoardDirection::Down);
    }

    #[test]
    fn test_clone() {
        let dir = BoardDirection::Across;
        let cloned = dir.clone();
        assert_eq!(dir, cloned);
    }
}
