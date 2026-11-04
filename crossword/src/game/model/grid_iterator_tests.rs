#[cfg(test)]
mod tests {
    use super::super::{Coordinate, Grid, GridIterator, MoveDirection};

    #[test]
    fn test_new() {
        let grid = Grid::new(3, 3, 0);
        let iter = GridIterator::new(&grid, Coordinate::new(0, 0), MoveDirection::Right, true);

        assert_eq!(iter.start_pos, Coordinate::new(0, 0));
        assert_eq!(iter.current_pos, Coordinate::new(0, 0));
    }

    #[test]
    fn test_iterate_right() {
        let grid = Grid::from_vec(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 3, 3);

        let iter = GridIterator::new(&grid, Coordinate::new(0, 0), MoveDirection::Right, false);
        let results: Vec<(Coordinate, i32)> = iter.collect();

        // Starting at (0,0), going right: (0,0)=1, (1,0)=2, (2,0)=3
        let expected = vec![
            (Coordinate::new(0, 0), 1),
            (Coordinate::new(1, 0), 2),
            (Coordinate::new(2, 0), 3),
        ];
        assert_eq!(results, expected);
    }

    #[test]
    fn test_iterate_down() {
        let grid = Grid::from_vec(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 3, 3);

        let iter = GridIterator::new(&grid, Coordinate::new(0, 0), MoveDirection::Down, false);
        let values: Vec<i32> = iter.map(|(_, val)| val).collect();

        // Starting at (0,0), going down: (0,0)=1, (0,1)=4, (0,2)=7
        assert_eq!(values, vec![1, 4, 7]);
    }

    #[test]
    fn test_iterate_left() {
        let grid = Grid::from_vec(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 3, 3);

        let iter = GridIterator::new(&grid, Coordinate::new(2, 0), MoveDirection::Left, false);
        let values: Vec<i32> = iter.map(|(_, val)| val).collect();

        // Starting at (2,0)=3, going left: (2,0)=3, (1,0)=2, (0,0)=1
        assert_eq!(values, vec![3, 2, 1]);
    }

    #[test]
    fn test_iterate_up() {
        let grid = Grid::from_vec(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 3, 3);

        let iter = GridIterator::new(&grid, Coordinate::new(0, 2), MoveDirection::Up, false);
        let values: Vec<i32> = iter.map(|(_, val)| val).collect();

        // Starting at (0,2)=7, going up: (0,2)=7, (0,1)=4, (0,0)=1
        assert_eq!(values, vec![7, 4, 1]);
    }

    #[test]
    fn test_iterate_from_middle() {
        let grid = Grid::from_vec(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 3, 3);

        let iter = GridIterator::new(&grid, Coordinate::new(1, 1), MoveDirection::Right, true);
        let values: Vec<i32> = iter.map(|(_, val)| val).collect();

        // Starting at (1,1)=5, going right with looping: includes start, wraps around
        assert_eq!(values, vec![5, 6, 4]);
    }

    #[test]
    fn test_iterate_single_cell_grid() {
        let grid = Grid::new(1, 1, 42);

        let iter = GridIterator::new(&grid, Coordinate::new(0, 0), MoveDirection::Right, false);
        let values: Vec<i32> = iter.map(|(_, val)| val).collect();

        // Single cell, should return just that cell
        assert_eq!(values, vec![42]);
    }

    #[test]
    fn test_iterate_2x2_grid() {
        let grid = Grid::from_vec(vec![vec!['A', 'B'], vec!['C', 'D']], 2, 2);

        let iter = GridIterator::new(&grid, Coordinate::new(0, 0), MoveDirection::Right, true);
        let values: Vec<char> = iter.map(|(_, val)| val).collect();

        // Starting at (0,0)='A', going right with looping: 'A', 'B', wraps to next row but stops before returning to 'A'
        assert_eq!(values, vec!['A', 'B']);
    }

    #[test]
    fn test_wrapping_behavior() {
        let grid = Grid::from_vec(vec![vec![1, 2], vec![3, 4]], 2, 2);

        // Test that moving right from (1,0) wraps to (0,1) with looping
        let iter = GridIterator::new(&grid, Coordinate::new(1, 0), MoveDirection::Right, true);
        let values: Vec<i32> = iter.map(|(_, val)| val).collect();

        // From (1,0)=2: returns 2, then wraps to (0,1)=3, then stops before returning to start
        assert_eq!(values, vec![2, 1]);
    }

    #[test]
    #[should_panic(expected = "Start position is out of bounds")]
    fn test_panic_on_invalid_start() {
        let grid = Grid::new(3, 3, 0);
        let mut iter = GridIterator::new(&grid, Coordinate::new(5, 5), MoveDirection::Right, true);

        // Should panic when calling next() with invalid start position
        iter.next();
    }

    #[test]
    fn test_returns_coordinate_and_value() {
        let grid = Grid::from_vec(vec![vec![1, 2], vec![3, 4]], 2, 2);

        let mut iter = GridIterator::new(&grid, Coordinate::new(0, 0), MoveDirection::Right, false);

        // Verify we get (coordinate, value) tuples - first element should be start position
        if let Some((coord, val)) = iter.next() {
            assert_eq!(coord, Coordinate::new(0, 0));
            assert_eq!(val, 1);
        }
    }
}
