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
}
