#[cfg(test)]
mod tests {
    use super::super::{Coordinate, Grid, GridIterator, MoveDirection};

    #[test]
    fn test_new() {
        let grid = Grid::new(3, 0);
        let iter = GridIterator::new(&grid, Coordinate::new(0, 0), MoveDirection::Right);

        assert_eq!(iter.start_pos, Coordinate::new(0, 0));
        assert_eq!(iter.current_pos, Coordinate::new(0, 0));
    }

    #[test]
    fn test_iterate_right() {
        let grid = Grid::from_vec(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 3);

        let iter = GridIterator::new(&grid, Coordinate::new(0, 0), MoveDirection::Right);
        let results: Vec<(Coordinate, i32)> = iter.collect();

        // Starting at (0,0), going right: (1,0)=2, (2,0)=3, wraps to (0,1)=4, etc.
        let expected = vec![
            (Coordinate::new(1, 0), 2),
            (Coordinate::new(2, 0), 3),
            (Coordinate::new(0, 1), 4),
            (Coordinate::new(1, 1), 5),
            (Coordinate::new(2, 1), 6),
            (Coordinate::new(0, 2), 7),
            (Coordinate::new(1, 2), 8),
            (Coordinate::new(2, 2), 9),
        ];
        assert_eq!(results, expected);
    }

    #[test]
    fn test_iterate_down() {
        let grid = Grid::from_vec(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 3);

        let iter = GridIterator::new(&grid, Coordinate::new(0, 0), MoveDirection::Down);
        let values: Vec<i32> = iter.map(|(_, val)| val).collect();

        // Starting at (0,0), going down: (0,1)=4, (0,2)=7, wraps to (1,0)=2, etc.
        assert_eq!(values, vec![4, 7, 2, 5, 8, 3, 6, 9]);
    }

    #[test]
    fn test_iterate_left() {
        let grid = Grid::from_vec(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 3);

        let iter = GridIterator::new(&grid, Coordinate::new(2, 0), MoveDirection::Left);
        let values: Vec<i32> = iter.map(|(_, val)| val).collect();

        // Starting at (2,0)=3, going left: (1,0)=2, (0,0)=1
        assert_eq!(values, vec![2, 1]);
    }

    #[test]
    fn test_iterate_up() {
        let grid = Grid::from_vec(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 3);

        let iter = GridIterator::new(&grid, Coordinate::new(0, 2), MoveDirection::Up);
        let values: Vec<i32> = iter.map(|(_, val)| val).collect();

        // Starting at (0,2)=7, going up: (0,1)=4, (0,0)=1
        assert_eq!(values, vec![4, 1]);
    }

    #[test]
    fn test_iterate_from_middle() {
        let grid = Grid::from_vec(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 3);

        let iter = GridIterator::new(&grid, Coordinate::new(1, 1), MoveDirection::Right);
        let values: Vec<i32> = iter.map(|(_, val)| val).collect();

        // Starting at (1,1)=5, going right: (2,1)=6, wraps to (0,2)=7, (1,2)=8, (2,2)=9
        assert_eq!(values, vec![6, 7, 8, 9]);
    }

    #[test]
    fn test_iterate_single_cell_grid() {
        let grid = Grid::new(1, 42);

        let iter = GridIterator::new(&grid, Coordinate::new(0, 0), MoveDirection::Right);
        let values: Vec<i32> = iter.map(|(_, val)| val).collect();

        // Single cell, should immediately return to start
        assert_eq!(values, vec![]);
    }

    #[test]
    fn test_iterate_2x2_grid() {
        let grid = Grid::from_vec(vec![vec!['A', 'B'], vec!['C', 'D']], 2);

        let iter = GridIterator::new(&grid, Coordinate::new(0, 0), MoveDirection::Right);
        let values: Vec<char> = iter.map(|(_, val)| val).collect();

        // Starting at (0,0)='A', going right: 'B', 'C', 'D'
        assert_eq!(values, vec!['B', 'C', 'D']);
    }

    #[test]
    fn test_wrapping_behavior() {
        let grid = Grid::from_vec(vec![vec![1, 2], vec![3, 4]], 2);

        // Test that moving right from (1,0) wraps to (0,1)
        let iter = GridIterator::new(&grid, Coordinate::new(1, 0), MoveDirection::Right);
        let values: Vec<i32> = iter.map(|(_, val)| val).collect();

        // From (1,0)=2: wraps to (0,1)=3, (1,1)=4
        assert_eq!(values, vec![3, 4]);
    }

    #[test]
    #[should_panic(expected = "Start position is out of bounds")]
    fn test_panic_on_invalid_start() {
        let grid = Grid::new(3, 0);
        let mut iter = GridIterator::new(&grid, Coordinate::new(5, 5), MoveDirection::Right);

        // Should panic when calling next() with invalid start position
        iter.next();
    }

    #[test]
    fn test_returns_coordinate_and_value() {
        let grid = Grid::from_vec(vec![vec![1, 2], vec![3, 4]], 2);

        let mut iter = GridIterator::new(&grid, Coordinate::new(0, 0), MoveDirection::Right);

        // Verify we get (coordinate, value) tuples
        if let Some((coord, val)) = iter.next() {
            assert_eq!(coord, Coordinate::new(1, 0));
            assert_eq!(val, 2);
        }
    }
}
