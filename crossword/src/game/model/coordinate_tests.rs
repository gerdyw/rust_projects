#[cfg(test)]
mod tests {
    use super::super::Coordinate;

    #[test]
    fn test_new() {
        let coord = Coordinate::new(3, 5);
        assert_eq!(coord.col, 3);
        assert_eq!(coord.row, 5);
    }

    #[test]
    fn test_up() {
        let coord = Coordinate::new(2, 3);
        let up = coord.up();
        assert_eq!(up.col, 2);
        assert_eq!(up.row, 2);
    }

    #[test]
    fn test_down() {
        let coord = Coordinate::new(2, 3);
        let down = coord.down();
        assert_eq!(down.col, 2);
        assert_eq!(down.row, 4);
    }

    #[test]
    fn test_left() {
        let coord = Coordinate::new(2, 3);
        let left = coord.left();
        assert_eq!(left.col, 1);
        assert_eq!(left.row, 3);
    }

    #[test]
    fn test_right() {
        let coord = Coordinate::new(2, 3);
        let right = coord.right();
        assert_eq!(right.col, 3);
        assert_eq!(right.row, 3);
    }

    #[test]
    fn test_is_valid_inside_bounds() {
        let coord = Coordinate::new(2, 3);
        assert!(coord.is_valid(5, 5));
    }

    #[test]
    fn test_is_valid_at_edge() {
        let coord = Coordinate::new(0, 0);
        assert!(coord.is_valid(5, 5));

        let coord = Coordinate::new(4, 4);
        assert!(coord.is_valid(5, 5));
    }

    #[test]
    fn test_is_valid_out_of_bounds() {
        let coord = Coordinate::new(-1, 0);
        assert!(!coord.is_valid(5, 5));

        let coord = Coordinate::new(0, -1);
        assert!(!coord.is_valid(5, 5));

        let coord = Coordinate::new(5, 0);
        assert!(!coord.is_valid(5, 5));

        let coord = Coordinate::new(0, 5);
        assert!(!coord.is_valid(5, 5));
    }

    #[test]
    fn test_equality() {
        let coord1 = Coordinate::new(2, 3);
        let coord2 = Coordinate::new(2, 3);
        let coord3 = Coordinate::new(3, 2);

        assert_eq!(coord1, coord2);
        assert_ne!(coord1, coord3);
    }

    #[test]
    fn test_set_row() {
        let coord = Coordinate::new(2, 3);
        let new_coord = coord.set_row(5);

        assert_eq!(new_coord.col, 2);
        assert_eq!(new_coord.row, 5);
        assert_eq!(coord.row, 3); // Original unchanged
    }

    #[test]
    fn test_set_col() {
        let coord = Coordinate::new(2, 3);
        let new_coord = coord.set_col(7);

        assert_eq!(new_coord.col, 7);
        assert_eq!(new_coord.row, 3);
        assert_eq!(coord.col, 2); // Original unchanged
    }

    #[test]
    fn test_set_axis_across() {
        use super::super::BoardDirection;
        let coord = Coordinate::new(2, 3);
        let new_coord = coord.set_axis(BoardDirection::Across, 8);

        assert_eq!(new_coord.col, 8);
        assert_eq!(new_coord.row, 3);
    }

    #[test]
    fn test_set_axis_down() {
        use super::super::BoardDirection;
        let coord = Coordinate::new(2, 3);
        let new_coord = coord.set_axis(BoardDirection::Down, 6);

        assert_eq!(new_coord.col, 2);
        assert_eq!(new_coord.row, 6);
    }

    #[test]
    fn test_move_direction() {
        use super::super::MoveDirection;
        let coord = Coordinate::new(5, 5);

        assert_eq!(
            coord.move_direction(MoveDirection::Up),
            Coordinate::new(5, 4)
        );
        assert_eq!(
            coord.move_direction(MoveDirection::Down),
            Coordinate::new(5, 6)
        );
        assert_eq!(
            coord.move_direction(MoveDirection::Left),
            Coordinate::new(4, 5)
        );
        assert_eq!(
            coord.move_direction(MoveDirection::Right),
            Coordinate::new(6, 5)
        );
    }

    #[test]
    fn test_order_board_dir_across() {
        use super::super::BoardDirection;
        use std::cmp::Ordering;

        let coord1 = Coordinate::new(2, 1);
        let coord2 = Coordinate::new(3, 1);
        let coord3 = Coordinate::new(1, 2);

        // Same row, compare by col
        assert_eq!(
            coord1.order_board_dir(&coord2, BoardDirection::Across),
            Ordering::Less
        );
        assert_eq!(
            coord2.order_board_dir(&coord1, BoardDirection::Across),
            Ordering::Greater
        );

        // Different row, row takes precedence
        assert_eq!(
            coord1.order_board_dir(&coord3, BoardDirection::Across),
            Ordering::Less
        );
    }

    #[test]
    fn test_order_board_dir_down() {
        use super::super::BoardDirection;
        use std::cmp::Ordering;

        let coord1 = Coordinate::new(1, 2);
        let coord2 = Coordinate::new(1, 3);
        let coord3 = Coordinate::new(2, 1);

        // Same col, compare by row
        assert_eq!(
            coord1.order_board_dir(&coord2, BoardDirection::Down),
            Ordering::Less
        );
        assert_eq!(
            coord2.order_board_dir(&coord1, BoardDirection::Down),
            Ordering::Greater
        );

        // Different col, col takes precedence
        assert_eq!(
            coord1.order_board_dir(&coord3, BoardDirection::Down),
            Ordering::Less
        );
    }

    #[test]
    fn test_order_in_move_dir() {
        use super::super::MoveDirection;
        use std::cmp::Ordering;

        let coord1 = Coordinate::new(2, 1);
        let coord2 = Coordinate::new(3, 1);

        // Moving right (forward)
        assert_eq!(
            coord1.order_in_move_dir(&coord2, MoveDirection::Right),
            Ordering::Less
        );

        // Moving left (backward) reverses order
        assert_eq!(
            coord1.order_in_move_dir(&coord2, MoveDirection::Left),
            Ordering::Greater
        );
    }

    #[test]
    fn test_ord_trait() {
        let coord1 = Coordinate::new(1, 1);
        let coord2 = Coordinate::new(2, 1);
        let coord3 = Coordinate::new(1, 2);

        // Same row, ordered by col
        assert!(coord1 < coord2);
        assert!(coord2 > coord1);

        // Different row, ordered by row first
        assert!(coord1 < coord3);
        assert!(coord3 > coord1);
    }

    #[test]
    fn test_partial_ord_trait() {
        let coord1 = Coordinate::new(1, 1);
        let coord2 = Coordinate::new(1, 1);
        let coord3 = Coordinate::new(2, 1);

        assert_eq!(coord1.partial_cmp(&coord2), Some(std::cmp::Ordering::Equal));
        assert_eq!(coord1.partial_cmp(&coord3), Some(std::cmp::Ordering::Less));
        assert_eq!(
            coord3.partial_cmp(&coord1),
            Some(std::cmp::Ordering::Greater)
        );
    }
}
