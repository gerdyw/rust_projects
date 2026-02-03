#[cfg(test)]
mod tests {
    use super::super::{BoardDirection, MoveDirection};

    #[test]
    fn test_reverse_left() {
        assert_eq!(MoveDirection::Left.reverse(), MoveDirection::Right);
    }

    #[test]
    fn test_reverse_right() {
        assert_eq!(MoveDirection::Right.reverse(), MoveDirection::Left);
    }

    #[test]
    fn test_reverse_up() {
        assert_eq!(MoveDirection::Up.reverse(), MoveDirection::Down);
    }

    #[test]
    fn test_reverse_down() {
        assert_eq!(MoveDirection::Down.reverse(), MoveDirection::Up);
    }

    #[test]
    fn test_orthogonal_left() {
        assert_eq!(MoveDirection::Left.orthogonal(), MoveDirection::Up);
    }

    #[test]
    fn test_orthogonal_right() {
        assert_eq!(MoveDirection::Right.orthogonal(), MoveDirection::Down);
    }

    #[test]
    fn test_orthogonal_up() {
        assert_eq!(MoveDirection::Up.orthogonal(), MoveDirection::Left);
    }

    #[test]
    fn test_orthogonal_down() {
        assert_eq!(MoveDirection::Down.orthogonal(), MoveDirection::Right);
    }

    #[test]
    fn test_into_board_direction_left() {
        assert_eq!(
            MoveDirection::Left.into_board_direction(),
            BoardDirection::Across
        );
    }

    #[test]
    fn test_into_board_direction_right() {
        assert_eq!(
            MoveDirection::Right.into_board_direction(),
            BoardDirection::Across
        );
    }

    #[test]
    fn test_into_board_direction_up() {
        assert_eq!(
            MoveDirection::Up.into_board_direction(),
            BoardDirection::Down
        );
    }

    #[test]
    fn test_into_board_direction_down() {
        assert_eq!(
            MoveDirection::Down.into_board_direction(),
            BoardDirection::Down
        );
    }

    #[test]
    fn test_is_backwards_left() {
        assert!(MoveDirection::Left.is_backwards());
    }

    #[test]
    fn test_is_backwards_up() {
        assert!(MoveDirection::Up.is_backwards());
    }

    #[test]
    fn test_is_backwards_right() {
        assert!(!MoveDirection::Right.is_backwards());
    }

    #[test]
    fn test_is_backwards_down() {
        assert!(!MoveDirection::Down.is_backwards());
    }

    #[test]
    fn test_equality() {
        assert_eq!(MoveDirection::Left, MoveDirection::Left);
        assert_eq!(MoveDirection::Right, MoveDirection::Right);
        assert_eq!(MoveDirection::Up, MoveDirection::Up);
        assert_eq!(MoveDirection::Down, MoveDirection::Down);
        assert_ne!(MoveDirection::Left, MoveDirection::Right);
    }

    #[test]
    fn test_clone() {
        let dir = MoveDirection::Right;
        let cloned = dir.clone();
        assert_eq!(dir, cloned);
    }
}
